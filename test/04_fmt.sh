# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -ex

cargo fmt --check
for CRATE in examples/ applications/*/
do
    cargo fmt --check --manifest-path ${CRATE}/Cargo.toml
done
