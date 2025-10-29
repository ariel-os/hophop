# Steps to get the DECT firmware onto the nRF9151 network core

For any of the code here to work, the nRF9151's network core
(which is not usually programmed by Ariel OS or any other RTOS)
needs to be flashed with dedicated firmware provided by Nordic
(`mfw-nr+_nrf91x1_1.1.0.zip`).

This image is currently not available for public download;
quoting [its docs](https://docs.nordicsemi.com/bundle/ncs-latest/page/nrfxlib/nrf_modem/doc/dectphy.html),
"you must contact the Nordic Semiconductor sales department".

Flashing this may be obvious for regular users of Nordic's tools,
but not to those using typical Rust workflows:

* Download `nrfutil` from <https://www.nordicsemi.com/Products/Development-tools/nRF-Util/Download>
* Run `nrfutil install 91`, which downloads extra code into your home directory (`~/.nrfutil`).
* Ensure that Segger's JLink is installed in the exact location nrfutil expects it to be
  (eg. by unpacking `https://www.segger.com/downloads/jlink/JLink_Linux_V880_x86_64.tgz` in /opt/SEGGER/JLink).
* Run `nrfutil 91 modem-firmware-upgrade --firmware mfw-nr+_nrf91x1_1.1.0.zip --all-jlinks`
  (or specify the concrete JLink device).
