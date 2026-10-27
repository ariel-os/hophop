// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs a DECT FT and forwards IP traffic to a UART.

#![no_std]
#![no_main]

use ariel_os::log::info;

use ts_103_636_utils::identifiers::{AbsoluteChannel, LongRdId};

#[ariel_os::task(autostart)]
async fn main() {
    /* FIXME: duplicate from ../embedded-pt/ */
    info!("Initializing DECT MAC, trusting that Ariel OS did the basic setup");
    let mut dect = hophop::nrfxlib_mac::DectMac::create(());

    dect.systemmode_set_mac().await;

    let our_long_id = LongRdId::new(u32::from_be_bytes(
        *ariel_os::identity::device_id_bytes()
            .expect("we know this platform to have one")
            .as_ref()
            .first_chunk()
            .expect("we know this platform has sufficiently long IDs"),
    ))
    .expect("serial numbers used with examples are not so unlucky as to start with 4 byte zeros");
    info!("Our Long RD ID is {:?}", our_long_id);

    /* end duplicate */

    dect.control_functional_mode_set_deactivate().await;

    let keys = Some(hophop::association::Mode1Keys {
        // 4A7573744164656661756C7421212121 is more readable as:
        cipher_key: b"JustAdefault!!!!",
        integrity_key: b"JustAdefault!!!!",
    });
    let security = if let Some(hophop::association::Mode1Keys {
        integrity_key,
        cipher_key,
    }) = keys
    {
        nrfxlib_sys::nrf_modem_dect_control_configure_params__bindgen_ty_1 {
            mode: nrfxlib_sys::nrf_modem_dect_mac_security_mode_NRF_MODEM_DECT_MAC_SECURITY_MODE_1,
            integrity_key: *integrity_key,
            cipher_key: *cipher_key,
        }
    } else {
        nrfxlib_sys::nrf_modem_dect_control_configure_params__bindgen_ty_1 {
            mode:
                nrfxlib_sys::nrf_modem_dect_mac_security_mode_NRF_MODEM_DECT_MAC_SECURITY_MODE_NONE,
            integrity_key: [0; _],
            cipher_key: [0; _],
        }
    };

    dect.control_configure(&mut nrfxlib_sys::nrf_modem_dect_control_configure_params {
        // The maximum; we'll leave it to the network to lower the limits.
        // FIXME: take from hardware (although here it's the the standard's limit, but maybe we
        // have to adjust to a lower one from the hardware?)
        // FIXME why does this fail when we put in 23_DB?
        max_tx_power: nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_10_DB,
        // FIXME: take from hardware
        max_mcs: nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_4,
        // FIXME: Decide (this is what the vendor examples default to)
        expected_mcs1_rx_rssi_level: -68,
        // FIXME: could we decide that later? (probably not: we come in with som expectation of
        // the network already)
        long_rd_id: our_long_id.into(),
        // FIXME: configure
        phy_band_group_index:
            nrfxlib_sys::nrf_modem_dect_mac_band_group_index_NRF_MODEM_DECT_MAC_PHY_BAND_GROUP_IDX0,
        power_save: false,
        security,
        // FIXME: Decide (this is what the vendor uses in their examples)
        stats_averaging_length: 2,
    })
    .await;

    dect.control_functional_mode_set_activate().await;

    use nrf_modem::ErrorSource;
    let result = unsafe {
        nrfxlib_sys::nrf_modem_dect_mac_rssi_scan(
            &mut nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_params {
                // -71dBm..-85dBm and 200ms: defaults from dects_hell
                channel_scan_length: 200, // ms
                threshold_min: -85,       // dBm
                threshold_max: -71,       // dBm
                band: nrfxlib_sys::nrf_modem_dect_mac_band_NRF_MODEM_DECT_MAC_PHY_BAND1,
                // or set to 0 to scan the whole band
                num_channels: 1,
                channel_list: [1665; _],
            },
        )
    }
    .into_result();
    info!("Starting RSSI scan: {:?}", result);
    let result = dect.receive_singleton_event().await;
    info!("Done: {:?}", result);

    // Pick one by a process similar to how eMuhdra picks their secret keys
    let channel = AbsoluteChannel::new(1665).unwrap();
    info!("Continuing with channel {}", channel);

    // FIXME all parameters
    let mut association_config = nrfxlib_sys::nrf_modem_dect_mac_association_config {
        // as used in samples/dect/dect_shell/prj.conf, for example
        max_num_neighbours: 50,
        // documented as not supported, but picking the only value used in the v3.4.0 source
        max_num_ft_neighbours: 0,
        neighbor_info_triggers:
            nrfxlib_sys::nrf_modem_dect_mac_association_config__bindgen_ty_1 {
                inactivity_timer: 60_000, // 60s
            },
        default_tx_flow_config:
            // AIU those are for the 2 signalling flows and the 4 user flows
            [
            nrfxlib_sys::nrf_modem_dect_mac_association_config__bindgen_ty_2 {
                priority: 0,
                dlc_service_type: nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3,
                dlc_sdu_lifetime: nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S,
            },
            nrfxlib_sys::nrf_modem_dect_mac_association_config__bindgen_ty_2 {
                priority: 0,
                dlc_service_type: nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3,
                dlc_sdu_lifetime: nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S,
            },
            nrfxlib_sys::nrf_modem_dect_mac_association_config__bindgen_ty_2 {
                priority: 3,
                dlc_service_type: nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3,
                dlc_sdu_lifetime: nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S,
            },
            nrfxlib_sys::nrf_modem_dect_mac_association_config__bindgen_ty_2 {
                priority: 4,
                dlc_service_type: nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3,
                dlc_sdu_lifetime: nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S,
            },
            nrfxlib_sys::nrf_modem_dect_mac_association_config__bindgen_ty_2 {
                priority: 5,
                dlc_service_type: nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3,
                dlc_sdu_lifetime: nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S,
            },
            nrfxlib_sys::nrf_modem_dect_mac_association_config__bindgen_ty_2 {
                priority: 6,
                dlc_service_type: nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3,
                dlc_sdu_lifetime: nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S,
            },
            ],
    };

    // TX power and RACH seem to be mandatory
    let mut config_flags =
        nrfxlib_sys::nrf_modem_dect_mac_cluster_config__bindgen_ty_1__bindgen_ty_1 {
            _bitfield_1: nrfxlib_sys::__BindgenBitfieldUnit::new([0]),
            _bitfield_align_1: [0; 0],
        };
    config_flags.set_has_max_tx_power(1);
    config_flags.set_has_rach_config(1);

    // FIXME all parameters
    let mut cluster_config = nrfxlib_sys::nrf_modem_dect_mac_cluster_config {
        // flags
        __bindgen_anon_1: nrfxlib_sys::nrf_modem_dect_mac_cluster_config__bindgen_ty_1 { flags: config_flags },
        // don't be fooled: that's a nibble value as output by dect_utils_lib_dbm_to_phy_tx_power
        // (undocumented, but clear from dect_shell code) -- just so happens that the value 10
        // (0x0a) happens to also encode for 10dBm
        beacon_tx_power: 10,
        // not sure, but there are limited sets of allowed values
        count_to_trigger: nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_2,
        relative_quality: 0,
        min_quality: 0,
        // see beacon_tx_power
        cluster_max_tx_power: 10,
        cluster_beacon_period: nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_500_MS,
        cluster_channel: channel.into(),
        network_id: 0x87654321,
        rach_configuration: nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config {
            // probably disregarded due to bit mask
            policy: nrfxlib_sys::nrf_modem_dect_mac_rach_config_policy_NRF_MODEM_DECT_MAC_RACH_CONFIG_POLICY_FILL,
            common: nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_common_config {
                response_window_length: 8,
                max_transmission_length: 8,
                cw_min_sig: 2,
                cw_max_sig: 7,
            },
            config: nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config {
                fill:  nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_fill {
                    percentage: 100,
                },
                // probably not used because policy is set to FILL
                periodic: nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_periodic {
                    period: 0,
                    length: 0,
                },
            }
        },
        triggers: nrfxlib_sys::nrf_modem_dect_mac_cluster_config_cluster_info_triggers {
            busy_threshold: 20,
        },
        ipv6_config: nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_config_t {
            type_: nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_NONE,
            address: [0; 16],
        },
    };

    let result = unsafe {
        nrfxlib_sys::nrf_modem_dect_mac_cluster_configure(
            &mut nrfxlib_sys::nrf_modem_dect_mac_cluster_configure_params {
                // FIXME all parameters

                // as used anywhere in v3.4.0 code (but probably should be influenced by the RSSI
                // bit field)
                cluster_period_start_offset: 0,
                association_config: &mut association_config,
                cluster_config: &mut cluster_config,
            },
        )
    }
    .into_result();
    info!("Setting cluster coniguration: {:?}", result);
    let result = dect.receive_singleton_event().await;
    info!("Done: {:?}", result);

    loop {
        ariel_os::time::Timer::after_secs(5).await;
    }
}
