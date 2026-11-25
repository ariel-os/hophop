// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Tools to manage peers when in the FT role.

use ts_103_636_utils::identifiers::LongRdId;

/// Reception point for peer status data: This is what the MAC callbacks in an FT can sink their
/// notification data into.
// FIXME: Currently, we're all passing that through & for interior mutability -- because our one
// impl does it, and as long as that is the case, we don't need a &'static mut T (just for T to be
// &U), which would just be one extra static to set up.
pub trait PeerStatus: Sync {
    /// States that a peer has become reachable, or is still reachable.
    // FIXME: There's a lot of more data that could get, but as long as the recipient ignores it...
    fn associated(&self, added_or_refreshed: LongRdId);
    /// States that a peer has become unreachable, or is still unreachable.
    // FIXME: Could also get some "reason", but see above
    fn removed(&self, removed: LongRdId);
}

mod impl_atomic {
    use super::*;

    use core::num::NonZero;
    use core::sync::atomic::Ordering::Relaxed;
    use core::sync::atomic::{AtomicBool, AtomicU32};

    /// A list of present and absent peers.
    ///
    /// This could probably be optimized to be faster on the happy path by doing something like a
    /// hash map, but we're only talking relaxed atomics and small N, so this should be good enough.
    ///
    /// Operations rely on every value being present in there only once. This is only upheld if the
    /// writings methods are not called at the same time. (Insertion could be made to complete in a
    /// single pass if this were not required, but it seems more practical this way).
    // FIXME: compare_exchange is overkill when we could also know that we're an exclusive writer,
    // and everyone else just reads. (But CAS shouldn't be too expensive given that we know that it
    // will work due to only having one writer).
    pub struct AtomicPresence<const N: usize> {
        entries: [AtomicU32; N],
        overflow: AtomicBool,
    }

    impl<const N: usize> AtomicPresence<N> {
        #[expect(
            clippy::new_without_default,
            reason = "practically only relevant in const context"
        )]
        pub const fn new() -> Self {
            Self {
                entries: [const { AtomicU32::new(0) }; _],
                overflow: AtomicBool::new(false),
            }
        }

        fn insert(&self, value: NonZero<u32>) {
            for i in self.entries.iter() {
                if i.load(Relaxed) == value.into() {
                    return;
                }
            }
            for i in self.entries.iter() {
                if i.compare_exchange(0, value.into(), Relaxed, Relaxed)
                    .is_ok()
                {
                    return;
                }
            }
            self.overflow.store(true, Relaxed);
        }

        fn remove(&self, value: NonZero<u32>) {
            for i in self.entries.iter() {
                if i.compare_exchange(value.into(), 0, Relaxed, Relaxed)
                    .is_ok()
                {
                    return;
                }
            }
        }

        /// Lists all peers that are or were in the list at some point between the iterator's
        /// creation and its exhaustion.
        ///
        /// Even if the internal invariant of every item being present only once is upheld (by
        /// modifications only happening from one thread), this function can return peers twice if
        /// kept around across change events. (To do that, it would need to commit to a view of the
        /// data, and then events queuing up might result in losing track of the overall state).
        // FIXME: As we can't expose "everything only ocne" property anyway, might simplify
        // intrnals.
        pub fn peers(&self) -> impl Iterator<Item = LongRdId> {
            self.entries
                .iter()
                .filter_map(|i| LongRdId::new(i.load(Relaxed)))
        }
    }

    impl<const N: usize> PeerStatus for AtomicPresence<N> {
        fn associated(&self, added_or_refreshed: LongRdId) {
            self.insert(added_or_refreshed.into());
        }

        fn removed(&self, removed: LongRdId) {
            self.remove(removed.into());
        }
    }
}

mod impl_defmt {
    use super::*;
    use defmt::*;

    /// Recipient of association data that sends it to the `defmt::info!` channel.
    pub struct LogToDefmt;

    impl PeerStatus for LogToDefmt {
        fn associated(&self, added_or_refreshed: LongRdId) {
            warn!("Associated with {}", added_or_refreshed)
        }

        fn removed(&self, removed: LongRdId) {
            warn!("Association removed: {}", removed);
        }
    }
}

pub use impl_atomic::AtomicPresence;
pub use impl_defmt::LogToDefmt;
