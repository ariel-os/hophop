#!/bin/sh
# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

# Kept in a shell script to be easily portable to no-GitHub CI systems.
#
# This expects the Ariel OS "getting started" setup to be present, and suitable
# caching options to be set.

set -ex

pipx run reuse lint
cargo vet check

RUSTFLAGS="-D warnings" cargo check --workspace
RUSTFLAGS="-D warnings" cargo check --workspace --all-features
cargo clippy --workspace -- --deny clippy::all --deny clippy::pedantic
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features
cargo fmt --check
# hophop can't be built on host architectures
cargo test --workspace --exclude hophop
cargo test --workspace --all-features --exclude hophop

for DIR in ts-103-636-numbers ts-103-636-utils
do
    cd "${DIR}"
    cargo doc2readme --check
    cd ..
done

# Initially those do build tests only; turning clippy and checks on is a good
# next step, but only once these stabilize a little.

cd examples
# We enable IPv6, and current Ariel needs something in there
export CONFIG_NET_IPV6_STATIC_ADDRESS=fe80::1
export CONFIG_NET_IPV6_STATIC_GATEWAY_ADDRESS=::
# FIXME: Going through `run` but not really -- because a plain build fails due to the multiple binaries.
for EX in rx tx rssi ping
do
    laze build -b nrf9151-dk -D LOG=trace -D CARGO_RUNNER=true run --bin ${EX}
done
cd ..

cd applications
for APP in embedded-pt bridge-pt
do
    laze build -C $APP/ -b nrf9151-dk -D LOG=trace
done
