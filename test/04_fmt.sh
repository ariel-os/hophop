# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -ex

cargo fmt --check
for APP in embedded-pt bridge-pt
do
    cargo fmt --check --manifest-path applications/$APP/Cargo.toml
done
