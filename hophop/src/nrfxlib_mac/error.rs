use nrf_modem::nrfxlib_sys;

// Having MacError used in a Result transparently only works if it can use the zero niche.
const _: () = assert!(nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_OK == 0);

/// Errors reported by the MAC layer
pub struct MacError(core::num::NonZero<u8>);

impl core::fmt::Debug for MacError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self.0.into() {
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_OK => unreachable!(),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_FAIL => "FAIL",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_INVALID_PARAM => "INVALID_PARAM",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NOT_ALLOWED => "NOT_ALLOWED",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_CONFIG => "NO_CONFIG",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_RD_NOT_FOUND => "RD_NOT_FOUND",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_TEMP_FAILURE => "TEMP_FAILURE",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RESOURCES => "NO_RESOURCES",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RESPONSE => "NO_RESPONSE",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NW_REJECT => "NW_REJECT",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_MEMORY => "NO_MEMORY",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RSSI_RESULTS => "NO_RSSI_RESULTS",
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_DLC_DISCARD_TIMER_EXPIRED => {
                "DLC_DISCARD_TIMER_EXPIRED"
            }
            _ => "(unknown error)",
        })
    }
}

impl defmt::Format for MacError {
    fn format(&self, fmt: defmt::Formatter) {
        match self.0.into() {
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_OK => unreachable!(),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_FAIL => defmt::write!(fmt, "FAIL"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_INVALID_PARAM => defmt::write!(fmt, "INVALID_PARAM"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NOT_ALLOWED => defmt::write!(fmt, "NOT_ALLOWED"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_CONFIG => defmt::write!(fmt, "NO_CONFIG"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_RD_NOT_FOUND => defmt::write!(fmt, "RD_NOT_FOUND"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_TEMP_FAILURE => defmt::write!(fmt, "TEMP_FAILURE"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RESOURCES => defmt::write!(fmt, "NO_RESOURCES"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RESPONSE => defmt::write!(fmt, "NO_RESPONSE"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NW_REJECT => defmt::write!(fmt, "NW_REJECT"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_MEMORY => defmt::write!(fmt, "NO_MEMORY"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_NO_RSSI_RESULTS => defmt::write!(fmt, "NO_RSSI_RESULTS"),
            nrfxlib_sys::nrf_modem_dect_mac_err_NRF_MODEM_DECT_MAC_STATUS_DLC_DISCARD_TIMER_EXPIRED => {
                defmt::write!(fmt, "DLC_DISCARD_TIMER_EXPIRED")
            }
            _ => defmt::write!(fmt, "(unknown error)"),
        }
    }
}

pub trait MacErrorExt {
    fn as_mac_status(self) -> Result<(), MacError>;
}

impl MacErrorExt for u8 {
    fn as_mac_status(self) -> Result<(), MacError> {
        if let Ok(nonzero) = self.try_into() {
            Err(MacError(nonzero))
        } else {
            Ok(())
        }
    }
}
