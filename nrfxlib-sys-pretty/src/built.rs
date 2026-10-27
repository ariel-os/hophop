// SPDX-FileCopyrightText: Derived file from the nrfxlib-sys inputs
// SPDX-License-Identifier: LicenseRef-Nordic-5-Clause
pub mod dect_clock {
#[doc(alias = "nrf_modem_dect_clock_sync_event_handler_set")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event_handler_set as sync_event_handler_set;
/// Possible values for [`sync_event_handler_t`]
pub mod sync_event_handler_t_values {
}
#[doc = r####"Application handler prototype for clock sync events."####]
///
/// See [`sync_event_handler_t_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_clock_sync_event_handler_t")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event_handler_t as sync_event_handler_t;
#[doc(alias = "nrf_modem_dect_clock_sync_enable")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_enable as sync_enable;
#[doc(alias = "nrf_modem_dect_clock_sync_disable")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_disable as sync_disable;
#[doc(alias = "nrf_modem_dect_clock_sync_get_pulse_info")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_get_pulse_info as sync_get_pulse_info;
/// Possible values for [`sync_pulse_direction`]
pub mod sync_pulse_direction_values {
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_pulse_direction_NRF_MODEM_DECT_CLOCK_SYNC_PULSE_INCOMING as INCOMING;
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_pulse_direction_NRF_MODEM_DECT_CLOCK_SYNC_PULSE_OUTGOING as OUTGOING;
}
#[doc = r####"Clock synchronization pulse direction."####]
///
/// See [`sync_pulse_direction_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_clock_sync_pulse_direction")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_pulse_direction as sync_pulse_direction;
#[doc(alias = "nrf_modem_dect_clock_sync_get_state")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_get_state as sync_get_state;
#[doc(alias = "nrf_modem_dect_clock_sync_enable_ref_output")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_enable_ref_output as sync_enable_ref_output;
#[doc(alias = "nrf_modem_dect_clock_sync_disable_ref_output")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_disable_ref_output as sync_disable_ref_output;
/// Possible values for [`sync_err`]
pub mod sync_err_values {
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_err_NRF_MODEM_DECT_CLOCK_SYNC_ERROR as ERROR;
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_err_NRF_MODEM_DECT_CLOCK_SYNC_NOT_ACTIVE as NOT_ACTIVE;
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_err_NRF_MODEM_DECT_CLOCK_SYNC_NO_ERROR as NO_ERROR;
}
#[doc = r####"DECT CS errors."####]
///
/// See [`sync_err_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_clock_sync_err")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_err as sync_err;
#[doc(alias = "nrf_modem_dect_clock_sync_enable_event")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_enable_event as sync_enable_event;
#[doc(alias = "nrf_modem_dect_clock_sync_pulse_info_event")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_pulse_info_event as sync_pulse_info_event;
#[doc(alias = "nrf_modem_dect_clock_sync_disable_event")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_disable_event as sync_disable_event;
#[doc(alias = "nrf_modem_dect_clock_sync_state_event")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_state_event as sync_state_event;
#[doc(alias = "nrf_modem_dect_clock_sync_enable_ref_output_event")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_enable_ref_output_event as sync_enable_ref_output_event;
#[doc(alias = "nrf_modem_dect_clock_sync_disable_ref_output_event")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_disable_ref_output_event as sync_disable_ref_output_event;
/// Possible values for [`sync_event_id`]
pub mod sync_event_id_values {
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event_id_NRF_MODEM_DECT_CLOCK_SYNC_EVT_DISABLE as DISABLE;
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event_id_NRF_MODEM_DECT_CLOCK_SYNC_EVT_DISABLE_REF_OUTPUT as DISABLE_REF_OUTPUT;
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event_id_NRF_MODEM_DECT_CLOCK_SYNC_EVT_ENABLE as ENABLE;
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event_id_NRF_MODEM_DECT_CLOCK_SYNC_EVT_ENABLE_REF_OUTPUT as ENABLE_REF_OUTPUT;
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event_id_NRF_MODEM_DECT_CLOCK_SYNC_EVT_PULSE_INFO as PULSE_INFO;
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event_id_NRF_MODEM_DECT_CLOCK_SYNC_EVT_STATE as STATE;
}
/// See [`sync_event_id_values`] for variants.
#[doc(alias = "nrf_modem_dect_clock_sync_event_id")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event_id as sync_event_id;
#[doc(alias = "nrf_modem_dect_clock_sync_event__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event__bindgen_ty_1 as sync_event___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_clock_sync_event")]
pub use nrfxlib_sys::nrf_modem_dect_clock_sync_event as sync_event;
}
pub mod dect_control {
#[doc(alias = "nrf_modem_dect_control_systemmode_set")]
pub use nrfxlib_sys::nrf_modem_dect_control_systemmode_set as systemmode_set;
/// Possible values for [`systemmode`]
pub mod systemmode_values {
pub use nrfxlib_sys::nrf_modem_dect_control_systemmode_NRF_MODEM_DECT_MODE_MAC as MAC;
pub use nrfxlib_sys::nrf_modem_dect_control_systemmode_NRF_MODEM_DECT_MODE_NONE as NONE;
}
#[doc = r####"Dect system modes."####]
///
/// See [`systemmode_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_control_systemmode")]
pub use nrfxlib_sys::nrf_modem_dect_control_systemmode as systemmode;
#[doc(alias = "nrf_modem_dect_control_functional_mode_set")]
pub use nrfxlib_sys::nrf_modem_dect_control_functional_mode_set as functional_mode_set;
/// Possible values for [`functional_mode`]
pub mod functional_mode_values {
pub use nrfxlib_sys::nrf_modem_dect_control_functional_mode_NRF_MODEM_DECT_CONTROL_FUNCTIONAL_MODE_ACTIVATE as ACTIVATE;
pub use nrfxlib_sys::nrf_modem_dect_control_functional_mode_NRF_MODEM_DECT_CONTROL_FUNCTIONAL_MODE_DEACTIVATE as DEACTIVATE;
}
#[doc = r####"Dect functional modes."####]
///
/// See [`functional_mode_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_control_functional_mode")]
pub use nrfxlib_sys::nrf_modem_dect_control_functional_mode as functional_mode;
#[doc(alias = "nrf_modem_dect_control_configure")]
pub use nrfxlib_sys::nrf_modem_dect_control_configure as configure;
#[doc(alias = "nrf_modem_dect_control_configure_params")]
pub use nrfxlib_sys::nrf_modem_dect_control_configure_params as configure_params;
#[doc(alias = "nrf_modem_dect_control_configure_params__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_control_configure_params__bindgen_ty_1 as configure_params_security;
}
pub mod dect_dlc {
#[doc(alias = "nrf_modem_dect_dlc_data_tx")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_data_tx as data_tx;
#[doc(alias = "nrf_modem_dect_dlc_data_tx_params")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_data_tx_params as data_tx_params;
#[doc(alias = "nrf_modem_dect_dlc_data_discard")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_data_discard as data_discard;
#[doc(alias = "nrf_modem_dect_dlc_data_discard_params")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_data_discard_params as data_discard_params;
/// Possible values for [`service_type`]
pub mod service_type_values {
pub use nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_0 as _0;
pub use nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_dlc_service_type_NRF_MODEM_DECT_DLC_SERVICE_TYPE_3 as _3;
}
#[doc = r####"Data link control (DLC) service types."####]
///
/// See [`service_type_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_dlc_service_type")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_service_type as service_type;
/// Possible values for [`sdu_lifetime`]
pub mod sdu_lifetime_values {
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_0_5_MS as _0_5_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_100_MS as _100_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_10_MS as _10_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_150_MS as _150_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_16_S as _16_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_1_5_S as _1_5_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_1_MS as _1_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_1_S as _1_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_200_MS as _200_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_20_MS as _20_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_250_MS as _250_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_2_5_S as _2_5_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_2_S as _2_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_300_MS as _300_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_30_MS as _30_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_32_S as _32_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_3_S as _3_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_40_MS as _40_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_4_S as _4_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_500_MS as _500_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_50_MS as _50_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_5_MS as _5_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_5_S as _5_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_60_MS as _60_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_60_S as _60_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_6_S as _6_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_70_MS as _70_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_750_MS as _750_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_80_MS as _80_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_8_S as _8_S;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_90_MS as _90_MS;
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime_NRF_MODEM_DECT_DLC_SDU_LIFETIME_INFINITY as INFINITY;
}
#[doc = r####"DLC SDU discard timer."####]
///
/// See [`sdu_lifetime_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_dlc_sdu_lifetime")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_sdu_lifetime as sdu_lifetime;
/// Possible values for [`flow_control_status`]
pub mod flow_control_status_values {
pub use nrfxlib_sys::nrf_modem_dect_dlc_flow_control_status_NRF_MODEM_DECT_DLC_FLOW_CTRL_STATUS_OFF as OFF;
pub use nrfxlib_sys::nrf_modem_dect_dlc_flow_control_status_NRF_MODEM_DECT_DLC_FLOW_CTRL_STATUS_ON as ON;
}
#[doc = r####"DLC flow control status."####]
///
/// See [`flow_control_status_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_dlc_flow_control_status")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_flow_control_status as flow_control_status;
#[doc(alias = "nrf_modem_dect_dlc_data_tx_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_data_tx_cb_params as data_tx_cb_params;
#[doc(alias = "nrf_modem_dect_dlc_data_discard_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_data_discard_cb_params as data_discard_cb_params;
#[doc(alias = "nrf_modem_dect_dlc_data_rx_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_data_rx_ntf_cb_params as data_rx_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_dlc_flow_control_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_dlc_flow_control_ntf_cb_params as flow_control_ntf_cb_params;
}
pub mod dect_mac {
#[doc(alias = "nrf_modem_dect_mac_callback_set")]
pub use nrfxlib_sys::nrf_modem_dect_mac_callback_set as callback_set;
#[doc(alias = "nrf_modem_dect_mac_op_callbacks")]
pub use nrfxlib_sys::nrf_modem_dect_mac_op_callbacks as op_callbacks;
#[doc(alias = "nrf_modem_dect_mac_ntf_callbacks")]
pub use nrfxlib_sys::nrf_modem_dect_mac_ntf_callbacks as ntf_callbacks;
#[doc(alias = "nrf_modem_dect_mac_association")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association as association;
#[doc(alias = "nrf_modem_dect_mac_association_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_params as association_params;
#[doc(alias = "nrf_modem_dect_mac_association_release")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_release as association_release;
#[doc(alias = "nrf_modem_dect_mac_association_release_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_release_params as association_release_params;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon_receive")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_receive as cluster_beacon_receive;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon_receive_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_receive_params as cluster_beacon_receive_params;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon_receive_stop")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_receive_stop as cluster_beacon_receive_stop;
#[doc(alias = "nrf_modem_dect_mac_cluster_configure")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_configure as cluster_configure;
#[doc(alias = "nrf_modem_dect_mac_cluster_configure_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_configure_params as cluster_configure_params;
#[doc(alias = "nrf_modem_dect_mac_network_beacon_configure")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_beacon_configure as network_beacon_configure;
#[doc(alias = "nrf_modem_dect_mac_network_beacon_configure_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_beacon_configure_params as network_beacon_configure_params;
#[doc(alias = "nrf_modem_dect_mac_network_scan")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_scan as network_scan;
#[doc(alias = "nrf_modem_dect_mac_network_scan_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_scan_params as network_scan_params;
#[doc(alias = "nrf_modem_dect_mac_network_scan_stop")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_scan_stop as network_scan_stop;
#[doc(alias = "nrf_modem_dect_mac_rssi_scan")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rssi_scan as rssi_scan;
#[doc(alias = "nrf_modem_dect_mac_rssi_scan_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_params as rssi_scan_params;
#[doc(alias = "nrf_modem_dect_mac_rssi_scan_stop")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_stop as rssi_scan_stop;
#[doc(alias = "nrf_modem_dect_mac_cluster_info")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_info as cluster_info;
#[doc(alias = "nrf_modem_dect_mac_neighbor_info")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbor_info as neighbor_info;
#[doc(alias = "nrf_modem_dect_mac_neighbor_info_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbor_info_params as neighbor_info_params;
#[doc(alias = "nrf_modem_dect_mac_neighbor_list")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbor_list as neighbor_list;
/// Possible values for [`err`]
pub mod err_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_DLC_DISCARD_TIMER_EXPIRED as DLC_DISCARD_TIMER_EXPIRED;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_FAIL as FAIL;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_INVALID_PARAM as INVALID_PARAM;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NOT_ALLOWED as NOT_ALLOWED;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_CONFIG as NO_CONFIG;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_MEMORY as NO_MEMORY;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RESOURCES as NO_RESOURCES;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RESPONSE as NO_RESPONSE;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RSSI_RESULTS as NO_RSSI_RESULTS;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NW_REJECT as NW_REJECT;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_OK as OK;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_RD_NOT_FOUND as RD_NOT_FOUND;
pub use nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_TEMP_FAILURE as TEMP_FAILURE;
}
#[doc = r####" nRF Modem DECT MAC status codes"####]
///
/// See [`err_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_err")]
pub use nrfxlib_sys::nrf_modem_dect_mac_err as err;
/// Possible values for [`association_indication_status`]
pub mod association_indication_status_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_association_indication_status_NRF_MODEM_DECT_MAC_ASSOCIATION_INDICATION_STATUS_LONG_ID_CONFLICT as LONG_ID_CONFLICT;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_indication_status_NRF_MODEM_DECT_MAC_ASSOCIATION_INDICATION_STATUS_MAX_NUM_NEIGHBOURS as MAX_NUM_NEIGHBOURS;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_indication_status_NRF_MODEM_DECT_MAC_ASSOCIATION_INDICATION_STATUS_NO_RESOURCES_FOR_RESPONSE as NO_RESOURCES_FOR_RESPONSE;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_indication_status_NRF_MODEM_DECT_MAC_ASSOCIATION_INDICATION_STATUS_RD_CAPA_MISMATCH as RD_CAPA_MISMATCH;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_indication_status_NRF_MODEM_DECT_MAC_ASSOCIATION_INDICATION_STATUS_SHORT_ID_CONFLICT as SHORT_ID_CONFLICT;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_indication_status_NRF_MODEM_DECT_MAC_ASSOCIATION_INDICATION_STATUS_SUCCESS as SUCCESS;
}
#[doc = r####" Association indication statuses. Association enables parent to route messages"####]
///
/// See [`association_indication_status_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_association_indication_status")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_indication_status as association_indication_status;
/// Possible values for [`association_reject_cause`]
pub mod association_reject_cause_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_CAUSE_CONFLICTED_SHORT_ID as CONFLICTED_SHORT_ID;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_CAUSE_NO_HW_CAPACITY as NO_HW_CAPACITY;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_CAUSE_NO_RADIO_CAPACITY as NO_RADIO_CAPACITY;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_CAUSE_OTHER_REASON as OTHER_REASON;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_CAUSE_SECURITY_NEEDED as SECURITY_NEEDED;
}
#[doc = r####"Association reject causes."####]
///
/// See [`association_reject_cause_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_association_reject_cause")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_cause as association_reject_cause;
/// Possible values for [`association_reject_time`]
pub mod association_reject_time_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_TIME_0S as _0S;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_TIME_10S as _10S;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_TIME_120S as _120S;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_TIME_180S as _180S;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_TIME_300S as _300S;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_TIME_30S as _30S;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_TIME_5S as _5S;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_TIME_600S as _600S;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time_NRF_MODEM_DECT_MAC_ASSOCIATION_REJECT_TIME_60S as _60S;
}
#[doc = r####" Association reject times. Application must wait the time before re-attempt"####]
///
/// See [`association_reject_time_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_association_reject_time")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_reject_time as association_reject_time;
/// Possible values for [`association_setup_cause`]
pub mod association_setup_cause_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_association_setup_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_SETUP_CAUSE_AFTER_ERROR as AFTER_ERROR;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_setup_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_SETUP_CAUSE_CHANGE_OPER_CHANNEL as CHANGE_OPER_CHANNEL;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_setup_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_SETUP_CAUSE_CHANGE_OPER_MODE as CHANGE_OPER_MODE;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_setup_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_SETUP_CAUSE_INIT as INIT;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_setup_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_SETUP_CAUSE_MOBILITY as MOBILITY;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_setup_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_SETUP_CAUSE_NEW_SET_OF_FLOWS as NEW_SET_OF_FLOWS;
pub use nrfxlib_sys::nrf_modem_dect_mac_association_setup_cause_NRF_MODEM_DECT_MAC_ASSOCIATION_SETUP_CAUSE_OTHER_REASON as OTHER_REASON;
}
#[doc = r####"Association setup causes."####]
///
/// See [`association_setup_cause_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_association_setup_cause")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_setup_cause as association_setup_cause;
/// Possible values for [`cluster_beacon_period`]
pub mod cluster_beacon_period_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_1000_MS as _1000_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_100_MS as _100_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_10_MS as _10_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_1500_MS as _1500_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_16000_MS as _16000_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_2000_MS as _2000_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_32000_MS as _32000_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_4000_MS as _4000_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_500_MS as _500_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_50_MS as _50_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period_NRF_MODEM_DECT_MAC_CLUSTER_BEACON_PERIOD_8000_MS as _8000_MS;
}
#[doc = r####"Cluster beacon sending periods."####]
///
/// See [`cluster_beacon_period_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon_period")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_period as cluster_beacon_period;
/// Possible values for [`cluster_max_tx_power`]
pub mod cluster_max_tx_power_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_0_DB as _0_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_10_DB as _10_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_13_DB as _13_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_16_DB as _16_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_19_DB as _19_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_21_DB as _21_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_23_DB as _23_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_4_DB as _4_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_7_DB as _7_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_NEGATIVE_12_DB as NEGATIVE_12_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_NEGATIVE_4_DB as NEGATIVE_4_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power_NRF_MODEM_DECT_MAC_CLUSTER_MAX_TX_POWER_NEGATIVE_8_DB as NEGATIVE_8_DB;
}
#[doc = r####"Cluster maximum TX powers."####]
///
/// See [`cluster_max_tx_power_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_cluster_max_tx_power")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_max_tx_power as cluster_max_tx_power;
/// Possible values for [`communication_method`]
pub mod communication_method_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_communication_method_NRF_MODEM_DECT_MAC_COMMUNICATION_METHOD_PAGING as PAGING;
pub use nrfxlib_sys::nrf_modem_dect_mac_communication_method_NRF_MODEM_DECT_MAC_COMMUNICATION_METHOD_POLLING as POLLING;
pub use nrfxlib_sys::nrf_modem_dect_mac_communication_method_NRF_MODEM_DECT_MAC_COMMUNICATION_METHOD_RACH as RACH;
}
#[doc = r####"Communication methods."####]
///
/// See [`communication_method_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_communication_method")]
pub use nrfxlib_sys::nrf_modem_dect_mac_communication_method as communication_method;
/// Possible values for [`count_to_trigger`]
pub mod count_to_trigger_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_128 as _128;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_16 as _16;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_24 as _24;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_256 as _256;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_3 as _3;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_32 as _32;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_4 as _4;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_40 as _40;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_48 as _48;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_5 as _5;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_6 as _6;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_64 as _64;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_7 as _7;
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger_NRF_MODEM_DECT_MAC_COUNT_TO_TRIGGER_8 as _8;
}
#[doc = r####" Counts to trigger. Count to trigger can be used in decision to"####]
///
/// See [`count_to_trigger_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_count_to_trigger")]
pub use nrfxlib_sys::nrf_modem_dect_mac_count_to_trigger as count_to_trigger;
/// Possible values for [`harq_delay_subslots`]
pub mod harq_delay_subslots_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_subslots_NRF_MODEM_DECT_MAC_HARQ_DELAY_SUBSLOTS_0 as _0;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_subslots_NRF_MODEM_DECT_MAC_HARQ_DELAY_SUBSLOTS_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_subslots_NRF_MODEM_DECT_MAC_HARQ_DELAY_SUBSLOTS_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_subslots_NRF_MODEM_DECT_MAC_HARQ_DELAY_SUBSLOTS_3 as _3;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_subslots_NRF_MODEM_DECT_MAC_HARQ_DELAY_SUBSLOTS_4 as _4;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_subslots_NRF_MODEM_DECT_MAC_HARQ_DELAY_SUBSLOTS_5 as _5;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_subslots_NRF_MODEM_DECT_MAC_HARQ_DELAY_SUBSLOTS_6 as _6;
}
#[doc = r####"HARQ delay subslots."####]
///
/// See [`harq_delay_subslots_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_harq_delay_subslots")]
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_subslots as harq_delay_subslots;
/// Possible values for [`harq_delay_time`]
pub mod harq_delay_time_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_100MS as _100MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_105US as _105US;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_10MS as _10MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_120MS as _120MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_140MS as _140MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_160MS as _160MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_180MS as _180MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_1MS as _1MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_200MS as _200MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_200US as _200US;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_20MS as _20MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_240MS as _240MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_280MS as _280MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_2MS as _2MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_30MS as _30MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_320MS as _320MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_360MS as _360MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_400MS as _400MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_400US as _400US;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_40MS as _40MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_450MS as _450MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_4MS as _4MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_500MS as _500MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_50MS as _50MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_60MS as _60MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_6MS as _6MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_70MS as _70MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_800US as _800US;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_80MS as _80MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_8MS as _8MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time_NRF_MODEM_DECT_MAC_HARQ_DELAY_TIME_90MS as _90MS;
}
#[doc = r####"HARQ delay times."####]
///
/// See [`harq_delay_time_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_harq_delay_time")]
pub use nrfxlib_sys::nrf_modem_dect_mac_harq_delay_time as harq_delay_time;
/// Possible values for [`ie_type`]
pub mod ie_type_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_CONFIGURATION_REQUEST as CONFIGURATION_REQUEST;
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_LOAD_INFO as LOAD_INFO;
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_MEASUREMENT_REPORT as MEASUREMENT_REPORT;
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_NEIGHBOURING as NEIGHBOURING;
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_RADIO_DEVICE_STATUS as RADIO_DEVICE_STATUS;
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_RANDOM_ACCESS_RESOURCE as RANDOM_ACCESS_RESOURCE;
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_RD_CAPABILITY as RD_CAPABILITY;
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_RESOURCE_ALLOCATION as RESOURCE_ALLOCATION;
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_ROUTE_INFO as ROUTE_INFO;
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type_NRF_MODEM_DECT_MAC_IE_TYPE_SECURITY_INFO as SECURITY_INFO;
}
#[doc = r####"MAC information element types"####]
///
/// See [`ie_type_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_ie_type")]
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_type as ie_type;
/// Possible values for [`max_mcs`]
pub mod max_mcs_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_0 as _0;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_10 as _10;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_11 as _11;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_3 as _3;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_4 as _4;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_5 as _5;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_6 as _6;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_7 as _7;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_8 as _8;
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs_NRF_MODEM_DECT_MAC_MAX_MCS_9 as _9;
}
#[doc = r####"Max modulation and coding schemes."####]
///
/// See [`max_mcs_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_max_mcs")]
pub use nrfxlib_sys::nrf_modem_dect_mac_max_mcs as max_mcs;
/// Possible values for [`band_group_index`]
pub mod band_group_index_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_band_group_index_NRF_MODEM_DECT_MAC_PHY_BAND_GROUP_IDX0 as IDX0;
pub use nrfxlib_sys::nrf_modem_dect_mac_band_group_index_NRF_MODEM_DECT_MAC_PHY_BAND_GROUP_IDX1 as IDX1;
}
#[doc = r####"Band group index."####]
///
/// See [`band_group_index_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_band_group_index")]
pub use nrfxlib_sys::nrf_modem_dect_mac_band_group_index as band_group_index;
/// Possible values for [`band`]
pub mod band_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_band_NRF_MODEM_DECT_MAC_PHY_BAND1 as PHY_BAND1;
pub use nrfxlib_sys::nrf_modem_dect_mac_band_NRF_MODEM_DECT_MAC_PHY_BAND2 as PHY_BAND2;
pub use nrfxlib_sys::nrf_modem_dect_mac_band_NRF_MODEM_DECT_MAC_PHY_BAND22 as PHY_BAND22;
pub use nrfxlib_sys::nrf_modem_dect_mac_band_NRF_MODEM_DECT_MAC_PHY_BAND4 as PHY_BAND4;
pub use nrfxlib_sys::nrf_modem_dect_mac_band_NRF_MODEM_DECT_MAC_PHY_BAND9 as PHY_BAND9;
}
#[doc = r####" Bands. Countries allow different bands. Bands frequencies are defined"####]
///
/// See [`band_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_band")]
pub use nrfxlib_sys::nrf_modem_dect_mac_band as band;
/// Possible values for [`nw_beacon_period`]
pub mod nw_beacon_period_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_beacon_period_NRF_MODEM_DECT_MAC_NW_BEACON_PERIOD_1000_MS as _1000_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_beacon_period_NRF_MODEM_DECT_MAC_NW_BEACON_PERIOD_100_MS as _100_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_beacon_period_NRF_MODEM_DECT_MAC_NW_BEACON_PERIOD_1500_MS as _1500_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_beacon_period_NRF_MODEM_DECT_MAC_NW_BEACON_PERIOD_2000_MS as _2000_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_beacon_period_NRF_MODEM_DECT_MAC_NW_BEACON_PERIOD_4000_MS as _4000_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_beacon_period_NRF_MODEM_DECT_MAC_NW_BEACON_PERIOD_500_MS as _500_MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_beacon_period_NRF_MODEM_DECT_MAC_NW_BEACON_PERIOD_50_MS as _50_MS;
}
#[doc = r####"Network beacon sending periods."####]
///
/// See [`nw_beacon_period_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_nw_beacon_period")]
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_beacon_period as nw_beacon_period;
/// Possible values for [`nw_id_filter_mode`]
pub mod nw_id_filter_mode_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_id_filter_mode_NRF_MODEM_DECT_MAC_NW_ID_FILTER_MODE_24MSB as _24MSB;
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_id_filter_mode_NRF_MODEM_DECT_MAC_NW_ID_FILTER_MODE_32BIT as _32BIT;
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_id_filter_mode_NRF_MODEM_DECT_MAC_NW_ID_FILTER_MODE_NONE as NONE;
}
#[doc = r####"Network ID filter modes."####]
///
/// See [`nw_id_filter_mode_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_nw_id_filter_mode")]
pub use nrfxlib_sys::nrf_modem_dect_mac_nw_id_filter_mode as nw_id_filter_mode;
/// Possible values for [`ra_repeat_type`]
pub mod ra_repeat_type_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_ra_repeat_type_NRF_MODEM_DECT_MAC_RA_REPEAT_TYPE_FRAMES as FRAMES;
pub use nrfxlib_sys::nrf_modem_dect_mac_ra_repeat_type_NRF_MODEM_DECT_MAC_RA_REPEAT_TYPE_SINGLE as SINGLE;
pub use nrfxlib_sys::nrf_modem_dect_mac_ra_repeat_type_NRF_MODEM_DECT_MAC_RA_REPEAT_TYPE_SUBSLOTS as SUBSLOTS;
}
#[doc = r####"Random Access repeat types."####]
///
/// See [`ra_repeat_type_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_ra_repeat_type")]
pub use nrfxlib_sys::nrf_modem_dect_mac_ra_repeat_type as ra_repeat_type;
/// Possible values for [`rd_capability_beta`]
pub mod rd_capability_beta_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_beta_NRF_MODEM_DECT_MAC_RD_CAPABILITY_BETA_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_beta_NRF_MODEM_DECT_MAC_RD_CAPABILITY_BETA_12 as _12;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_beta_NRF_MODEM_DECT_MAC_RD_CAPABILITY_BETA_16 as _16;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_beta_NRF_MODEM_DECT_MAC_RD_CAPABILITY_BETA_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_beta_NRF_MODEM_DECT_MAC_RD_CAPABILITY_BETA_4 as _4;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_beta_NRF_MODEM_DECT_MAC_RD_CAPABILITY_BETA_8 as _8;
}
#[doc = r####" Radio Device capability fourier transform scaling factors."####]
///
/// See [`rd_capability_beta_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_beta")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_beta as rd_capability_beta;
/// Possible values for [`rd_capability_dlc_service_type`]
pub mod rd_capability_dlc_service_type_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_dlc_service_type_NRF_MODEM_DECT_MAC_RD_CAPABILITY_DLC_SERVICE_TYPE_0 as _0;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_dlc_service_type_NRF_MODEM_DECT_MAC_RD_CAPABILITY_DLC_SERVICE_TYPE_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_dlc_service_type_NRF_MODEM_DECT_MAC_RD_CAPABILITY_DLC_SERVICE_TYPE_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_dlc_service_type_NRF_MODEM_DECT_MAC_RD_CAPABILITY_DLC_SERVICE_TYPE_3_2_1 as _3_2_1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_dlc_service_type_NRF_MODEM_DECT_MAC_RD_CAPABILITY_DLC_SERVICE_TYPE_ALL as ALL;
}
#[doc = r####"Radio device capability DLC service types."####]
///
/// See [`rd_capability_dlc_service_type_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_dlc_service_type")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_dlc_service_type as rd_capability_dlc_service_type;
/// Possible values for [`rd_capability_num_harq`]
pub mod rd_capability_num_harq_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_num_harq_NRF_MODEM_DECT_MAC_RD_CAPABILITY_NUM_HARQ_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_num_harq_NRF_MODEM_DECT_MAC_RD_CAPABILITY_NUM_HARQ_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_num_harq_NRF_MODEM_DECT_MAC_RD_CAPABILITY_NUM_HARQ_4 as _4;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_num_harq_NRF_MODEM_DECT_MAC_RD_CAPABILITY_NUM_HARQ_8 as _8;
}
#[doc = r####"Radio Device capability number of HARQs."####]
///
/// See [`rd_capability_num_harq_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_num_harq")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_num_harq as rd_capability_num_harq;
/// Possible values for [`rd_capability_max_nss`]
pub mod rd_capability_max_nss_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_max_nss_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MAX_NSS_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_max_nss_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MAX_NSS_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_max_nss_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MAX_NSS_4 as _4;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_max_nss_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MAX_NSS_8 as _8;
}
#[doc = r####"Max number of spatial stream transmission capabilities."####]
///
/// See [`rd_capability_max_nss_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_max_nss")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_max_nss as rd_capability_max_nss;
/// Possible values for [`rd_capability_muu`]
pub mod rd_capability_muu_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_muu_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MUU_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_muu_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MUU_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_muu_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MUU_4 as _4;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_muu_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MUU_8 as _8;
}
#[doc = r####"Radio Device capability subcarrier scaling factors."####]
///
/// See [`rd_capability_muu_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_muu")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_muu as rd_capability_muu;
/// Possible values for [`rd_capability_operating_mode`]
pub mod rd_capability_operating_mode_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_operating_mode_NRF_MODEM_DECT_MAC_RD_CAPABILITY_OPERATING_MODE_FT_ONLY as FT_ONLY;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_operating_mode_NRF_MODEM_DECT_MAC_RD_CAPABILITY_OPERATING_MODE_PT_FT as PT_FT;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_operating_mode_NRF_MODEM_DECT_MAC_RD_CAPABILITY_OPERATING_MODE_PT_ONLY as PT_ONLY;
}
#[doc = r####" Radio Device capability operating modes."####]
///
/// See [`rd_capability_operating_mode_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_operating_mode")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_operating_mode as rd_capability_operating_mode;
/// Possible values for [`rd_capability_power_class`]
pub mod rd_capability_power_class_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_power_class_NRF_MODEM_DECT_MAC_RD_CAPABILITY_POWER_CLASS_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_power_class_NRF_MODEM_DECT_MAC_RD_CAPABILITY_POWER_CLASS_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_power_class_NRF_MODEM_DECT_MAC_RD_CAPABILITY_POWER_CLASS_3 as _3;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_power_class_NRF_MODEM_DECT_MAC_RD_CAPABILITY_POWER_CLASS_4 as _4;
}
#[doc = r####"Radio Device power classes."####]
///
/// See [`rd_capability_power_class_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_power_class")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_power_class as rd_capability_power_class;
/// Possible values for [`rd_capability_release`]
pub mod rd_capability_release_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_release_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RELEASE_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_release_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RELEASE_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_release_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RELEASE_3 as _3;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_release_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RELEASE_4 as _4;
}
#[doc = r####"Supported specification version"####]
///
/// See [`rd_capability_release_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_release")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_release as rd_capability_release;
/// Possible values for [`rd_capability_rx_for_tx_div`]
pub mod rd_capability_rx_for_tx_div_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_for_tx_div_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_FOR_TX_DIV_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_for_tx_div_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_FOR_TX_DIV_2 as _2;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_for_tx_div_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_FOR_TX_DIV_4 as _4;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_for_tx_div_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_FOR_TX_DIV_8 as _8;
}
#[doc = r####"Reception capability of the TX diversity transmission."####]
///
/// See [`rd_capability_rx_for_tx_div_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_rx_for_tx_div")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_for_tx_div as rd_capability_rx_for_tx_div;
/// Possible values for [`rd_capability_rx_gain`]
pub mod rd_capability_rx_gain_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_GAIN_0_DB as _0_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_GAIN_2_DB as _2_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_GAIN_4_DB as _4_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_GAIN_6_DB as _6_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_GAIN_NEGATIVE_10_DB as NEGATIVE_10_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_GAIN_NEGATIVE_2_DB as NEGATIVE_2_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_GAIN_NEGATIVE_4_DB as NEGATIVE_4_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_GAIN_NEGATIVE_6_DB as NEGATIVE_6_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain_NRF_MODEM_DECT_MAC_RD_CAPABILITY_RX_GAIN_NEGATIVE_8_DB as NEGATIVE_8_DB;
}
#[doc = r####"Radio Device capability RX gain."####]
///
/// See [`rd_capability_rx_gain_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_rx_gain")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_rx_gain as rd_capability_rx_gain;
/// Possible values for [`rd_capability_soft_buffer_size`]
pub mod rd_capability_soft_buffer_size_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size_NRF_MODEM_DECT_MAC_RD_CAPABILITY_SOFT_BUFFER_SIZE_1024K as _1024K;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size_NRF_MODEM_DECT_MAC_RD_CAPABILITY_SOFT_BUFFER_SIZE_128K as _128K;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size_NRF_MODEM_DECT_MAC_RD_CAPABILITY_SOFT_BUFFER_SIZE_16K as _16K;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size_NRF_MODEM_DECT_MAC_RD_CAPABILITY_SOFT_BUFFER_SIZE_2048K as _2048K;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size_NRF_MODEM_DECT_MAC_RD_CAPABILITY_SOFT_BUFFER_SIZE_256K as _256K;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size_NRF_MODEM_DECT_MAC_RD_CAPABILITY_SOFT_BUFFER_SIZE_25K as _25K;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size_NRF_MODEM_DECT_MAC_RD_CAPABILITY_SOFT_BUFFER_SIZE_32K as _32K;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size_NRF_MODEM_DECT_MAC_RD_CAPABILITY_SOFT_BUFFER_SIZE_512K as _512K;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size_NRF_MODEM_DECT_MAC_RD_CAPABILITY_SOFT_BUFFER_SIZE_64K as _64K;
}
#[doc = r####"Radio Device capability soft buffer size."####]
///
/// See [`rd_capability_soft_buffer_size_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_soft_buffer_size")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_soft_buffer_size as rd_capability_soft_buffer_size;
/// Possible values for [`rd_status`]
pub mod rd_status_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_NRF_MODEM_DECT_MAC_RD_STATUS_MEMORY_FULL as MEMORY_FULL;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_NRF_MODEM_DECT_MAC_RD_STATUS_NORMAL_OPERATION as NORMAL_OPERATION;
}
#[doc = r####"Radio Device status."####]
///
/// See [`rd_status_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_status")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status as rd_status;
/// Possible values for [`rd_status_duration`]
pub mod rd_status_duration_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_1000MS as _1000MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_100MS as _100MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_1500MS as _1500MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_2000MS as _2000MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_200MS as _200MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_3000MS as _3000MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_4000MS as _4000MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_400MS as _400MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_50MS as _50MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_600MS as _600MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_800MS as _800MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration_NRF_MODEM_DECT_MAC_RD_STATUS_DURATION_UNKNOWN as UNKNOWN;
}
#[doc = r####"Radio Device status duration."####]
///
/// See [`rd_status_duration_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_status_duration")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_status_duration as rd_status_duration;
/// Possible values for [`release_cause`]
pub mod release_cause_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_DECT_MAC_RELEASE_CAUSE_RACH_RESOURCE_FAILURE as RACH_RESOURCE_FAILURE;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_BAD_RADIO_QUALITY as BAD_RADIO_QUALITY;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_CONNECTION_TERMINATION as CONNECTION_TERMINATION;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_INCOMPATIBLE_CONFIGURATION as INCOMPATIBLE_CONFIGURATION;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_INSUFFICIENT_HW_RESOURCES as INSUFFICIENT_HW_RESOURCES;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_INSUFFICIENT_RADIO_RESOURCES as INSUFFICIENT_RADIO_RESOURCES;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_LONG_INACTIVITY as LONG_INACTIVITY;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_MOBILITY as MOBILITY;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_OTHER_ERROR as OTHER_ERROR;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_OTHER_REASON as OTHER_REASON;
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause_NRF_MODEM_DECT_MAC_RELEASE_CAUSE_SECURITY_ERROR as SECURITY_ERROR;
}
#[doc = r####"Association release cause."####]
///
/// See [`release_cause_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_release_cause")]
pub use nrfxlib_sys::nrf_modem_dect_mac_release_cause as release_cause;
/// Possible values for [`resource_allocation_repeat_type`]
pub mod resource_allocation_repeat_type_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_repeat_type_NRF_MODEM_DECT_MAC_RESOURCE_ALLOCATION_REPEAT_TYPE_FRAMES as FRAMES;
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_repeat_type_NRF_MODEM_DECT_MAC_RESOURCE_ALLOCATION_REPEAT_TYPE_FRAMES_GROUP as FRAMES_GROUP;
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_repeat_type_NRF_MODEM_DECT_MAC_RESOURCE_ALLOCATION_REPEAT_TYPE_SINGLE as SINGLE;
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_repeat_type_NRF_MODEM_DECT_MAC_RESOURCE_ALLOCATION_REPEAT_TYPE_SUBSLOTS as SUBSLOTS;
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_repeat_type_NRF_MODEM_DECT_MAC_RESOURCE_ALLOCATION_REPEAT_TYPE_SUBSLOTS_GROUP as SUBSLOTS_GROUP;
}
#[doc = r####"Resource allocation repeat type."####]
///
/// See [`resource_allocation_repeat_type_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_resource_allocation_repeat_type")]
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_repeat_type as resource_allocation_repeat_type;
/// Possible values for [`resource_allocation_type`]
pub mod resource_allocation_type_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_type_NRF_MODEM_DECT_MAC_RESOURCE_ALLOCATION_TYPE_BOTH_ALLOCATIONS as BOTH_ALLOCATIONS;
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_type_NRF_MODEM_DECT_MAC_RESOURCE_ALLOCATION_TYPE_DOWNLINK_ALLOCATION as DOWNLINK_ALLOCATION;
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_type_NRF_MODEM_DECT_MAC_RESOURCE_ALLOCATION_TYPE_RELEASE_ALL as RELEASE_ALL;
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_type_NRF_MODEM_DECT_MAC_RESOURCE_ALLOCATION_TYPE_UPLINK_ALLOCATION as UPLINK_ALLOCATION;
}
#[doc = r####"Allocation type."####]
///
/// See [`resource_allocation_type_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_resource_allocation_type")]
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_type as resource_allocation_type;
/// Possible values for [`sched_resource_fail_time`]
pub mod sched_resource_fail_time_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_1000MS as _1000MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_100MS as _100MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_1500MS as _1500MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_200MS as _200MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_20MS as _20MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_3000MS as _3000MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_4000MS as _4000MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_5000MS as _5000MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_500MS as _500MS;
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time_NRF_MODEM_DECT_MAC_SCHED_RESOURCE_FAIL_TIME_50MS as _50MS;
}
#[doc = r####"Schedule resource fail time."####]
///
/// See [`sched_resource_fail_time_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_sched_resource_fail_time")]
pub use nrfxlib_sys::nrf_modem_dect_mac_sched_resource_fail_time as sched_resource_fail_time;
/// Possible values for [`tx_power`]
pub mod tx_power_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_0_DB as _0_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_10_DB as _10_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_13_DB as _13_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_16_DB as _16_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_19_DB as _19_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_21_DB as _21_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_23_DB as _23_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_4_DB as _4_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_7_DB as _7_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_NEGATIVE_12_DB as NEGATIVE_12_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_NEGATIVE_16_DB as NEGATIVE_16_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_NEGATIVE_20_DB as NEGATIVE_20_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_NEGATIVE_30_DB as NEGATIVE_30_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_NEGATIVE_40_DB as NEGATIVE_40_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_NEGATIVE_4_DB as NEGATIVE_4_DB;
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power_NRF_MODEM_DECT_MAC_TX_POWER_NEGATIVE_8_DB as NEGATIVE_8_DB;
}
#[doc = r####"TX power."####]
///
/// See [`tx_power_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_tx_power")]
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_power as tx_power;
/// Possible values for [`quality_threshold`]
pub mod quality_threshold_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_quality_threshold_NRF_MODEM_DECT_MAC_QUALITY_THRESHOLD_0 as _0;
pub use nrfxlib_sys::nrf_modem_dect_mac_quality_threshold_NRF_MODEM_DECT_MAC_QUALITY_THRESHOLD_3 as _3;
pub use nrfxlib_sys::nrf_modem_dect_mac_quality_threshold_NRF_MODEM_DECT_MAC_QUALITY_THRESHOLD_6 as _6;
pub use nrfxlib_sys::nrf_modem_dect_mac_quality_threshold_NRF_MODEM_DECT_MAC_QUALITY_THRESHOLD_9 as _9;
}
#[doc = r####"Quality threshold."####]
///
/// See [`quality_threshold_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_quality_threshold")]
pub use nrfxlib_sys::nrf_modem_dect_mac_quality_threshold as quality_threshold;
/// Possible values for [`rach_config_policy`]
pub mod rach_config_policy_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rach_config_policy_NRF_MODEM_DECT_MAC_RACH_CONFIG_POLICY_FILL as FILL;
pub use nrfxlib_sys::nrf_modem_dect_mac_rach_config_policy_NRF_MODEM_DECT_MAC_RACH_CONFIG_POLICY_PERIODIC as PERIODIC;
}
#[doc = r####"Random access resource configuration policy."####]
///
/// See [`rach_config_policy_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rach_config_policy")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rach_config_policy as rach_config_policy;
/// Possible values for [`rd_capability_mac_security`]
pub mod rd_capability_mac_security_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_mac_security_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MAC_SECURITY_MODE_1 as MODE_1;
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_mac_security_NRF_MODEM_DECT_MAC_RD_CAPABILITY_MAC_SECURITY_NONE as NONE;
}
#[doc = r####"Radio Device capability security modes."####]
///
/// See [`rd_capability_mac_security_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_rd_capability_mac_security")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_mac_security as rd_capability_mac_security;
/// Possible values for [`security_mode`]
pub mod security_mode_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_security_mode_NRF_MODEM_DECT_MAC_SECURITY_MODE_1 as _1;
pub use nrfxlib_sys::nrf_modem_dect_mac_security_mode_NRF_MODEM_DECT_MAC_SECURITY_MODE_NONE as NONE;
}
#[doc = r####"Security mode."####]
///
/// See [`security_mode_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_security_mode")]
pub use nrfxlib_sys::nrf_modem_dect_mac_security_mode as security_mode;
/// Possible values for [`ipv6_address_type`]
pub mod ipv6_address_type_values {
pub use nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_FULL as FULL;
pub use nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_NONE as NONE;
pub use nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type_NRF_MODEM_DECT_MAC_IPV6_ADDRESS_TYPE_PREFIX as PREFIX;
}
#[doc = r####"MAC IPv6 address type"####]
///
/// See [`ipv6_address_type_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_mac_ipv6_address_type")]
pub use nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_type as ipv6_address_type;
#[doc(alias = "nrf_modem_dect_mac_route_info_ie")]
pub use nrfxlib_sys::nrf_modem_dect_mac_route_info_ie as route_info_ie;
#[doc(alias = "nrf_modem_dect_mac_ie_resource_alloc")]
pub use nrfxlib_sys::nrf_modem_dect_mac_ie_resource_alloc as ie_resource_alloc;
#[doc(alias = "nrf_modem_dect_mac_resource_allocation_ie__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_ie__bindgen_ty_1 as resource_allocation_ie___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_resource_allocation_ie")]
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_ie as resource_allocation_ie;
#[doc(alias = "nrf_modem_dect_mac_resource_allocation_ie__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_resource_allocation_ie__bindgen_ty_1__bindgen_ty_1 as resource_allocation_ie_flags;
#[doc(alias = "nrf_modem_dect_mac_random_access_resource_ie__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_random_access_resource_ie__bindgen_ty_1 as random_access_resource_ie___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_random_access_resource_ie")]
pub use nrfxlib_sys::nrf_modem_dect_mac_random_access_resource_ie as random_access_resource_ie;
#[doc(alias = "nrf_modem_dect_mac_random_access_resource_ie__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_random_access_resource_ie__bindgen_ty_1__bindgen_ty_1 as random_access_resource_ie_flags;
#[doc(alias = "nrf_modem_dect_mac_rd_capa_phy_capabilities")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capa_phy_capabilities as rd_capa_phy_capabilities;
#[doc(alias = "nrf_modem_dect_mac_rd_capability_ie__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_ie__bindgen_ty_1 as rd_capability_ie__bindgen_ty_1;
#[doc(alias = "nrf_modem_dect_mac_rd_capability_ie")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rd_capability_ie as rd_capability_ie;
#[doc(alias = "nrf_modem_dect_mac_neighbouring_ie__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbouring_ie__bindgen_ty_1 as neighbouring_ie___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_neighbouring_ie")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbouring_ie as neighbouring_ie;
#[doc(alias = "nrf_modem_dect_mac_neighbouring_ie__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbouring_ie__bindgen_ty_1__bindgen_ty_1 as neighbouring_ie_flags;
#[doc(alias = "nrf_modem_dect_mac_load_info_ie__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_load_info_ie__bindgen_ty_1 as load_info_ie___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_load_info_ie")]
pub use nrfxlib_sys::nrf_modem_dect_mac_load_info_ie as load_info_ie;
#[doc(alias = "nrf_modem_dect_mac_load_info_ie__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_load_info_ie__bindgen_ty_1__bindgen_ty_1 as load_info_ie_flags;
#[doc(alias = "nrf_modem_dect_mac_measurement_report_ie__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_measurement_report_ie__bindgen_ty_1 as measurement_report_ie___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_measurement_report_ie")]
pub use nrfxlib_sys::nrf_modem_dect_mac_measurement_report_ie as measurement_report_ie;
#[doc(alias = "nrf_modem_dect_mac_measurement_report_ie__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_measurement_report_ie__bindgen_ty_1__bindgen_ty_1 as measurement_report_ie_flags;
#[doc(alias = "nrf_modem_dect_mac_radio_device_status_ie")]
pub use nrfxlib_sys::nrf_modem_dect_mac_radio_device_status_ie as radio_device_status_ie;
#[doc(alias = "nrf_modem_dect_mac_ie__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_ie__bindgen_ty_1 as ie_ie;
#[doc(alias = "nrf_modem_dect_mac_ie")]
pub use nrfxlib_sys::nrf_modem_dect_mac_ie as ie;
#[doc(alias = "nrf_modem_dect_mac_association_response__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_response__bindgen_ty_1 as association_response___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_association_response")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_response as association_response;
#[doc(alias = "nrf_modem_dect_mac_association_response__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_response__bindgen_ty_1__bindgen_ty_1 as association_response_flags;
#[doc(alias = "nrf_modem_dect_mac_rssi_result")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rssi_result as rssi_result;
#[doc(alias = "nrf_modem_dect_mac_control_functional_mode_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_control_functional_mode_cb_params as control_functional_mode_cb_params;
#[doc(alias = "nrf_modem_dect_mac_control_configure_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_control_configure_cb_params as control_configure_cb_params;
#[doc(alias = "nrf_modem_dect_mac_control_systemmode_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_control_systemmode_cb_params as control_systemmode_cb_params;
#[doc(alias = "nrf_modem_dect_mac_rx_signal_info_t")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rx_signal_info_t as rx_signal_info_t;
#[doc(alias = "nrf_modem_dect_mac_ipv6_address_config_t")]
pub use nrfxlib_sys::nrf_modem_dect_mac_ipv6_address_config_t as ipv6_address_config_t;
#[doc(alias = "nrf_modem_dect_mac_association_cb_params__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_cb_params__bindgen_ty_1 as association_cb_params___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_association_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_cb_params as association_cb_params;
#[doc(alias = "nrf_modem_dect_mac_association_cb_params__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_cb_params__bindgen_ty_1__bindgen_ty_1 as association_cb_params_flags;
#[doc(alias = "nrf_modem_dect_mac_association_release_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_release_cb_params as association_release_cb_params;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon_receive_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_receive_cb_params as cluster_beacon_receive_cb_params;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon_receive_stop_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_receive_stop_cb_params as cluster_beacon_receive_stop_cb_params;
#[doc(alias = "nrf_modem_dect_mac_cluster_configure_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_configure_cb_params as cluster_configure_cb_params;
#[doc(alias = "nrf_modem_dect_mac_cluster_info_cb_params_nrf_modem_dect_mac_cluster_status_info")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_info_cb_params_nrf_modem_dect_mac_cluster_status_info as cluster_info_cb_params_nrf_modem_dect_mac_cluster_status_info;
#[doc(alias = "nrf_modem_dect_mac_cluster_info_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_info_cb_params as cluster_info_cb_params;
#[doc(alias = "nrf_modem_dect_mac_neighbor_info_cb_params_nrf_modem_dect_mac_neighbor_status_info")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbor_info_cb_params_nrf_modem_dect_mac_neighbor_status_info as neighbor_info_cb_params_nrf_modem_dect_mac_neighbor_status_info;
#[doc(alias = "nrf_modem_dect_mac_neighbor_info_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbor_info_cb_params as neighbor_info_cb_params;
#[doc(alias = "nrf_modem_dect_mac_neighbor_list_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbor_list_cb_params as neighbor_list_cb_params;
#[doc(alias = "nrf_modem_dect_mac_network_beacon_configure_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_beacon_configure_cb_params as network_beacon_configure_cb_params;
#[doc(alias = "nrf_modem_dect_mac_network_scan_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_scan_cb_params as network_scan_cb_params;
#[doc(alias = "nrf_modem_dect_mac_network_scan_stop_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_scan_stop_cb_params as network_scan_stop_cb_params;
#[doc(alias = "nrf_modem_dect_mac_rssi_scan_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_cb_params as rssi_scan_cb_params;
#[doc(alias = "nrf_modem_dect_mac_rssi_scan_stop_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_stop_cb_params as rssi_scan_stop_cb_params;
#[doc(alias = "nrf_modem_dect_mac_network_beacon__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_beacon__bindgen_ty_1 as network_beacon___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_network_beacon")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_beacon as network_beacon;
#[doc(alias = "nrf_modem_dect_mac_network_beacon__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_beacon__bindgen_ty_1__bindgen_ty_1 as network_beacon_flags;
#[doc(alias = "nrf_modem_dect_mac_association_request__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_request__bindgen_ty_1 as association_request___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_association_request")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_request as association_request;
#[doc(alias = "nrf_modem_dect_mac_association_request__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_request__bindgen_ty_1__bindgen_ty_1 as association_request_flags;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon__bindgen_ty_1 as cluster_beacon___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon as cluster_beacon;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon__bindgen_ty_1__bindgen_ty_1 as cluster_beacon_flags;
#[doc(alias = "nrf_modem_dect_mac_rssi_scan_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_rssi_scan_ntf_cb_params as rssi_scan_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_ntf_cb_params as cluster_beacon_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_network_beacon_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_network_beacon_ntf_cb_params as network_beacon_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_association_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_ntf_cb_params as association_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_association_release_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_release_ntf_cb_params as association_release_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_cluster_ch_load_change_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_ch_load_change_ntf_cb_params as cluster_ch_load_change_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_neighbor_inactivity_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbor_inactivity_ntf_cb_params as neighbor_inactivity_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_neighbor_paging_failure_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_neighbor_paging_failure_ntf_cb_params as neighbor_paging_failure_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_phy_band_info")]
pub use nrfxlib_sys::nrf_modem_dect_mac_phy_band_info as phy_band_info;
#[doc(alias = "nrf_modem_dect_mac_capability_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_capability_ntf_cb_params as capability_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon_rx_failure_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_rx_failure_ntf_cb_params as cluster_beacon_rx_failure_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_ipv6_config_update_ntf_cb_params")]
pub use nrfxlib_sys::nrf_modem_dect_mac_ipv6_config_update_ntf_cb_params as ipv6_config_update_ntf_cb_params;
#[doc(alias = "nrf_modem_dect_mac_tx_flow_config")]
pub use nrfxlib_sys::nrf_modem_dect_mac_tx_flow_config as tx_flow_config;
#[doc(alias = "nrf_modem_dect_mac_parent_info_triggers")]
pub use nrfxlib_sys::nrf_modem_dect_mac_parent_info_triggers as parent_info_triggers;
#[doc(alias = "nrf_modem_dect_mac_cluster_beacon_config")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_beacon_config as cluster_beacon_config;
#[doc(alias = "nrf_modem_dect_mac_association_config__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_config__bindgen_ty_1 as association_config_neighbor_info_triggers;
#[doc(alias = "nrf_modem_dect_mac_association_config__bindgen_ty_2")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_config__bindgen_ty_2 as association_config__bindgen_ty_2;
#[doc(alias = "nrf_modem_dect_mac_association_config")]
pub use nrfxlib_sys::nrf_modem_dect_mac_association_config as association_config;
#[doc(alias = "nrf_modem_dect_mac_cluster_config__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_config__bindgen_ty_1 as cluster_config___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config as cluster_config_nrf_modem_dect_mac_cluster_rach_config;
#[doc(alias = "nrf_modem_dect_mac_cluster_config_cluster_info_triggers")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_config_cluster_info_triggers as cluster_config_cluster_info_triggers;
#[doc(alias = "nrf_modem_dect_mac_cluster_config")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_config as cluster_config;
#[doc(alias = "nrf_modem_dect_mac_cluster_config__bindgen_ty_1__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_config__bindgen_ty_1__bindgen_ty_1 as cluster_config_flags;
#[doc(alias = "nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_common_config")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_common_config as cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_common_config;
#[doc(alias = "nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config as cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config;
#[doc(alias = "nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_fill")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_fill as cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_fill;
#[doc(alias = "nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_periodic")]
pub use nrfxlib_sys::nrf_modem_dect_mac_cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_periodic as cluster_config_nrf_modem_dect_mac_cluster_rach_config_nrf_modem_dect_mac_rach_config_nrf_modem_dect_mac_rach_config_periodic;
}
pub mod dect_phy {
#[doc(alias = "nrf_modem_dect_phy_event_handler_set")]
pub use nrfxlib_sys::nrf_modem_dect_phy_event_handler_set as event_handler_set;
/// Possible values for [`event_handler_t`]
pub mod event_handler_t_values {
}
#[doc = r####"Application handler prototype for PHY events."####]
///
/// See [`event_handler_t_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_phy_event_handler_t")]
pub use nrfxlib_sys::nrf_modem_dect_phy_event_handler_t as event_handler_t;
#[doc(alias = "nrf_modem_dect_phy_capability_get")]
pub use nrfxlib_sys::nrf_modem_dect_phy_capability_get as capability_get;
#[doc(alias = "nrf_modem_dect_phy_init")]
pub use nrfxlib_sys::nrf_modem_dect_phy_init as init;
#[doc(alias = "nrf_modem_dect_phy_deinit")]
pub use nrfxlib_sys::nrf_modem_dect_phy_deinit as deinit;
#[doc(alias = "nrf_modem_dect_phy_activate")]
pub use nrfxlib_sys::nrf_modem_dect_phy_activate as activate;
/// Possible values for [`radio_mode`]
pub mod radio_mode_values {
pub use nrfxlib_sys::nrf_modem_dect_phy_radio_mode_NRF_MODEM_DECT_PHY_RADIO_MODE_COUNT as COUNT;
pub use nrfxlib_sys::nrf_modem_dect_phy_radio_mode_NRF_MODEM_DECT_PHY_RADIO_MODE_LOW_LATENCY as LOW_LATENCY;
pub use nrfxlib_sys::nrf_modem_dect_phy_radio_mode_NRF_MODEM_DECT_PHY_RADIO_MODE_LOW_LATENCY_WITH_STANDBY as LOW_LATENCY_WITH_STANDBY;
pub use nrfxlib_sys::nrf_modem_dect_phy_radio_mode_NRF_MODEM_DECT_PHY_RADIO_MODE_NON_LBT_WITH_STANDBY as NON_LBT_WITH_STANDBY;
}
#[doc = r####" Radio modes."####]
///
/// See [`radio_mode_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_phy_radio_mode")]
pub use nrfxlib_sys::nrf_modem_dect_phy_radio_mode as radio_mode;
#[doc(alias = "nrf_modem_dect_phy_deactivate")]
pub use nrfxlib_sys::nrf_modem_dect_phy_deactivate as deactivate;
#[doc(alias = "nrf_modem_dect_phy_rx")]
pub use nrfxlib_sys::nrf_modem_dect_phy_rx as rx;
#[doc(alias = "nrf_modem_dect_phy_rx_params")]
pub use nrfxlib_sys::nrf_modem_dect_phy_rx_params as rx_params;
#[doc(alias = "nrf_modem_dect_phy_tx")]
pub use nrfxlib_sys::nrf_modem_dect_phy_tx as tx;
#[doc(alias = "nrf_modem_dect_phy_tx_params")]
pub use nrfxlib_sys::nrf_modem_dect_phy_tx_params as tx_params;
#[doc(alias = "nrf_modem_dect_phy_tx_harq")]
pub use nrfxlib_sys::nrf_modem_dect_phy_tx_harq as tx_harq;
#[doc(alias = "nrf_modem_dect_phy_tx_rx")]
pub use nrfxlib_sys::nrf_modem_dect_phy_tx_rx as tx_rx;
#[doc(alias = "nrf_modem_dect_phy_tx_rx_params")]
pub use nrfxlib_sys::nrf_modem_dect_phy_tx_rx_params as tx_rx_params;
#[doc(alias = "nrf_modem_dect_phy_rssi")]
pub use nrfxlib_sys::nrf_modem_dect_phy_rssi as rssi;
#[doc(alias = "nrf_modem_dect_phy_rssi_params")]
pub use nrfxlib_sys::nrf_modem_dect_phy_rssi_params as rssi_params;
#[doc(alias = "nrf_modem_dect_phy_cancel")]
pub use nrfxlib_sys::nrf_modem_dect_phy_cancel as cancel;
#[doc(alias = "nrf_modem_dect_phy_configure")]
pub use nrfxlib_sys::nrf_modem_dect_phy_configure as configure;
#[doc(alias = "nrf_modem_dect_phy_config_params")]
pub use nrfxlib_sys::nrf_modem_dect_phy_config_params as config_params;
#[doc(alias = "nrf_modem_dect_phy_radio_config")]
pub use nrfxlib_sys::nrf_modem_dect_phy_radio_config as radio_config;
#[doc(alias = "nrf_modem_dect_phy_radio_config_params")]
pub use nrfxlib_sys::nrf_modem_dect_phy_radio_config_params as radio_config_params;
#[doc(alias = "nrf_modem_dect_phy_link_config")]
pub use nrfxlib_sys::nrf_modem_dect_phy_link_config as link_config;
#[doc(alias = "nrf_modem_dect_phy_link_config_params")]
pub use nrfxlib_sys::nrf_modem_dect_phy_link_config_params as link_config_params;
#[doc(alias = "nrf_modem_dect_phy_band_get")]
pub use nrfxlib_sys::nrf_modem_dect_phy_band_get as band_get;
#[doc(alias = "nrf_modem_dect_phy_latency_get")]
pub use nrfxlib_sys::nrf_modem_dect_phy_latency_get as latency_get;
#[doc(alias = "nrf_modem_dect_phy_time_get")]
pub use nrfxlib_sys::nrf_modem_dect_phy_time_get as time_get;
#[doc(alias = "nrf_modem_dect_phy_test_rf_tx_cw_control")]
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_control as test_rf_tx_cw_control;
/// Possible values for [`test_rf_tx_cw_oper`]
pub mod test_rf_tx_cw_oper_values {
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_oper_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_OPER_OFF as OFF;
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_oper_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_OPER_ON as ON;
}
#[doc = r####"Test RF TX CW operation codes."####]
///
/// See [`test_rf_tx_cw_oper_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_phy_test_rf_tx_cw_oper")]
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_oper as test_rf_tx_cw_oper;
#[doc(alias = "nrf_modem_dect_phy_stf_cover_seq_control")]
pub use nrfxlib_sys::nrf_modem_dect_phy_stf_cover_seq_control as stf_cover_seq_control;
/// Possible values for [`err`]
pub mod err_values {
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_COMBINED_OP_FAILED as ERR_COMBINED_OP_FAILED;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_HANDLE_IN_USE as ERR_HANDLE_IN_USE;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_INVALID_DURATION as ERR_INVALID_DURATION;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_INVALID_NETWORK_ID as ERR_INVALID_NETWORK_ID;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_INVALID_PARAMETER as ERR_INVALID_PARAMETER;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_INVALID_PHY_HEADER as ERR_INVALID_PHY_HEADER;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_INVALID_START_TIME as ERR_INVALID_START_TIME;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_LBT_CHANNEL_BUSY as ERR_LBT_CHANNEL_BUSY;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_LBT_START_TIME_LATE as ERR_LBT_START_TIME_LATE;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_MODEM_ERROR as ERR_MODEM_ERROR;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_MODEM_ERROR_RF_STATE as ERR_MODEM_ERROR_RF_STATE;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_NOT_ALLOWED as ERR_NOT_ALLOWED;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_NOT_FOUND as ERR_NOT_FOUND;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_NO_MEMORY as ERR_NO_MEMORY;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_NO_ONGOING_HARQ_RX as ERR_NO_ONGOING_HARQ_RX;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_OP_CANCELED as ERR_OP_CANCELED;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_OP_SCHEDULING_CONFLICT as ERR_OP_SCHEDULING_CONFLICT;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_OP_START_TIME_LATE as ERR_OP_START_TIME_LATE;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_OP_TIMEOUT as ERR_OP_TIMEOUT;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_PARAMETER_UNAVAILABLE as ERR_PARAMETER_UNAVAILABLE;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_PAYLOAD_UNAVAILABLE as ERR_PAYLOAD_UNAVAILABLE;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_PROD_LOCK as ERR_PROD_LOCK;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_RADIO_MODE_CONFLICT as ERR_RADIO_MODE_CONFLICT;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_RF_START_TIME_LATE as ERR_RF_START_TIME_LATE;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_TEMP_HIGH as ERR_TEMP_HIGH;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_TX_POWER_OVER_MAX_LIMIT as ERR_TX_POWER_OVER_MAX_LIMIT;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_UNSUPPORTED_CARRIER as ERR_UNSUPPORTED_CARRIER;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_UNSUPPORTED_DATA_SIZE as ERR_UNSUPPORTED_DATA_SIZE;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_ERR_UNSUPPORTED_OP as ERR_UNSUPPORTED_OP;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_OK_WITH_HARQ_RESET as OK_WITH_HARQ_RESET;
pub use nrfxlib_sys::nrf_modem_dect_phy_err_NRF_MODEM_DECT_PHY_SUCCESS as SUCCESS;
}
#[doc = r####"DECT PHY errors."####]
///
/// See [`err_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_phy_err")]
pub use nrfxlib_sys::nrf_modem_dect_phy_err as err;
#[doc(alias = "nrf_modem_dect_phy_capability__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_phy_capability__bindgen_ty_1 as capability__bindgen_ty_1;
#[doc(alias = "nrf_modem_dect_phy_capability")]
pub use nrfxlib_sys::nrf_modem_dect_phy_capability as capability;
#[doc(alias = "nrf_modem_dect_phy_band")]
pub use nrfxlib_sys::nrf_modem_dect_phy_band as band;
/// Possible values for [`rx_mode`]
pub mod rx_mode_values {
pub use nrfxlib_sys::nrf_modem_dect_phy_rx_mode_NRF_MODEM_DECT_PHY_RX_MODE_CONTINUOUS as CONTINUOUS;
pub use nrfxlib_sys::nrf_modem_dect_phy_rx_mode_NRF_MODEM_DECT_PHY_RX_MODE_SEMICONTINUOUS as SEMICONTINUOUS;
pub use nrfxlib_sys::nrf_modem_dect_phy_rx_mode_NRF_MODEM_DECT_PHY_RX_MODE_SINGLE_SHOT as SINGLE_SHOT;
}
#[doc = r####"Reception mode."####]
///
/// See [`rx_mode_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_phy_rx_mode")]
pub use nrfxlib_sys::nrf_modem_dect_phy_rx_mode as rx_mode;
/// Possible values for [`rssi_interval`]
pub mod rssi_interval_values {
pub use nrfxlib_sys::nrf_modem_dect_phy_rssi_interval_NRF_MODEM_DECT_PHY_RSSI_INTERVAL_12_SLOTS as _12_SLOTS;
pub use nrfxlib_sys::nrf_modem_dect_phy_rssi_interval_NRF_MODEM_DECT_PHY_RSSI_INTERVAL_24_SLOTS as _24_SLOTS;
pub use nrfxlib_sys::nrf_modem_dect_phy_rssi_interval_NRF_MODEM_DECT_PHY_RSSI_INTERVAL_OFF as OFF;
}
#[doc = r####"RSSI reporting interval."####]
///
/// See [`rssi_interval_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_phy_rssi_interval")]
pub use nrfxlib_sys::nrf_modem_dect_phy_rssi_interval as rssi_interval;
#[doc(alias = "nrf_modem_dect_phy_hdr_type_1")]
pub use nrfxlib_sys::nrf_modem_dect_phy_hdr_type_1 as hdr_type_1;
#[doc(alias = "nrf_modem_dect_phy_feedback__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_phy_feedback__bindgen_ty_1 as feedback_format1;
#[doc(alias = "nrf_modem_dect_phy_feedback__bindgen_ty_2")]
pub use nrfxlib_sys::nrf_modem_dect_phy_feedback__bindgen_ty_2 as feedback_format2;
#[doc(alias = "nrf_modem_dect_phy_feedback__bindgen_ty_3")]
pub use nrfxlib_sys::nrf_modem_dect_phy_feedback__bindgen_ty_3 as feedback_format3;
#[doc(alias = "nrf_modem_dect_phy_feedback__bindgen_ty_4")]
pub use nrfxlib_sys::nrf_modem_dect_phy_feedback__bindgen_ty_4 as feedback_format4;
#[doc(alias = "nrf_modem_dect_phy_feedback__bindgen_ty_5")]
pub use nrfxlib_sys::nrf_modem_dect_phy_feedback__bindgen_ty_5 as feedback_format5;
#[doc(alias = "nrf_modem_dect_phy_feedback__bindgen_ty_6")]
pub use nrfxlib_sys::nrf_modem_dect_phy_feedback__bindgen_ty_6 as feedback_format6;
#[doc(alias = "nrf_modem_dect_phy_feedback")]
pub use nrfxlib_sys::nrf_modem_dect_phy_feedback as feedback;
#[doc(alias = "nrf_modem_dect_phy_hdr_type_2")]
pub use nrfxlib_sys::nrf_modem_dect_phy_hdr_type_2 as hdr_type_2;
#[doc(alias = "nrf_modem_dect_phy_hdr")]
pub use nrfxlib_sys::nrf_modem_dect_phy_hdr as hdr;
/// Possible values for [`hdr_status`]
pub mod hdr_status_values {
pub use nrfxlib_sys::nrf_modem_dect_phy_hdr_status_NRF_MODEM_DECT_PHY_HDR_STATUS_INVALID as INVALID;
pub use nrfxlib_sys::nrf_modem_dect_phy_hdr_status_NRF_MODEM_DECT_PHY_HDR_STATUS_VALID as VALID;
pub use nrfxlib_sys::nrf_modem_dect_phy_hdr_status_NRF_MODEM_DECT_PHY_HDR_STATUS_VALID_RX_END as VALID_RX_END;
}
#[doc = r####"Physical header status."####]
///
/// See [`hdr_status_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_phy_hdr_status")]
pub use nrfxlib_sys::nrf_modem_dect_phy_hdr_status as hdr_status;
#[doc(alias = "nrf_modem_dect_phy_link_id")]
pub use nrfxlib_sys::nrf_modem_dect_phy_link_id as link_id;
#[doc(alias = "nrf_modem_dect_phy_rx_filter")]
pub use nrfxlib_sys::nrf_modem_dect_phy_rx_filter as rx_filter;
#[doc(alias = "nrf_modem_dect_phy_latency_info__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_phy_latency_info__bindgen_ty_1 as latency_info__bindgen_ty_1;
#[doc(alias = "nrf_modem_dect_phy_latency_info__bindgen_ty_2")]
pub use nrfxlib_sys::nrf_modem_dect_phy_latency_info__bindgen_ty_2 as latency_info_operation;
#[doc(alias = "nrf_modem_dect_phy_latency_info__bindgen_ty_3")]
pub use nrfxlib_sys::nrf_modem_dect_phy_latency_info__bindgen_ty_3 as latency_info_stack;
#[doc(alias = "nrf_modem_dect_phy_latency_info")]
pub use nrfxlib_sys::nrf_modem_dect_phy_latency_info as latency_info;
#[doc(alias = "nrf_modem_dect_phy_latency_info__bindgen_ty_2__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_phy_latency_info__bindgen_ty_2__bindgen_ty_1 as latency_info_receive;
#[doc(alias = "nrf_modem_dect_phy_latency_info__bindgen_ty_2__bindgen_ty_2")]
pub use nrfxlib_sys::nrf_modem_dect_phy_latency_info__bindgen_ty_2__bindgen_ty_2 as latency_info_transmit;
#[doc(alias = "nrf_modem_dect_phy_init_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_init_event as init_event;
#[doc(alias = "nrf_modem_dect_phy_op_complete_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_op_complete_event as op_complete_event;
#[doc(alias = "nrf_modem_dect_phy_rssi_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_rssi_event as rssi_event;
#[doc(alias = "nrf_modem_dect_phy_cancel_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_cancel_event as cancel_event;
#[doc(alias = "nrf_modem_dect_phy_pcc_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_pcc_event as pcc_event;
#[doc(alias = "nrf_modem_dect_phy_pcc_crc_failure_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_pcc_crc_failure_event as pcc_crc_failure_event;
#[doc(alias = "nrf_modem_dect_phy_pdc_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_pdc_event as pdc_event;
#[doc(alias = "nrf_modem_dect_phy_pdc_crc_failure_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_pdc_crc_failure_event as pdc_crc_failure_event;
#[doc(alias = "nrf_modem_dect_phy_configure_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_configure_event as configure_event;
#[doc(alias = "nrf_modem_dect_phy_radio_config_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_radio_config_event as radio_config_event;
#[doc(alias = "nrf_modem_dect_phy_link_config_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_link_config_event as link_config_event;
#[doc(alias = "nrf_modem_dect_phy_time_get_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_time_get_event as time_get_event;
#[doc(alias = "nrf_modem_dect_phy_capability_get_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_capability_get_event as capability_get_event;
#[doc(alias = "nrf_modem_dect_phy_band_get_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_band_get_event as band_get_event;
/// Possible values for [`test_rf_tx_cw_err`]
pub mod test_rf_tx_cw_err_values {
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_err_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_ERR_BUFFER_RESERVE_FAILED as BUFFER_RESERVE_FAILED;
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_err_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_ERR_DSP_CONFIG_FAILURE as DSP_CONFIG_FAILURE;
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_err_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_ERR_ERROR as ERROR;
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_err_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_ERR_INVALID_OPERATION as INVALID_OPERATION;
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_err_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_ERR_MSG_SEND_FAILED as MSG_SEND_FAILED;
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_err_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_ERR_NO_ERROR as NO_ERROR;
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_err_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_ERR_TX_ALREADY_OFF as TX_ALREADY_OFF;
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_err_NRF_MODEM_DECT_PHY_TEST_RF_TX_CW_ERR_TX_ALREADY_ON as TX_ALREADY_ON;
}
#[doc = r####"Test RF TX CW operation status codes."####]
///
/// See [`test_rf_tx_cw_err_values`] for variants.
///
/// Full original documentation:
///
#[doc(alias = "nrf_modem_dect_phy_test_rf_tx_cw_err")]
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_err as test_rf_tx_cw_err;
#[doc(alias = "nrf_modem_dect_phy_test_rf_tx_cw_control_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_test_rf_tx_cw_control_event as test_rf_tx_cw_control_event;
#[doc(alias = "nrf_modem_dect_phy_stf_control_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_stf_control_event as stf_control_event;
#[doc(alias = "nrf_modem_dect_phy_deinit_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_deinit_event as deinit_event;
#[doc(alias = "nrf_modem_dect_phy_activate_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_activate_event as activate_event;
#[doc(alias = "nrf_modem_dect_phy_deactivate_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_deactivate_event as deactivate_event;
#[doc(alias = "nrf_modem_dect_phy_latency_info_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_latency_info_event as latency_info_event;
#[doc(alias = "nrf_modem_dect_phy_reject_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_reject_event as reject_event;
/// Possible values for [`event_id`]
pub mod event_id_values {
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_ACTIVATE as ACTIVATE;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_BANDS as BANDS;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_CANCELED as CANCELED;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_CAPABILITY as CAPABILITY;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_COMPLETED as COMPLETED;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_CONFIGURE as CONFIGURE;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_DEACTIVATE as DEACTIVATE;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_DEINIT as DEINIT;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_INIT as INIT;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_LATENCY as LATENCY;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_LINK_CONFIG as LINK_CONFIG;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_PCC as PCC;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_PCC_ERROR as PCC_ERROR;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_PDC as PDC;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_PDC_ERROR as PDC_ERROR;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_RADIO_CONFIG as RADIO_CONFIG;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_REJECT as REJECT;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_RSSI as RSSI;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_STF_CONFIG as STF_CONFIG;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_TEST_RF_TX_CW_CONTROL_CONFIG as TEST_RF_TX_CW_CONTROL_CONFIG;
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id_NRF_MODEM_DECT_PHY_EVT_TIME as TIME;
}
/// See [`event_id_values`] for variants.
#[doc(alias = "nrf_modem_dect_phy_event_id")]
pub use nrfxlib_sys::nrf_modem_dect_phy_event_id as event_id;
#[doc(alias = "nrf_modem_dect_phy_event__bindgen_ty_1")]
pub use nrfxlib_sys::nrf_modem_dect_phy_event__bindgen_ty_1 as event___bindgen_anon_1;
#[doc(alias = "nrf_modem_dect_phy_event")]
pub use nrfxlib_sys::nrf_modem_dect_phy_event as event;
}
