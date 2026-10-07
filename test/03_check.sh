# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -ex

RUSTFLAGS="-D warnings" cargo check --workspace
RUSTFLAGS="-D warnings" cargo check --workspace --all-features
cargo clippy --workspace -- --deny clippy::all --deny clippy::pedantic
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
