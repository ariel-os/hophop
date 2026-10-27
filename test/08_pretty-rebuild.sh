# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -ex

cd nrfxlib-sys-pretty/nrfxlib-sys-pretty-gen/
cargo run
git diff --exit-code ../src/built.rs
