// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Global queues (implemented in channels) for transporting information back from interrupts into
//! the application.

use super::error::MacError;
use super::{ClusterBeacon, DlcDataRx};

// This should evolve a bit
//
// So far, this is only used for the lockstepping parts, when exactly one thing is possible and
// exactly one thing happens.
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
/// Events which are only generated while the [`DectMac`] is in a `&mut self` operation that has no
/// other events. Typically, this is all kinds of global mode changes.
///
/// While we could unwrap() those already in the ISR in many cases rather than in the processing
/// task, this does carry a 1-byte return value because in *some* cases we need it (eg.
/// for [`DectMac::dlc_data_tx()`]), and then it's simpler to do it everywhere consistently.
///
/// Length is 2 to allow use for scan and scan_stop: If a scan is stopped before time, confirmation
/// of the end of scan and of the stop command will both arrive. They will be indistinguishable,
/// and that doesn't matter.
pub(crate) static SINGLETON_EVENTS: Channel<CriticalSectionRawMutex, Result<(), MacError>, 2> =
    Channel::new();

/// Events during a scan (maybe also during associated operation).
///
/// FIXME: How should overflow be indicated?
pub(crate) static BEACON_EVENTS: Channel<
    CriticalSectionRawMutex,
    ClusterBeacon,
    // FIXME: What's a good number?
    2,
> = Channel::new();

// FIXME: We definitely want to use something smarter; ideally ownership of net pool entries when
// we better understand who allocates what.
pub(crate) static PACKETS: Channel<
    CriticalSectionRawMutex,
    // If we make it bigger, we might easily exceed the ISR stack
    DlcDataRx,
    1,
> = Channel::new();
