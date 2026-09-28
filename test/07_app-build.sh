# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -ex

# Initially those do build tests only; turning clippy and checks on is a good
# next step, but only once these stabilize a little.

# Listing them all so we notice if any fail. (There can be differences in
# build-time issues like "do we have a case for that UART configuration", but
# also laze-time requirements like "this needs an LED", and I'd rather exclude
# them explicitly here then to make sure this matches the docs).
BOARDS="-b nrf9151-dk -b makerdiary-nrf9151-connect-kit -b nordic-thingy-91-x-nrf9151"

cd examples
# We enable IPv6, and current Ariel needs something in there
export CONFIG_NET_IPV6_STATIC_ADDRESS=fe80::1
export CONFIG_NET_IPV6_STATIC_GATEWAY_ADDRESS=::
# FIXME: Going through `run` but not really -- because a plain build fails due to the multiple binaries.
for EX in rx tx rssi ping
do
    laze build ${BOARDS} -D LOG=trace -D CARGO_RUNNER=true --multiple-tasks run --bin ${EX}
done
cd ..

cd applications
for APP in embedded-pt bridge-pt
do
    laze build -C $APP/ ${BOARDS} -D LOG=trace
done
