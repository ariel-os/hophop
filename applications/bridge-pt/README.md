<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
bridge-pt
===========

This application sets up DECT IPv6 networking (in NI6W mode)
and connects it out via SLIP.

Running the application
-----------------------

* Run a peer with the `embedded-pt` applications (which means also running a DECT shell FT)
  as per its [documentation](../embedded-pt/README.md)

* Run the example:

    ```console
    $ laze build -b nrf9151-dk -D LOG=info run
    ```

* Use SLIP to connect your device's UART to a Linux tun interface.

  You can do this by:

  ```console
  $ cargo install Jelly
  $ Jelly /dev/ttyACM2 -t tun0
  ```

  which will guide you towards setting up your tun0 device if it is not available yet.

* Run `net nbr` on the FT to see your own IP address --
  NI6W is rather picky when it comes to those.

  ```console
  $ sudo ip -6 a del dev tun0 fe80::7d99:e9b0:922f:5140/64
  ```

  Resist the temptation to remove the auto-generated address:
  Linux or NetworkManager will restore it after a few seconds,
  Linux will pick the most recent address as source address,
  so if you remove it, you just make it more powerful,
  and break the demo.

* Ping your `embedded-pt` device based on the IP address the FT told you:

  ```console
  $ ping fe80::70d1:776d:f9cf:c820%tun0
  64 bytes from fe80::70d1:776d:f9cf:c820%tun0: icmp_seq=1 ttl=64 time=3077 ms
  64 bytes from fe80::70d1:776d:f9cf:c820%tun0: icmp_seq=2 ttl=64 time=2086 ms
  64 bytes from fe80::70d1:776d:f9cf:c820%tun0: icmp_seq=3 ttl=64 time=3066 ms
  ^C
  ```

* Use aiocoap to exchange encrypted CoAP communication.

  ```console
  $ pipx run --spec 'aiocoap[oscore,prettyprint]' aiocoap-client 'coap://[fe80::70d1:776d:f9cf:c820%tun0]/hello' --credentials ../embedded-pt/credentials.diag
  ```

  This takes two round-trips to run EDHOC, so it takes a few seconds,
  and the IP address  occasionally upsets things,
  especially since the CoAP transmission parameters do not expect the latency of the pings.
  This has room for optimization on many layers;
  for the time being, be patient and retry.

  (Some notes-to-self:
  The NI6W layer reorders packets somewhere,
  something duplicates packets a lot,
  aiocoap could be more patient when reordered unsuccessful OSCORE responses come back,
  and the CoAP parameters should be tuned for 2s+ expected RTTs, avoiding needless retransmissions.
  Running the example in a mode where not every invocation runs EDHOC will help too.)

  Use the logs, `--verbose` aiocoap output, or Wireshark
  to verify that .

  Note that those credentials do not contain real keys, but merely state that opportunistic encryption is used.
  Ariel OS contains examples of using actual credentials,
  but that would only increase the demo complexity,
  without significant changes to message size/content or computational complexity.
