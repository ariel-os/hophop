# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -ex

# hophop can't be built on host architectures
cargo test --workspace --exclude hophop
cargo test --workspace --all-features --exclude hophop
