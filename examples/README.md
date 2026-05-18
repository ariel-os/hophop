<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
Running hophop examples
=======================

For all examples
----------------

* Get an [nRF9151-DK](../doc/hardware.md), connect it and turn it on.

* Ensure you are set up to run Ariel OS examples.

  For the time being, it's easiest to follow the [Getting Started section in the Ariel OS book](https://ariel-os.github.io/ariel-os/dev/docs/book/getting-started.html).

* [Ensure that you have the DECT firmware running](../doc/dect-firmware.md).

* Beware that this is a research example,
  and that depending on your location, regulation on operating these devices does apply,
  especially as some examples emit transmissions controlled by the user alone.

* If ever you have multiple DKs connected,
  probe-rs will give you interactive options.

  You can skip that and statically set the probe to use
  by adding `-- --probe 1366:1059:xxxxxxxxxxxx` after the laze call (or withtout the `--` if you already).

Running the RSSI example
------------------------

* In this directory, run:

    ```console
    $ laze build -b nrf9151-dk run --bin rssi
    ```

    You can add the option `-D LOG=trace` before the `run` for more verbosity.

    The output this produces is a scan of Band 1, which is printed on screen;
    it's best to let that run through.

*   To visualize the output, store it by running

    ```
    $ laze build -b nrf9151-dk run --bin rssi -- --target-output-file=rssi.log
    ```

    and run the visualizer:

    ```
    $ ./show-rssi.py rssi.log
    ```

    (If anything is missing on the Python side, `pipx run ./show…` will fetch any missing components).

Running the RX example
----------------------

```
$ laze build -b nrf9151-dk run --bin rx
```

Note that this only produces output when data is sent on channel 1665;
use `dect ping -c` or `dect mac beacon_start -c 1665`
in the Nordic `dect_shell` example to send data from another boad.

Running the TX example
----------------------

```
$ laze build -b nrf9151-dk run --bin tx
```

This produces a single manually crafted beacon on chnanel 1665
whenever Button 1 is pressed.
Beware that this does not perform LBYT:
The user is expected to monitor that channel in parallel.

Running the ping example
------------------------

Note that this is particularly experimental w/rt exploring radio:
This emits bursts of transmissions that may easily be outside of regulations,
and best done in isolation of any other systems in the 1.9GHz band.

This requires at least two boards both running the same software, so in two terminals:

```console
laze build -b nrf9151-dk -D LOG=info -d stable -- --probe 1366:1059:00105aaaaaaa --target-output-file delta-a.log
laze build -b nrf9151-dk -D LOG=info -d stable -- --probe 1366:1059:00105bbbbbbb --target-output-file delta-b.log
```

(For some serial numbers easily found with `probe-rs list`).

When both are up, take turns pressing the boards' button 1 in alternation for several seconds.
A device sends short beacon messages that contain a time stamp on its own clock;
whenever it receives a message, it prints that timestamp but also its own time stamp.
Time stamps are on a 69.120MHz clock, and count from startup.

You can check the results with `python3 show-ping.py delta-a.log delta-b.log`, which shows you the device IDs (derived from Ariel OS MAC addresses, these are *not* the programmer's IDs)
along with their packet counts.

You can visualize the results using `python3 show-ping.py delta-a.log delta-b.log --plot 123 456`
(numbers taken from the previous command's output)
which (possibly after zooming in) shows something like this:

![Scatter plot](.media/ping-example.png)

It shows:
* A red dot for all pairs of messages sent by one device and received by the other, and vice versa, along with respective trend lines.
* Vastly inhomogenous axes:
  The x axis spans the whole experiment (typically a minute),
  while the y axis (in the same units) typically has a range of a few hundred ticks (ie. several microseconds).
* For every event, the x axis expresses the average between the two devices' clocks,
  and the y axis expresses the difference between those clocks.
* If both clocks were perfect clocks (but not started at the same time),
  all lines would be horizontal, and their vertical position would express the time difference between their startup times.
  As clocks are not perfect, the slope shows that the two clocks are slowly drifting relative to each other.
  Drifts are typically in the sub-PPM range, in total.
* Dots of the same color scatter evenly around their center line with a width of approx. 20 ticks.
  This indicates that in some part of the transmit-receive cycle, there is a uniformly distributed random time offset.
* The red and blue center lines are about 50 ticks apart,
  leaving a space of 30 ticks.
  This corresponds to some delay between transmission and reception.
  (Note that the time stamp at reception indicates when the STF was recognized, so this may just be the time that recognition takes).

  In theory, devices further apart should have that space widen by approx. 1 tick every 5m in distance.

Running this with more devices shows additional black dots:
Those represent events heard by both observing devices.
You'll observe that those are scattered more widely (approx. 40 ticks),
indicating that the uniform distribution is mostly an artifact of reception.
(If only transmission times were scattered and reception times were precise,
we would see them as a thin line, for there is only a single physical transmit event measured by two receivers).

Running the example building on the full Nordic stack
-----------------------------------------------------

* Set up a peer using the (non-phy) Nordict DECT Shell.

  (Sorry. One step at a time!)

* Configure that peer as PT on the default network with security off:

    ```
    desh:~$ dect sett --dev_type FT
    desh:~$ dect activate
    desh:~$ dect sett --sec_mode=none
    desh:~$ dect cluster_start 1665
    ```

    If anything fails, compare against my settings:

    ```
    desh:~$ dect sett -r
    dect sett -r
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
       Security mode:                       none
    ```

* Run the example:

    ```console
    $ laze build -b nrf9151-dk -D LOG=info run --bin nordicmacdemo
    ```

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

* Send from the FT to the hophop PT:

    ```console
    desh:~$ dect tx -t 4027574958 -d hello
    ```

    and watch it pop up in the hophop output.

* Works even for IP:

  ```console
  desh:~$ net nbr
  [ 1] 0x2000d16c  1          0/1/0/0  reachable      0  70:D1:77:6D:70:D1:77:6D fe80::70d1:776d:f00f:eaae
  desh:~$ ping -d fe80::70d1:776d:f00f:eaae
  ```

  (No responses yet, though.)
