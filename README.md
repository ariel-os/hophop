<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
# hophop

*Making DECT-2020 NR+ accessible in Rust.*

This project provides high-level tools and applications to make ETSI TS 103 636 ("DECT-2020 New Radio (NR)", also sometimes called DECT nr+) available in Rust.
This encompasses low-level utility libraries such as [`ts-103-636-numbers`] and [`ts-103-636-utils`],
Rust wrappers for using either the PHY or the MAC mode of Nordic chips inside the [`hophop` crate]
(with the intention to eventually provide unified integration for those modes and for future chips),
and [examples] and [applications] that illustrate usage or provide early functionality.

To jump right into using it, see the [getting-started documentation].

This project is currently active,
and documentation may be incomplete and/or outdated.
If you have any questions, anything is unclear or does not work, please open an issue in the issue tracker.

Also, have a picture of two hops:

![Two birds, Eurasian hoopoes, sitting](https://upload.wikimedia.org/wikipedia/commons/thumb/c/c5/Hop_met_Jong_%2818990969281%29.jpg/960px-Hop_met_Jong_%2818990969281%29.jpg)

(Picture by rob Stoeltje, [CC-BY](https://flickr.com/photos/46198971@N06/18990969281)) <!-- via Wikimedia Commons, https://commons.wikimedia.org/wiki/File:Hop_met_Jong_(18990969281).jpg -->

[`ts-103-636-numbers`]: https://crates.io/crates/ts-103-636-numbers
[`ts-103-636-utils`]: https://crates.io/crates/ts-103-636-utils
[getting-started documentation]: ./doc/getting-started.md
[`hophop` crate]: ./hophop/src/
[examples]: ./examples/
[applications]: ./applications/
