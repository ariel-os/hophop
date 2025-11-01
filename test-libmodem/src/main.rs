#![no_main]
#![no_std]
#![feature(c_variadic)] // to provide a `vsnprintf`
#![feature(sync_unsafe_cell)] // because that allows us to build buffers easily

use ariel_os::debug::{ExitCode, exit, log::error, log::info, log::trace, log::warn};

use ariel_os::hal::interrupt;

/// Set up RAM and peripherals accessed by the network core to be non-secure (as anything coming
/// from that core is NS)
///
/// Copied from nrf_modem
fn init_secure() {
    // Initializing embassy_nrf has to come first because it assumes POWER and CLOCK at the secure address
    // … but in Ariel that's already done.
    //let embassy_peripherals = embassy_nrf::init(Default::default());

    // Set IPC RAM to nonsecure … and as that's just a symbol we do that with *all* of RAM.
    let spu = embassy_nrf::pac::SPU;
    // 256 KiB total RAM, 8k pages -- might suffice to go for half that, as the docs seem to
    // indicate that only the first 128k are accessible to the radio MCU.
    const REGIONS: usize = 256 / 8;
    for i in 0..REGIONS {
        info!("Changing properties of region {}", i);
        spu.ramregion(i as usize).perm().write(|w| {
            w.set_execute(true);
            w.set_write(true);
            w.set_read(true);
            w.set_secattr(false);
            w.set_lock(false);
        })
    }

    // embassy-net-nrf91 doesn't need those two (but I don't think they do any harm during
    // exploration)

    // Set regulator access registers to nonsecure
    spu.periphid(4).perm().write(|w| w.set_secattr(false));
    // Set clock and power access registers to nonsecure
    spu.periphid(5).perm().write(|w| w.set_secattr(false));

    // but that one is needed (and that'd be the clue that GPMEM is read across the cores)

    // Set IPC access register to nonsecure
    spu.periphid(42).perm().write(|w| w.set_secattr(false));
}

#[ariel_os::task(autostart)]
async fn main() {
    info!(
        "Hello from main()! Running on a {} board.",
        ariel_os::buildinfo::BOARD
    );

    // Debugging whether and how IPC is set up was useful earlier, but not at this stage where it
    // is working. (Back then, that was the baseline for the same code block copied into
    // nrf_modem_os_timedwait).
    #[cfg(any())]
    {
        let ipc = embassy_nrf::pac::IPC_S;
        info!("Looking into the IPC state");
        for i in 0..=3 {
            info!("GPMEM[{}] {:x}", i, ipc.gpmem(i).read());
        }
        for i in 0..=7 {
            let send = ipc.send_cnf(i).read().0;
            let receive = ipc.receive_cnf(i).read().0;
            info!("_CNF[{}] SEND {:02x} RECEIVE {:02x}", i, send, receive);
        }
        info!("INTEN   {:04x}", ipc.intpend().read().0);
        info!("INTPEND {:04x}", ipc.intpend().read().0);
        info!("…and continuing as before.");
    }

    // Note that if we pulled the REGULATORS access upfront, we'd need to use REGULATORS_S!
    init_secure();

    // Copied from latest embassy-nrf init:
    let mut needs_reset = false;
    // Workaround used in the nrf mdk: file system_nrf91.c , function SystemInit(), after `#if !defined(NRF_SKIP_UICR_HFXO_WORKAROUND)`
    let uicr = embassy_nrf::pac::UICR_S;
    let hfxocnt = uicr.hfxocnt().read().hfxocnt().to_bits();
    let hfxosrc = uicr.hfxosrc().read().hfxosrc().to_bits();
    const UICR_HFXOSRC: *mut u32 = 0x00FF801C as *mut u32;
    const UICR_HFXOCNT: *mut u32 = 0x00FF8020 as *mut u32;
    if hfxosrc == 1 {
        unsafe {
            let _ = uicr_write(UICR_HFXOSRC, 0);
        }
        needs_reset = true;
    }
    if hfxocnt == 255 {
        unsafe {
            let _ = uicr_write(UICR_HFXOCNT, 32);
        }
        needs_reset = true;
    }
    if needs_reset {
        warn!(
            "UICR bits were gravely misconfigure. Fixed, but this requires a reboot; you may want to attach to the soon-running session"
        );
        cortex_m::peripheral::SCB::sys_reset();
    }

    // Ariel has them, we steal.
    let mut cp = unsafe { cortex_m::Peripherals::steal() };

    // Enable the modem interrupts
    use cortex_m::peripheral::NVIC;
    unsafe {
        NVIC::unmask(embassy_nrf::pac::Interrupt::IPC);
        cp.NVIC
            .set_priority(embassy_nrf::pac::Interrupt::IPC, 0 << 5);
        // This will just give a warning that there's no registered handler yet, but show that in
        // theory, things are wired up right.
        cp.NVIC.request(embassy_nrf::pac::Interrupt::IPC);
    }

    // Not doing this through the nrf_modem crate: That goes off doing AT stuff even during init.
    // let result = nrf_modem::init_with_custom_layout(
    //     SystemMode {
    //         lte_support: false,
    //         lte_psm_support: false,
    //         nbiot_support: false,
    //         gnss_support: false,
    //         preference: ConnectionPreference::None,
    //     },
    //     // not overriding it: even then the RAM requirement is fixed
    //     Default::default(),
    // )
    // .await;
    // info!("Result is {:?}", Debug2Format(&result));

    // inlined instead what I think matters from init_with_custom_layout:

    // The modem is only certified when the DC/DC converter is enabled and it isn't by default
    unsafe {
        embassy_nrf::pac::REGULATORS_NS
            .dcdcen()
            .write(|dcdc| dcdc.set_dcdcen(true));
    }

    unsafe { nrf_modem::init_heap() };

    // FIXME: We're not doing anything to ensure they are in the first 128K RAM (not even assert
    // that)
    static BUF1: core::cell::SyncUnsafeCell<
        [u8; nrfxlib_sys::NRF_MODEM_CELLULAR_SHMEM_CTRL_SIZE as _],
    > = core::cell::SyncUnsafeCell::new([0; nrfxlib_sys::NRF_MODEM_CELLULAR_SHMEM_CTRL_SIZE as _]);
    static BUF2: core::cell::SyncUnsafeCell<[u8; 1024]> =
        core::cell::SyncUnsafeCell::new([0; 1024]);

    // This could be the assert…
    info!("Buf at {:x}", &raw const BUF1 as usize);

    let params = nrfxlib_sys::nrf_modem_init_params {
        shmem: nrfxlib_sys::nrf_modem_shmem_cfg {
            ctrl: nrfxlib_sys::nrf_modem_shmem_cfg__bindgen_ty_1 {
                // From tracing through init it seems it's even fine with all those being 0, but
                // at some point we get "Unexpected control region size "
                base: &raw const BUF1 as _,
                size: nrfxlib_sys::NRF_MODEM_CELLULAR_SHMEM_CTRL_SIZE,
            },
            tx: nrfxlib_sys::nrf_modem_shmem_cfg__bindgen_ty_2 {
                // This would be fine to be 0 initially, but at some point the nrf_modem_init
                // function does want something from in here. (FIXME: Verify; later tests indicate
                // that that's really optional).
                base: &raw const BUF2 as _,
                size: 1024,
            },
            // From tracing through init it seems it's even fine with all those being 0, but
            // at some point we get "Unexpected control region size "
            rx: nrfxlib_sys::nrf_modem_shmem_cfg__bindgen_ty_3 { base: 0, size: 0 },
            trace: nrfxlib_sys::nrf_modem_shmem_cfg__bindgen_ty_4 { base: 0, size: 0 },
        },
        ipc_irq_prio: 0,
        // ... but those two critically need to be present.
        fault_handler: Some(modem_fault_handler),
        dfu_handler: Some(modem_dfu_handler),
    };

    // FIXME copied from nrf-modem
    unsafe extern "C" fn modem_fault_handler(_info: *mut nrfxlib_sys::nrf_modem_fault_info) {
        error!(
            "Modem fault - reason: {}, pc: {}",
            (*_info).reason,
            (*_info).program_counter
        );
    }

    unsafe extern "C" fn modem_dfu_handler(_val: u32) {
        trace!("Modem DFU handler");
    }

    let result = unsafe { nrfxlib_sys::nrf_modem_init(&params) };
    info!("modem init returned {}", result);

    let result = unsafe { nrfxlib_sys::nrf_modem_is_initialized() as u32 };
    info!("is initialized? {}", result);

    extern "C" fn handler(arg: *const nrfxlib_sys::nrf_modem_dect_phy_event) {
        info!("Handler called");
    }

    let result = unsafe { nrfxlib_sys::nrf_modem_dect_phy_event_handler_set(Some(handler)) };
    info!("handler set? {}", result);

    let result = unsafe { nrfxlib_sys::nrf_modem_is_initialized() as u32 };
    info!("is initialized? {}", result);

    let result = unsafe { nrfxlib_sys::nrf_modem_dect_phy_init() };
    info!("init returned {}", result);

    let result = unsafe { nrfxlib_sys::nrf_modem_is_initialized() as u32 };
    info!("is initialized? {}", result);

    info!("Library loaded");

    info!(
        "And we still have our params {}",
        &raw const params as usize
    );

    // exit(ExitCode::SUCCESS);
}

mod interrupts {
    use super::*;

    // this is the cortex-m way, as in the README
    //
    // // Interrupt Handler for LTE related hardware. Defer straight to the library.
    // #[interrupt]
    // #[allow(non_snake_case)]
    // fn IPC() {
    //     nrf_modem::ipc_irq_handler();
    // }

    // this is the embassy way

    use embassy_nrf::{bind_interrupts, interrupt::typelevel};

    #[doc(hidden)]
    pub struct InterruptHandler {
        _private: (),
    }

    impl typelevel::Handler<typelevel::IPC> for InterruptHandler {
        unsafe fn on_interrupt() {
            warn!("ISR :-)");
            nrf_modem::ipc_irq_handler();
        }
    }

    bind_interrupts!(struct Irqs{
        IPC => InterruptHandler;
    });

    // and is there an extra Ariel way?
}

/// Crude libc implementations, often made to panic
pub mod provide_libc {
    use super::*;

    // These are all crude, and tinyrlibc would also provide them, but so far I only see them in debug
    // output, and frankly I might rather crash-and-know right now if there's too much C string
    // handling going on.

    #[unsafe(no_mangle)]
    pub extern "C" fn strncpy(
        dst: *mut core::ffi::c_char,
        src: *const core::ffi::c_char,
        n: usize,
    ) -> *mut core::ffi::c_char {
        panic!("I don't want string operations such as strncpy to happen");
        // that's not exactly it
        let dst_slice = unsafe { core::slice::from_raw_parts_mut(dst, n) };
        let src_slice = unsafe { core::slice::from_raw_parts(src, n) };
        dst_slice.copy_from_slice(src_slice);
        dst
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn vsnprintf(
        s: *mut core::ffi::c_char,
        maxlen: usize,
        format: *const core::ffi::c_char,
        _arg: ...
    ) -> core::ffi::c_int {
        let format = unsafe { core::ffi::CStr::from_ptr(format) };
        let format_str = format.to_str().unwrap();
        if format_str.len() < maxlen {
            info!("vsnprintf relaying plain format {}", format_str);
            unsafe {
                core::slice::from_raw_parts_mut(s, format_str.len() + 1)
                    .copy_from_slice(format.to_bytes_with_nul())
            };
            format_str.len() as _
        } else {
            info!("vsnprintf using {} returning 0-len", format_str);
            unsafe { s.write(0) };
            0
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn strcmp(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char) -> isize {
        panic!("I don't want string operations such as strcmp to happen");
    }
}

/// See their users; adapted from embassy-nrf
mod uicr_helpers {
    pub unsafe fn uicr_write(address: *mut u32, value: u32) {
        uicr_write_masked(address, value, 0xFFFF_FFFF)
    }

    pub unsafe fn uicr_write_masked(address: *mut u32, value: u32, mask: u32) {
        let curr_val = address.read_volatile();
        if curr_val & mask == value & mask {
            return;
        }

        // We can only change `1` bits to `0` bits.
        if curr_val & value & mask != value & mask {
            panic!("Can't write");
        }

        // Nrf9151 errata 7, need to disable interrups + use DSB https://docs.nordicsemi.com/bundle/errata_nRF9151_Rev2/page/ERR/nRF9151/Rev2/latest/anomaly_151_7.html
        cortex_m::interrupt::free(|_cs| {
            let nvmc = embassy_nrf::pac::NVMC;

            nvmc.config()
                .write(|w| w.set_wen(embassy_nrf::pac::nvmc::vals::Wen::WEN));
            while !nvmc.ready().read().ready() {}
            address.write_volatile(value | !mask);
            cortex_m::asm::dsb();
            while !nvmc.ready().read().ready() {}
            nvmc.config().write(|_| {});
            while !nvmc.ready().read().ready() {}
        });
    }
}

use uicr_helpers::*;
