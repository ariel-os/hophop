# Hardware usable with hophop

Currently, the project aims to support the [nRF9151](https://www.nordicsemi.com/Products/nRF9151) controller,
with the expectation to support all nRF9120 based ones
(which are nRF9151 and nRF9131;
note that the nRF916x series, while using identical radio firmware, contains different components).

The typical development board for which initial applications are laid out
is the [nRF9151-DK](https://www.nordicsemi.com/Products/Development-hardware/nRF9151-DK),
whose built-in antenna is suitable for basic DECT operation.

When experimenting with the nRF9131 through the [nRF9131-EK](https://docs.nordicsemi.com/bundle/ncs-latest/page/zephyr/boards/nordic/nrf9131ek/doc/index.html),
or any other untested board,
beware that it may not be trivial to obtain antennas for the DECT-2020 NR+ bands.

Note that the devices as shipped are running an LTE firmware on their network core;
see [the DECT firmware page](./dect-firmware.md) on how to change that.
