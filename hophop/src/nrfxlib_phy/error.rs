// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use nrf_modem::Error;
use nrfxlib_sys_pretty::dect_phy;

const _: () = const {
    assert!(
        dect_phy::err_values::SUCCESS == 0,
        "Constant for success switched and is now not aligned with Result niche optimization."
    );
};
#[derive(Debug, defmt::Format)]
pub struct PhyErr(core::num::NonZeroU16);
pub type PhyResult = Result<(), PhyErr>;

pub trait PhyResultExt {
    fn into_phy_result(self) -> PhyResult;
}

impl PhyResultExt for u16 {
    fn into_phy_result(self) -> PhyResult {
        match core::num::NonZeroU16::try_from(self) {
            Ok(v) => Err(PhyErr(v)),
            Err(_) => Ok(()),
        }
    }
}

/// Error type that encompasses both styles of errors returned by the libmodem APIs.
#[derive(Debug, defmt::Format)]
pub enum MixedError {
    General(Error),
    Phy(PhyErr),
    UsageError,
}

impl From<Error> for MixedError {
    fn from(input: Error) -> Self {
        MixedError::General(input)
    }
}

impl From<PhyErr> for MixedError {
    fn from(input: PhyErr) -> Self {
        MixedError::Phy(input)
    }
}
