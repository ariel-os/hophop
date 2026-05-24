embedded-pt
===========

This application sets up an embedded network stack that can connect into a DECT network.

It currently uses Noridc's proprietary IPv6 mode,
which hophop calls NI6W.

Running the application
-----------------------

* Make all the basics are set up,
  see [the examples documentation](../../examples/README.md#for-all-examples)

* Set up an FT (fixed terminal) using the (non-phy) Nordict DECT Shell.

  See [the reference projects' documentation](../../doc/reference-projects.md) for details.

* Configure that peer as PT on the default network with security off:

    ```console
    desh:~$ dect sett --dev_type FT
    desh:~$ dect activate
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
    $ CONFIG_NET_IPV6_STATIC_ADDRESS=fe80::70d1:776d:f9cf:c820 CONFIG_NET_IPV6_STATIC_GATEWAY_ADDRESS=:: laze build -b nrf9151-dk -D LOG=info run
    ```

    FIXME: The IP address will initially be wrong and nothing will work.
    Run `net nbr` on the FT (see below) and run again with that now correct IP address.

* Observe on the device being eventually associated with the shell FT:

    ```console
    desh:~$ dect neighbor_list
    dect neighbor_list
    Neighbor list request initiated.
    NET_EVENT_DECT_NEIGHBOR_LIST
      Neighbor count: 1
      Neighbor list:
        Neighbor long RD ID 4027574958
    ```

* Find your device's IP address and ping it:

    ```console
    desh:~$ net nbr
       Neighbor  Interface  Flags    State     Remain  Link              Address
    [ 1] 0x2000d16c  1          0/1/0/0  reachable      0  70:D1:77:6D:70:D1:77:6D fe80::70d1:776d:f9cf:c820
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

* The PT also runs a CoAP server with EDHOC/OSCORE.
  This is currently untested until there is a suitable client or bridge,
  because I found no way to make the DECT shell send CoAP requests.
  (`net udp send` won't take hex encoded data.)
