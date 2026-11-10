// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Runs a DECT FT and forwards IP traffic to a UART.

#![no_std]
#![no_main]

use ariel_os::log::{info, warn, Hex};

use ts_103_636_utils::identifiers::{AbsoluteChannel, LongRdId};

use nrfxlib_sys_pretty::{dect_control, dect_dlc, dect_mac};

#[ariel_os::task(autostart)]
async fn main() {
    /* FIXME: duplicate from ../embedded-pt/ */
    info!("Initializing DECT MAC, trusting that Ariel OS did the basic setup");
    let (mut dect, mut dlc_tx, mut dlc_rx) = hophop::nrfxlib_mac::DectMac::create(());

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
        cipher_key: *b"JustAdefault!!!!",
        integrity_key: *b"JustAdefault!!!!",
    });
    let security = if let Some(hophop::association::Mode1Keys {
        integrity_key,
        cipher_key,
    }) = keys
    {
        dect_control::configure_params_security {
            mode: dect_mac::security_mode_values::_1,
            integrity_key: integrity_key,
            cipher_key: cipher_key,
        }
    } else {
        dect_control::configure_params_security {
            mode: dect_mac::security_mode_values::NONE,
            integrity_key: [0; _],
            cipher_key: [0; _],
        }
    };

    dect.control_configure(&mut dect_control::configure_params {
        // The maximum; we'll leave it to the network to lower the limits.
        // FIXME: take from hardware (although here it's the the standard's limit, but maybe we
        // have to adjust to a lower one from the hardware?)
        // FIXME why does this fail when we put in 23_DB?
        max_tx_power: dect_mac::tx_power_values::_10_DB,
        // FIXME: take from hardware
        max_mcs: dect_mac::max_mcs_values::_4,
        // FIXME: Decide (this is what the vendor examples default to)
        expected_mcs1_rx_rssi_level: -68,
        // FIXME: could we decide that later? (probably not: we come in with som expectation of
        // the network already)
        long_rd_id: our_long_id.into(),
        // FIXME: configure
        phy_band_group_index: dect_mac::band_group_index_values::IDX0,
        power_save: false,
        security,
        // FIXME: Decide (this is what the vendor uses in their examples)
        stats_averaging_length: 2,
    })
    .await;

    dect.control_functional_mode_set_activate().await;

    use nrf_modem::ErrorSource;
    let result = unsafe {
        dect_mac::rssi_scan(&mut dect_mac::rssi_scan_params {
            // -71dBm..-85dBm and 200ms: defaults from dects_hell
            channel_scan_length: 200, // ms
            threshold_min: -85,       // dBm
            threshold_max: -71,       // dBm
            band: dect_mac::band_values::PHY_BAND1,
            // or set to 0 to scan the whole band
            num_channels: 1,
            channel_list: [1665; _],
        })
    }
    .into_result();
    info!("Starting RSSI scan: {:?}", result);
    let result = dect
        .receive_singleton_event(hophop::nrfxlib_mac::SingletonEventSource::RssiScan)
        .await;
    info!("Done: {:?}", result);

    // Pick one by a process similar to how eMuhdra picks their secret keys
    let channel = AbsoluteChannel::new(1665).unwrap();
    info!("Continuing with channel {}", channel);

    // FIXME all parameters
    let mut association_config = dect_mac::association_config {
        // as used in samples/dect/dect_shell/prj.conf, for example
        max_num_neighbours: 50,
        // documented as not supported
        max_num_ft_neighbours: 0,
        neighbor_info_triggers:
            dect_mac::association_config_neighbor_info_triggers {
                inactivity_timer: 60_000, // 60s
            },
        default_tx_flow_config:
            // AIU those are for the 2 signalling flows and the 4 user flows
            [
            dect_mac::association_config__bindgen_ty_2 {
                priority: 0,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 0,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 3,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 4,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 5,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            dect_mac::association_config__bindgen_ty_2 {
                priority: 6,
                dlc_service_type: dect_dlc::service_type_values::_3,
                dlc_sdu_lifetime: dect_dlc::sdu_lifetime_values::_8_S,
            },
            ],
    };

    // TX power and RACH seem to be mandatory
    let mut config_flags = dect_mac::cluster_config_flags {
        _bitfield_1: nrfxlib_sys::__BindgenBitfieldUnit::new([0]),
        _bitfield_align_1: [0; 0],
    };
    config_flags.set_has_max_tx_power(1);
    config_flags.set_has_rach_config(1);

    // FIXME all parameters
    let mut cluster_config = dect_mac::cluster_config {
        // flags
        __bindgen_anon_1: dect_mac::cluster_config___bindgen_anon_1 { flags: config_flags },
        // don't be fooled: that's a nibble value as output by dect_utils_lib_dbm_to_phy_tx_power
        // (undocumented, but clear from dect_shell code) -- just so happens that the value 10
        // (0x0a) happens to also encode for 10dBm
        beacon_tx_power: 10,
        // not sure, but there are limited sets of allowed values
        count_to_trigger: dect_mac::count_to_trigger_values::_2,
        relative_quality: 0,
        min_quality: 0,
        // see beacon_tx_power
        cluster_max_tx_power: 10,
        cluster_beacon_period: dect_mac::cluster_beacon_period_values::_500_MS,
        cluster_channel: channel.into(),
        network_id: 0x87654321,
        rach_configuration: dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config {
            // probably disregarded due to bit mask
            policy: dect_mac::rach_config_policy_values::FILL,
            common: dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_common_config {
                response_window_length: 8,
                max_transmission_length: 8,
                cw_min_sig: 2,
                cw_max_sig: 7,
            },
            config: dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config {
                fill:  dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_fill {
                    percentage: 100,
                },
                // probably not used because policy is set to FILL
                periodic: dect_mac::cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_periodic {
                    period: 0,
                    length: 0,
                },
            }
        },
        triggers: dect_mac::cluster_config_cluster_info_triggers {
            busy_threshold: 20,
        },
        ipv6_config: dect_mac::ipv6_address_config_t {
            type_: dect_mac::ipv6_address_type_values::NONE,
            address: [0; 16],
        },
    };

    let result = unsafe {
        dect_mac::cluster_configure(&mut dect_mac::cluster_configure_params {
            // FIXME all parameters

            // as used anywhere in v3.4.0 code (but probably should be influenced by the RSSI
            // bit field)
            cluster_period_start_offset: 0,
            association_config: &mut association_config,
            cluster_config: &mut cluster_config,
        })
    }
    .into_result();
    info!("Setting cluster coniguration: {:?}", result);
    let result = dect
        .receive_singleton_event(hophop::nrfxlib_mac::SingletonEventSource::ClusterConfigure)
        .await;
    info!("Done: {:?}", result);

    loop {
        // Very quick and very dirty non-border router -- if a peer doesn't address something in
        // such a way that DLC can forward it, we have to
        let rx = &dlc_rx.dlc_data_rx().await;
        let data = rx.data();
        let Some(dest_addr) = data.get(24..40) else {
            warn!("DLC data too short to be IP");
            continue;
        };
        if &dest_addr[8..12] != &u32::from(our_long_id).to_be_bytes() {
            warn!("Received DLC from {} on {} but it's not to be forwarded here: dest addr is {}", rx.sender(), rx.flow_id(), Hex(dest_addr));
            continue;
        }

        // not checking 12..16 in peers list, just assuming that exists
        let Some(dest) = LongRdId::new(u32::from_be_bytes(dest_addr[12..16].try_into().unwrap())) else {
            warn!("Can not send to zero RD ID");
            continue;
        };

        // This is needless copying, for we could also make DlcDataRx destructible into such a Vec,
        // but we're .
        let mut data_out = heapless::Vec::<u8, 1501>::from_slice(&data).expect("Sizes align");

        let new_hoplimit = data_out[7].saturating_sub(1);
        if new_hoplimit == 0 {
            warn!("Discarding packet that underflew hop limit");
            continue;
        }
        data_out[7] = new_hoplimit;

        let result = dlc_tx.dlc_data_tx(rx.flow_id(), dest, &data_out).await;
        info!("Forwarded data to {:?} with hop limit {}: {:?}", dest, new_hoplimit, result);
    }
}
