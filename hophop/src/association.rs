// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Tools for working with network associations.
//!
//! Code in here should eventually be independent of the underlying implementation of nr+, but is
//! currently tied to the [`nrfxlib_mac`].

// FIXME: which log level is suitable? probably debug, but defmt not having per-module granularity
// doesn't help.
use defmt::info;

use ts_103_636_utils::identifiers::{AbsoluteChannel, LongRdId, NetworkId24, NetworkId32};

use super::nrfxlib_mac;

/// Key material for a network in Mode 1 (AES-128)
pub struct Mode1Keys<'a> {
    pub integrity_key: &'a [u8; 16],
    pub cipher_key: &'a [u8; 16],
}

impl defmt::Format for Mode1Keys<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "Mode1Keys {{ [cryptographic keys not shown] }}")
    }
}

/// Description of a network that could be joined.
#[derive(defmt::Format)]
pub struct CandidateNetwork<'a> {
    /// List of channels that should be scanned for beacons.
    ///
    /// FIXME: Allow channels of multiple bands, and/or more channels than can be scanned in one
    /// go.
    pub channels: &'a [AbsoluteChannel],
    /// Network ID by which the network would be recognized.
    ///
    /// This can be left blank to accept any network that matches any other criteria.
    pub network_id: Option<NetworkId24>,
    /// Network keys.
    ///
    /// If set, only a network encrypted with those keys will be joined.
    pub keys: Option<Mode1Keys<'a>>,
    /// Beacon interval of the network.
    ///
    /// Channels are listened to for long enough to catch a beacon if one is sent. Setting this too
    /// high means that scanning takes longer, setting this too low means that a scan might return
    /// unsuccessful and send the application into a power saving loop until it so happens that a
    /// beacon arrives within the configured interval.
    ///
    /// FIXME: Should there be a retry multiplier? (Probably not: when not even beacons are
    /// received, we're just too far out).
    ///
    /// FIXME: The implementation could chunk up receive attempts so that the ill-effect of
    /// configuring this too high would just that scanning is done longer if there are no beacons
    /// received, but we'd still hop through the channels in relatively short intervals, taking
    /// care to cover all possible times where something could have been sent eventually.
    ///
    /// FIXME: Use proper milliseconds type (but with the current back-end, using ms is just too
    /// easy)
    pub beacon_interval_ms: u16,
}

#[derive(defmt::Format)]
pub struct PtConfiguration<'a> {
    /// Hint to various choices of power-vs-latency trade-offs
    pub power_save: bool,
    /// Radio ID to pick.
    ///
    /// FIXME: Should be more random/automated, but which part thereof?
    ///
    /// FIXME: Should this be per network?
    pub rd_id: LongRdId,
    pub networks: &'a [CandidateNetwork<'a>],
}

#[derive(defmt::Format, Debug, Copy, Clone, minicbor::Encode)]
#[cbor(map)]
pub struct Association {
    #[n(0)]
    pub network: NetworkId32,
    #[n(1)]
    pub parent: LongRdId,
    #[n(2)]
    pub channel: AbsoluteChannel,
}

/// Run a one-off association based on a list of candidate networks.
///
/// # API considerations
///
/// This assumes that association happens once, and then there is a continuous stream of data
/// exchanged until a disruptive disassociation (upon which association is started anew).
///
/// This will not be the case indefinitely; events that might touch both on association details and
/// the flowing data are:
/// - network rekeying
/// - some mobility scenarios (especially change of sink address)
/// - maybe changes to the LSB of the network ID
///
/// This API will change at some point to accommodate them.
///
/// # Expected time
///
/// This scans all channels relevant for all given networks, and due to [the underlying tools'
/// limitations](https://devzone.nordicsemi.com/f/nordic-q-a/128223/dect-mac-security-key-by-network),
/// encrypted networks need to scan independently even if they are on the same channel.
///
/// In a first approximation for the average case, when one network is present, association will
/// take half of that network's beacon period times the sum of the candidate networks' numbers of
/// channels.
///
/// # Return
///
/// This function returns if an association has been made.
///
/// FIXME: Which information is useful? What can we give the caller to find its *relevant* pieces
/// of which network was picked? (Just the `&'c CandidateNetwork<'c>` won't be useful; an index?
/// The network ID will often not tell much either.)
///
/// If no network is found, the function returns empty.
pub async fn associate<'c>(
    dect: &mut nrfxlib_mac::DectMac,
    config: &PtConfiguration<'c>,
) -> Option<(
    Association,
    impl core::future::Future<Output = nrfxlib_mac::AssociationEndEvent> + use<>,
)> {
    for network_template in config.networks {
        info!("Scanning for network {}", network_template);

        // FIXME: Can we require the Dect to be in deactivated mode, and then should we (or do we even
        // need to?) skip the first deactivate?
        dect.control_functional_mode_set_deactivate().await;

        let security = if let Some(Mode1Keys {
            integrity_key,
            cipher_key,
        }) = network_template.keys
        {
            nrfxlib_sys::nrf_modem_dect_control_configure_params__bindgen_ty_1 {
                // We have to decode the beacon's ciphered parts immediately rather than
                // doing that later when having read the network ID; see documentation on
                // "Expected time"
                mode:
                    nrfxlib_sys::nrf_modem_dect_mac_security_mode_NRF_MODEM_DECT_MAC_SECURITY_MODE_1,
                integrity_key: *integrity_key,
                cipher_key: *cipher_key,
            }
        } else {
            nrfxlib_sys::nrf_modem_dect_control_configure_params__bindgen_ty_1 {
                mode:
                    nrfxlib_sys::nrf_modem_dect_mac_security_mode_NRF_MODEM_DECT_MAC_SECURITY_MODE_NONE,
                integrity_key: [0; _],
                cipher_key: [0; _],
            }
        };

        dect.control_configure(&mut nrfxlib_sys::nrf_modem_dect_control_configure_params {
            // The maximum; we'll leave it to the network to lower the limits.
            // FIXME: take from hardware (although here it's the the standard's limit, but maybe we
            // have to adjust to a lower one from the hardware?)
            // FIXME why does this fail when we put in 23_DB?
            max_tx_power: nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_10_DB,
            // FIXME: take from hardware
            max_mcs: nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_4,
            // FIXME: Decide (this is what the vendor examples default to)
            expected_mcs1_rx_rssi_level: -68,
            // FIXME: could we decide that later? (probably not: we come in with som expectation of
            // the network already)
            long_rd_id: config.rd_id.into(),
            // FIXME: configure
            phy_band_group_index:
                nrfxlib_sys::nrf_modem_dect_mac_band_group_index_NRF_MODEM_DECT_MAC_PHY_BAND_GROUP_IDX0,
            power_save: config.power_save,
            security,
            // FIXME: Decide (this is what the vendor uses in their examples)
            stats_averaging_length: 2,
        })
        .await;

        dect.control_functional_mode_set_activate().await;

        let find_our_network = async |r: nrfxlib_mac::ScanReceiver| {
            // If we did the optimization of "we have multiple networks we can join, so we scan a
            // common channel and then pick one", we'd have to do something in here, but we
            // currently don't do that because it doesn't work with encryption due to
            // <https://devzone.nordicsemi.com/f/nordic-q-a/128223/dect-mac-security-key-by-network>, and we don't expect a lot of unencrypted cases, so we just pre-configure all filters anyway, and can then just return here.
            r.next().await
        };

        let mut channel_list = [0; _];
        let num_channels = network_template.channels.len();
        assert!(
            num_channels < channel_list.len(),
            "Multi-band scanning not yet supported"
        );
        for (place, template) in channel_list
            .iter_mut()
            .zip(network_template.channels.iter())
        {
            *place = u16::from(*template);
        }

        let (network_id_filter_mode, network_id_filter) = match network_template.network_id {
            // not sure why we'd ever filter for 32bit, those can be different next time around,
            // can they not?
            Some(n) => (nrfxlib_sys::nrf_modem_dect_mac_nw_id_filter_mode_NRF_MODEM_DECT_MAC_NW_ID_FILTER_MODE_24MSB, n.into_low_u32()),
            None => (nrfxlib_sys::nrf_modem_dect_mac_nw_id_filter_mode_NRF_MODEM_DECT_MAC_NW_ID_FILTER_MODE_NONE, 0),
        };

        let Some(params) = dect
            .mac_network_scan(
                &mut nrfxlib_sys::nrf_modem_dect_mac_network_scan_params {
                    // FIXME: we could optimize if we accepted "full band" rather than an explicit channel
                    // list
                    band: 0, // would then be nrfxlib_sys::nrf_modem_dect_mac_band_NRF_MODEM_DECT_MAC_PHY_BAND1,
                    num_channels: num_channels
                        .try_into()
                        .expect("fitting a much smaller array was just checked"),
                    channel_list,
                    // ms -- 60s is the maximum. I guess this is per band?
                    // +1: We have to account for just-on-the-brink cases, but beacons take less than 1ms,
                    // and the 10ppm over the maximum 60s don't create another 1ms
                    scan_time: network_template.beacon_interval_ms + 1,
                    network_id_filter_mode,
                    network_id_filter,
                },
                find_our_network,
            )
            .await
        else {
            continue;
        };

        info!(
            "Scanning succeeded, attempting association with the found FT {}",
            params
        );
        match dect
            .mac_association(params.transmitter_long_rd_id, params.network_id)
            .await
        {
            Ok(release) => {
                return Some((
                    Association {
                        network: params.network_id,
                        parent: params.transmitter_long_rd_id,
                        channel: params.channel,
                    },
                    release,
                ));
            }
            Err(e) => info!("Association failed: {}; continuing.", e),
        }
    }

    None
}
