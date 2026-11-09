<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
embedded-pt
===========

This application sets up an embedded network stack that can connect into a DECT network.

It currently uses Noridc's proprietary IPv6 mode,
which hophop calls NI6W.

Data flow
---------

This application is run on a development kit, usually (but not necessarily) connected to a PC.
The application sets up hophop instrument the MAC layer of the radio core.
IP traffic from the DLC layer (exposed from the radio core) is fed into the Ariel OS IP stack,
whereas IP traffic from the host through slipmux is ignored.
Both the host and the network can access a "hello world" user application through CoAP.
In addition, they can also use CoAP to control hophop's process of joining and leaving DECT NR+ networks.

![Illustration of the components described above in prose](../../doc/.media/scenarios.svg#page-embedded-pt)

Running the application
-----------------------

* Make all the basics are set up,
  see [getting-started documentation](../../doc/getting-started.md)

* Set up an FT (fixed terminal) using the (non-phy) Nordict DECT Shell.

  See [the reference projects' documentation](../../doc/reference-projects.md) for details.

* Configure that DECT Shell peer as FT on the default network with default security (dummy keys):

    ```console
    desh:~$ dect sett --dev_type FT
    desh:~$ dect cluster_start 1665
    ```

    If anything fails, compare against my settings:

    ```console
    desh:~$ dect sett -r
    DECT settings:
      Network ID:                           2271560481 (0x87654321)
      Transmitter Long RD ID:               1892775789 (0x70d1776d)
      Region/variant:                       eu
      Band:                                 1
      Auto activate:                        on
      Device type:                          FT
      Max TX power:                         19 dBm
      Max MCS:                              4
      Power save:                           off
    Common RSSI scanning settings:
      ch access: RSSI scan (msecs) per ch:  200
      ch access: RSSI scan busy (dBm):      signal level > -71
      ch access: RSSI scan possible (dBm):  -71 >= signal level > -85
      ch access: RSSI scan free (dBm):      signal level <= -85
      ch access: SCAN_SUITABLE%             75%
      Association related:
       Max cluster beacon RX fails:         10
      Cluster beacon:
       Period:                              2000 ms
       Max beacon TX power:                 10 dBm
       Max cluster TX power:                13 dBm
       Max num of neighbors:                50
       Channel reselection:                 enabled
       Channel reselection threshold:       80%
       Neighbor inactivity time:            1800000 ms
      Network beacon:
       Period:                              2000 ms
       Channel:                             1665
      Network join:
       Target FT:                           Any FT
       MIN_SENSITIVITY_LEVEL:               -95 dBm
      Security configuration:
       Security mode:                       mode_1
       Security integrity key:              4A7573744164656661756C7421212121
       Security cipher key:                 4A7573744164656661756C7421212121
    ```

* Run the example:

    ```console
    $ laze build -b nrf9151-dk -D LOG=info run
    ```

    Among other log messages, you will see how it connects, and that it obtains an IP address:

    ```
    [INFO ] Scanning succeeded, attempting association with the found FT ClusterBeacon { channel: 1665, … }
    [INFO ] Learned about association: Some(Association { network: 0x87654321, … }
    [INFO ] Setting IPv6 address: fe80:0000:0000:0000:70d1:776d:f9cf:c820
    ```

    **Not associating?**
    Compare the FT's settings with the settings above, maybe the channel, network ID or encryption key differs from those shown above (`dect sett -r`).

* You can also query the association status from Jelly:

  ```console
  $ Jelly /dev/serial/by-id/usb-Makerdiary_IFMCU_CMSIS-DAP_0123456789ABCDEF-if00
  ```

  (For small numbers of devices, you may also use the shorter `/dev/ttyACM0` style names,
  but the longer names are easier to match with the programmers.)

  Jelly automatically discovers the available resources
  (as shown in the CoAP pane on the right),
  as well as some details about the board.

  Enter `/hop/assoc` (tab completion is available);
  pressing return will send a GET request to that resource,
  and the CoAP pane will show that an association is present:

  ```cbor
  {0: 2271560481, 1: 160740142, 2: 1665, 3: [0]}
  ```

  (Items here signify the network ID, the parent's long RD ID, the channel number, and the type of IPv6 address, respectively).

* You can also verify the association from the shell at the FT:

    ```console
    desh:~$ dect neighbor_list
    dect neighbor_list
    Neighbor list request initiated.
    NET_EVENT_DECT_NEIGHBOR_LIST
      Neighbor count: 1
      Neighbor list:
        Neighbor long RD ID 4027574958
    desh:~$ net nbr
       Neighbor  Interface  Flags    State     Remain  Link              Address
    [ 1] 0x2000d16c  1          0/1/0/0  reachable      0  70:D1:77:6D:70:D1:77:6D fe80::70d1:776d:f9cf:c820
    ```

    Now you can ping it from the FT:

    ```console
    desh:~# ping -d fe80::70d1:776d:f9cf:c820
    Initiating ping to: fe80::70d1:776d:f9cf:c820
    Source IP addr: fe80::70d1:776d:70d1:776d
    Destination IP addr: fe80::70d1:776d:f9cf:c820
    Pinging fe80::70d1:776d:f9cf:c820 results: time=1.174secs, payload sent: 0, payload received 0
    Pinging fe80::70d1:776d:f9cf:c820 results: time=0.989secs, payload sent: 0, payload received 0
    Pinging fe80::70d1:776d:f9cf:c820 results: time=0.991secs, payload sent: 0, payload received 0
    Pinging fe80::70d1:776d:f9cf:c820 results: time=0.991secs, payload sent: 0, payload received 0
    Ping statistics for fe80::70d1:776d:f9cf:c820:
        Packets: Sent = 4, Received = 4, Lost = 0 (0% loss)
    Approximate round trip times in milli-seconds:
        Minimum = 989ms, Maximum = 1174ms, Average = 1036ms
    Pinging DONE
    ```

    The latency comes from the Nordic MAC's mode of operation
    and the configured beacon intervals (and possibly the PT being configured in power saving mode):
    There are no regular allocations for downstream traffic,
    it is only announced on demand in a beacon --
    and the default beacon interval is 2 seconds.

    So the first ping waits for however long it takes to the next beacon,
    that beacon announces data for the other device in a Broadcast Indication,
    then sends it,
    and our application responds during RACH.

    The ping command then waits 1 second, starts the next ping,
    but then it takes again 1s until the next beacon is availab.e

* Settings of the PT are available through Jelly as well:
  Enter `/hop/config` (again, `/hop/c<TAB>` auto-completes),
  and it shows the default configuration:

  ```cbor
  {0: [{0: [1665], 1: 8873283, 2: [h'4a7573744164656661756c7421212121', h'4a7573744164656661756c7421212121'], 3: 2000}], 1: 3898463644, 2: true}
  ```

  (Pressing F3 makes the CoAP pane full-screen).

  This lists that there is one known network, to be scanned at channel 1665, with encryption keys given in hexadecimal and a beacon interval of 2000ms;
  beyond the configured networks, it also lists the own RD ID and whether to use power saving mode.

  When you enter the `edit` command,
  Jelly will open your editor, and allow you to alter those settings,
  which are applied when saving and exiting.

  You can use this to associate with different FTs,
  or to intentionally sever the network connection.

  In the current example application, those settings are not persisted across reboots.

* The PT also runs a CoAP server with EDHOC/OSCORE.
  This can be tested most easily using the `bridge-pt` example.
