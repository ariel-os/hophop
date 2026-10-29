<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
sniffer
=======

This application continuously receives from its DECT NR+ interface,
sends out received packets in Wireshark's format,
and can be controlled through CoAP.

Running the application
-----------------------

* Run the example:

    ```console
    $ laze build -b nrf9151-dk run
    ```

* Use SLIP to connect your device's UART to a Linux tun interface.

  You can do this by:

  ```console
  $ cargo install Jelly
  $ Jelly /dev/serial/by-id/usb-SEGGER_J-Link_001012345678-if00 -t tun0
  ```

* Run Wireshark on tun0, configured as follows:

  - In Edit / Preferences / Protocols / DECT NR+, set Cipher Key 0 to "4a7573744164656661756c7421212121" (or whichever network you expect to decode), and port to 45319
