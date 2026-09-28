#!/bin/sh
# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0

# Kept in a shell script to be easily portable to no-GitHub CI systems.
#
# This expects the Ariel OS "getting started" setup to be present, and suitable
# caching options to be set.

set -ex

FAILED=""

for x in ./test/[0-9]*.sh
do
    "$x" || FAILED="${FAILED}+$x"
done

[ x"${FAILED}" = x"" ]

