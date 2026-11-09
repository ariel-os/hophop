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
APPLICATIONS="-a hophop-examples -a hophop-application-embedded-pt -a hophop-application-bridge-pt -a hophop-application-sniffer"

laze build ${BOARDS} ${APPLICATIONS} -D LOG=trace -g
