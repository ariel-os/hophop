// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Tools for defmt-printing recurring `nrfxlib` structs.

use defmt::info;
use nrf_modem::nrfxlib_sys;

pub(crate) fn debug_ies(ies: &[nrfxlib_sys::nrf_modem_dect_mac_ie]) {
    // This'd be easier if we just got the slice of the data
    for ie in ies {
        // I'm not fully sold that this is accurate.
        info!("IE: ie_type {}", ie.ie_type);
        #[cfg(false)]
        info!(
            "data dump: {}",
            Hex(unsafe {
                core::slice::from_raw_parts(
                    &ie.ie as *const _ as *const u8,
                    core::mem::size_of::<nrfxlib_sys::nrf_modem_dect_mac_ie__bindgen_ty_1>(),
                )
            })
        );
        match ie.ie_type {
            nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_RD_CAPABILITY => {
                let details = unsafe { &ie.ie.rd_capability };
                info!("  RD Capability: group_assign {} paging {} mesh {} scheduled {} dect_delay {} half_dup {} release {} operating_mode {} mac_security {} dlc {}",
                    details.group_assign_supported,
                    details.paging_supported,
                    details.mesh_supported,
                    details.scheduled_access_supported,
                    details.dect_delay_supported,
                    details.half_dup_supported,
                    details.release,
                    details.operating_mode,
                    details.mac_security,
                    details.dlc_service_type,
                );
                info!("    phy cap: power {} max_nss {} ... max_mcs {} ...",
                    details.phy_capabilities.power_class,
                    details.phy_capabilities.max_nss,
                    details.phy_capabilities.max_mcs,
                );
            },
            nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_RANDOM_ACCESS_RESOURCE => {
                let details = unsafe { &ie.ie.random_access_resource };
                info!("  Random Access Resource: channel {}, response_channel {}, max_tx {}, repetition {}",
                    unsafe { (&raw const details.channel).read_unaligned() },
                    unsafe { (&raw const details.response_channel).read_unaligned() },
                    details.max_rach_tx_length,
                    details.repetition,
                );
                info!("    allocation start {} slots {} length {}",
                    details.allocation.start_subslot,
                    details.allocation.use_slots,
                    unsafe { (&raw const details.allocation.length).read_unaligned() },
                );
            },
            _ => (),
        }
    }
}
