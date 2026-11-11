# nrfxlib-sys-pretty ![License: (MIT OR Apache-2.0) AND LicenseRef-Nordic-5-Clause](https://img.shields.io/badge/license-%28MIT%20OR%20Apache--2.0%29%20AND%20LicenseRef--Nordic--5--Clause-blue) [![nrfxlib-sys-pretty on crates.io](https://img.shields.io/crates/v/nrfxlib-sys-pretty)](https://crates.io/crates/nrfxlib-sys-pretty) [![nrfxlib-sys-pretty on docs.rs](https://docs.rs/nrfxlib-sys-pretty/badge.svg)](https://docs.rs/nrfxlib-sys-pretty) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/ariel-os/hophop/)

<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->

This crate serves several purposes at once around the [`nrfxlib_sys`][__link0] crate, focusing on the
DECT aspects (but could be extended to ther APIs in the same style):

* It re-exports its items by splitting them in semantic components. This way, code using it
  does not become cluttered with `nrf_modem_dect_mac_…` all over the place, and more
  importantly, names such as `nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_COMBINED_OP_FAILED`
  are accessible as [`dect_phy::err_values::ERR_COMBINED_OP_FAILED`][__link1].
  
  This include splitting enums into modules. (Ideally, they’d just be associated consts on the
  type, but that’d require changing bindgen output).

* It enhances documentation by cross-linking items:
  
  * Enum values are not linked from the enum type in bindgen. (This might also be addressed at
    bindgen, probably).
  * **Not implemented yet**: It moves bindgen pieces (for anonymous structs or unions) out of the way.
  * **Not implemented yet**: It splits event and notify things and links them.
* **Not implemented yet**: It gathers manually added comments where nrfxlib documentation is lacking.


 [__cargo_doc2readme_dependencies_info]: ggGmYW0CYXZlMC43LjJhdIQb2o_SNWoR6AAb3_T-k0ODPHwbnQW7uS_D2XsbjVFFtK-lC3BhYvVhcoQb50dTmFTrr2YbUjY2SsmFi_4bPP09CUdWHs8bk4BSbonOxjhhZIKCaGRlY3RfcGh59oJrbnJmeGxpYl9zeXNrNC4wLjArMy40LjA
 [__link0]: https://crates.io/crates/nrfxlib_sys/4.0.0+3.4.0
 [__link1]: https://docs.rs/dect_phy/latest/dect_phy/?search=err_values::ERR_COMBINED_OP_FAILED
