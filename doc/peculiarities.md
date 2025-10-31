Peculiarities around components used with this library
======================================================

nRF9151
-------

* As with all hardware, beware of the [errata](https://docs.nordicsemi.com/bundle/errata_nRF9151_Rev2/page/ERR/nRF9151/Rev2/latest/err_151_new.html); in particular, careful with LFXO.

* GNSS is not available with the DECT firmware.

* IPC peripheral:
  - If you're used to nRF53 series: the synchronization time [`t_IPC`](https://docs.nordicsemi.com/bundle/ps_nrf9151/page/ipc.html#ariaid-title16) is really long (165µs, compared to 1-4µs on 53). That's not necessarily latency, and I'm not sure yet if it also applies to signals from the *same* IPC source, but when doing any IPC manually, it's probably worth re-checking something more level-triggered after any event processing has started.
  - GPMEM sounds like a perfect place to dispatch initial memory offsets through, right? Not directly, though: this is [not shared data](https://devzone.nordicsemi.com/f/nordic-q-a/115496/cannot-read-value-using-gpmem-over-ipc-on-nrf7002dk/506724). Apparently the network core reads the application core's IPC GPMEM registers, but the docs don't say that.

* Network core:
  - Beware of peristent state across application core reboots.

    At a point during exploration, a program worked only after in a single pass without disconnecting the power supply or hitting the reset button,

    - the embassy nrf9160 program was flashed and run twice (first time it paniced immediately, 2nd time it did some exchanges with the radio core)
    - the own example was started, and then saw IPC interrupts. (Without the pre-treatment, it saw none).

    Therefore, when in doubt, take note of what you did between powercycles,
    and powercycle the device before asserting that something works.
