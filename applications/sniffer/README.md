<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
sniffer
=======

This application continuously receives from its DECT NR+ interface,
sends out received packets in Wireshark's format,
and can be controlled through CoAP.

Data flow
---------

This application is run with a PC connected to a development kit.
The application sets up hophop instrument the PHY layer of the radio core.
Control data (switching channels) is sent in through CoAP,
and IP traffic is produced directly by the application, bypassing the Ariel IP stack, by using static headers.
Both CoAP and IP traffic is multiplexed through slipmux to the host where Jelly provides an interface to CoAP,
and makes radio traffic visible on the Linux network stack for Wireshark to dissect.

![Illustration of the components described above in prose](../../doc/.media/scenarios.svg#page-sniffer)

Running the application
-----------------------

* Run the example:

    ```console
    $ laze build -b nrf9151-dk run
    ```

* Use SLIP to connect your device's UART to a Linux tun interface.

  You can do this by:

  ```console
  $ Jelly /dev/serial/by-id/usb-SEGGER_J-Link_001012345678-if00 -t tun0
  ```

* Run Wireshark on tun0, configured as follows:

  - In Edit / Preferences / Protocols / DECT NR+, set Cipher Key 0 to "4a7573744164656661756c7421212121" (or whichever network you expect to decode), and port to 45319

* Successful sniffing not only requires that the channel is configured right.
  For capturing data (but not beacons), it also requires that the network ID is configured right,
  since there is scrambling on the PHY layer.
  That is, data is not encrypted, but the encoding is influenced by the network ID to reduce interference.

  You can view the settings in Jelly by accessing the `/sniffconf` resource
  (i.e., entering `/sniffconf` in the user input field; tab completion is available):

  ```cbor
  {0: 1665, 1: 2271560481}
  ```

  This indicates the current channel and the network ID, respectively.

  Using the `edit` command,
  you can alter the channel and network ID.
