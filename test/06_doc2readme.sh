# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

set -ex

for DIR in ts-103-636-numbers ts-103-636-utils
do
    cd "${DIR}"
    cargo doc2readme --check
    cd ..
done
