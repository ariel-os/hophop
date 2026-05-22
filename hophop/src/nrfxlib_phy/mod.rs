// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! High-level wrappers around the DECT PHY.

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use nrf_modem::{Error, ErrorSource, nrfxlib_sys};

mod error;
use error::{MixedError, PhyResult, PhyResultExt as _};

mod latency;

mod rssi;
mod rx;

use ts_103_636_utils::identifiers::{AbsoluteChannel, NetworkId32};

pub const TICKS_PER_SECOND: u64 = 69_120_000;
pub const TICKS_PER_MILLISECOND: u64 = 69_120;

// FIXME: What's a good length? Probably events can pile up, like "here's the last data and by the
// way the transaction is now complete". And do we need the CS mutex?
//
// … or we just replace all of them with signalling into explicit expecters, leaving nothing that
// gets exclusively processed by a single task.
static DECT_EVENTS: embassy_sync::channel::Channel<CriticalSectionRawMutex, DectEventOuter, 32> =
    embassy_sync::channel::Channel::new();

/// Newtype around the 32-bit `handle` mechanism by which events can be correlated to who sent
/// them, especially when multiple operations are enqueued.
///
/// This type encodes this library's convetion about how they are used:
/// - Unmanaged values generally result in the event being passed on to `DECT_EVENTS`.
/// - Managed values cause an action directly in the ISR.
#[derive(Debug, defmt::Format, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Handle(pub u32);

// FIXME here and in DectEvent: I'd much rather just copy the few bytes around rather than
// repacking and copying; but that's optimization, and right now I want to get things to run.
//
// The whole API is internal anyway.
#[derive(Debug)]
struct DectEventOuter {
    time: u64,
    event: DectEvent,
}

#[derive(Debug)]
enum DectEvent {
    // Not relaying any fields we don't use yet; in particular, an init error would be instant
    // panic.
    Init,
    Activate,
    Configure,
    TimeGet,
    LatencyGet,
    Completed(PhyResult),
    /// This is both the `EVT_PCC_ERROR` that really is just CRC error, or failures during processing
    /// of a PCC.
    PccError(rx::PccError),
    Pcc(rx::Pcc),
    PdcError,
    /// Owned PDC, or None when out of pool space.
    Pdc(Option<heapless::pool::boxed::Box<rx::RecvPool>>),
    Rssi(Handle, Option<heapless::pool::boxed::Box<rssi::RssiPool>>),
}

// FIXME: This is only pub while the DectPhy object doesn't have an init that calls the low-level
// init.
extern "C" fn dect_event(arg: *const nrfxlib_sys::nrf_modem_dect_phy_event) {
    let arg: &nrfxlib_sys::nrf_modem_dect_phy_event = unsafe { &*arg };

    defmt::trace!("Handler called: id {}, time {}", arg.id, arg.time);
    let event = match arg.id {
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_INIT => {
            // SAFETY: Checked the discriminator
            let init = unsafe { &arg.__bindgen_anon_1.init };
            defmt::trace!(
                "Init event: err {:#x} ({}), temp {}°C, voltage {}mV, temperature_limit {}°C",
                // FIXME: Best guess is that they internally use packed enums and we don't
                init.err,
                match init.err {
                    nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_SUCCESS => "success",
                    nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_NOT_ALLOWED =>
                        "not allowed",
                    nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_TEMP_HIGH =>
                        "temp high",
                    nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_PROD_LOCK =>
                        "prod lock",
                    _ => "unknown",
                },
                init.temp,
                init.voltage,
                init.temperature_limit
            );
            assert_eq!(
                init.err,
                nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_SUCCESS
            );
            // We could optimize here and not even emit this as an init event but start with a
            // configured activation sequence right away. Not doing this yet because having things
            // run in something more async-like is easier to read (and thus to maintain), we don't
            // have any benchmarks that indicate otherwise, and if we manage to run on bare IPC,
            // that'll be easier to integrate.
            DectEvent::Init
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_CONFIGURE => {
            // SAFETY: Checked the discriminator
            let activate = unsafe { &arg.__bindgen_anon_1.activate };
            assert_eq!(
                activate.err,
                nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_SUCCESS
            );
            DectEvent::Configure
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_ACTIVATE => {
            // SAFETY: Checked the discriminator
            let activate = unsafe { &arg.__bindgen_anon_1.activate };
            assert_eq!(
                activate.err,
                nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_SUCCESS
            );
            DectEvent::Activate
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_RSSI => {
            // SAFETY: Checked the discriminator, and function
            unsafe { rssi::event(&raw const arg.__bindgen_anon_1.rssi) }
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_COMPLETED => {
            // SAFETY: Checked the discriminator
            let op = unsafe { &arg.__bindgen_anon_1.op_complete };
            defmt::trace!(
                "Op completed: handle {} err {} temp {} voltage {}",
                op.handle,
                op.err,
                op.temp,
                op.voltage
            );
            // Go into different queue?
            DectEvent::Completed(op.err.into_phy_result())
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_TIME => {
            // SAFETY: Checked the discriminator
            let time_get = unsafe { &arg.__bindgen_anon_1.time_get };
            assert_eq!(
                time_get.err,
                nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_SUCCESS,
                "Never saw this fail"
            );
            DectEvent::TimeGet
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_PCC => {
            // SAFETY: Checked the discriminator, and function
            unsafe { rx::event_pcc(&raw const arg.__bindgen_anon_1.pcc) }
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_PCC_ERROR => {
            DectEvent::PccError(rx::PccError::CrcError)
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_PDC => {
            // SAFETY: Checked the discriminator, and function
            unsafe { rx::event_pdc(&raw const arg.__bindgen_anon_1.pdc) }
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_PDC_ERROR => {
            DectEvent::PdcError
        }
        nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_LATENCY => {
            // SAFETY: Checked the discriminator, and function
            unsafe { latency::event(&raw const arg.__bindgen_anon_1.latency_get) }
        }
        _ => {
            defmt::warn!("Event had no known handler");
            return;
        }
    };
    DECT_EVENTS
        .try_send(DectEventOuter {
            event,
            time: arg.time,
        })
        .expect("Queue is managed");
}

pub struct DectPhy(());

impl DectPhy {
    /// Starts the NRF Modem library with a manually specified memory layout
    ///
    /// The `_modem_is_set_up` argument is a stand-in for that there *should* be some assurance
    /// value returned from the OS that the modem was indeed set up (ideally: with some
    /// parameters); the `()` tuple is a stand-in that will evolve as Ariel OS's `take_modem()`
    /// will evolve.
    pub async fn init_after_modem_init(_modem_is_set_up: ()) -> Result<Self, Error> {
        defmt::trace!("Setting up own memory");
        // FIXME: Can we leave it to the user to allocate this?
        rssi::init();
        rx::init();

        defmt::trace!("Setting DECT handler");

        // Note that unlike typical C callbacks, this callback setup takes no argument -- if it did, we
        // might consider abstracting here, by passing in the original function and accepting a
        // single-call indicrection instead of the extern "C" on the handler.

        unsafe { nrfxlib_sys::nrf_modem_dect_phy_event_handler_set(Some(dect_event)) }
            .into_result()?;

        defmt::trace!("Initializing DECT PHY");

        unsafe { nrfxlib_sys::nrf_modem_dect_phy_init() }.into_result()?;

        defmt::trace!("Initialization started.");

        let DectEventOuter {
            event: DectEvent::Init,
            ..
        } = DECT_EVENTS.receive().await
        else {
            panic!("Sequence violation: Event before Init event");
        };

        // We have to call this before setting a modem mode: After, it will return
        // NRF_MODEM_DECT_PHY_ERR_NOT_ALLOWED.
        unsafe { nrfxlib_sys::nrf_modem_dect_phy_latency_get() }.into_result()?;
        let DectEventOuter {
            event: DectEvent::LatencyGet,
            ..
        } = DECT_EVENTS.receive().await
        else {
            panic!("Sequence violation");
        };

        // FIXME take parameters
        let params = nrfxlib_sys::nrf_modem_dect_phy_config_params {
            band_group_index: 0,
            harq_rx_process_count: 4,
            harq_rx_expiry_time_us: 1000000,
        };
        unsafe { nrfxlib_sys::nrf_modem_dect_phy_configure(&raw const params) }.into_result()?;
        let DectEventOuter {
            event: DectEvent::Configure,
            ..
        } = DECT_EVENTS.receive().await
        else {
            panic!("Sequence violation");
        };

        // FIXME power hog? delay to runtime?
        let mode =
            nrfxlib_sys::nrf_modem_dect_phy_radio_mode_NRF_MODEM_DECT_PHY_RADIO_MODE_LOW_LATENCY;
        unsafe { nrfxlib_sys::nrf_modem_dect_phy_activate(mode) }.into_result()?;
        let DectEventOuter {
            event: DectEvent::Activate,
            ..
        } = DECT_EVENTS.receive().await
        else {
            panic!("Sequence violation");
        };

        Ok(Self(()))
    }

    pub async fn time_get(&mut self) -> Result<u64, Error> {
        unsafe { nrfxlib_sys::nrf_modem_dect_phy_time_get() }.into_result()?;

        let DectEventOuter {
            event: DectEvent::TimeGet,
            time,
        } = DECT_EVENTS.receive().await
        else {
            panic!("Sequence violation");
        };

        Ok(time)
    }

    /// Transmit a message at the indicated time, or immediately if `start_time` is 0.
    ///
    /// The `network_id` influences scrambling. Pass in the full 32-bit network ID; this function
    /// picks it apart depending on the PCC length. Beware that this is required to be non-zero.
    pub async fn tx(
        &mut self,
        start_time: u64,
        channel: AbsoluteChannel,
        network_id: NetworkId32,
        pcc: &[u8],
        pdc: &[u8],
    ) -> Result<(), MixedError> {
        let phy_type = match pcc.len() {
            5 => 0,
            10 => 1,
            _ => panic!("Not a valid header length"),
        };

        unsafe {
            // FIXME: everything
            nrfxlib_sys::nrf_modem_dect_phy_tx(&nrfxlib_sys::nrf_modem_dect_phy_tx_params {
                start_time,
                handle: 2468,
                // FIXME: Verify that libmodem or the network core does the >> 8 / & 0xff.
                //
                // (Probably: otherwise, the "must not be zero" can not be upheld).
                network_id: network_id.into(),
                phy_type,
                lbt_rssi_threshold_max: 0, // see below
                carrier: channel.into(),
                lbt_period: 0, // BIG FIXME
                // The object may be smaller than expected for phy_header, but then, phy_type tells
                // to only access the smaller struct fields anyway.
                phy_header: pcc.as_ptr() as _,
                bs_cqi: nrfxlib_sys::NRF_MODEM_DECT_PHY_BS_CQI_NOT_USED as _,
                // Missing `const` in C? They won't really write in there, will they?
                data: pdc.as_ptr().cast_mut(),
                data_size: pdc.len() as _,
            })
        }
        .into_result()
        .map_err(MixedError::General)?;

        match DECT_EVENTS.receive().await {
            DectEventOuter {
                event: DectEvent::Completed(e),
                ..
            } => e.map_err(MixedError::Phy),
            _ => panic!("Sequence violation"),
        }
    }
}
