// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! CoAP resouorces for managing RDs.

use crate::association::{Association, PtConfiguration};

use coap_handler_implementations::*;

pub fn all<G>(
    getter: G,
    config: &ControllablePtConfiguration,
) -> impl coap_handler::Handler + coap_handler::Reporting
where
    G: FnMut() -> Option<Association>,
{
    // When doing remote config regularly, we'd recommend shorter paths, but now while we're mostly
    // using this from slipmux, it is practical to have something explicit -- and those are never
    // set in stone but always auto-discovered anyway.
    new_dispatcher()
        .below(&["assoc"], association_status(getter))
        .below(&["config"], config.coap_server())
}

/// Builds a CoAP resource for viewing an `Association` (based on a getter)
///
/// To retain flexibility of the implementation, use this with `.below()` rather than `.at()`.
pub fn association_status<G>(getter: G) -> impl coap_handler::Handler + coap_handler::Reporting
where
    G: FnMut() -> Option<Association>,
{
    use coap_message_utils::Error;

    struct ServeStatus<G>(G);

    impl<G: FnMut() -> Option<Association>> GetRenderable for ServeStatus<G> {
        type Get = Option<crate::association::Association>;

        fn get(&mut self) -> Result<Self::Get, Error> {
            Ok((self.0)())
        }
    }

    wkc::ConstantSingleRecordReport::new(
        TypeHandler::new_minicbor_2(with_get(ServeStatus(getter))),
        &[coap_handler::Attribute::ResourceType(
            "tag:ariel-os.org,2026:experimental-hophop",
        )],
    )
}

/// Place that contains a [`PtConfiguration`] that can be edited over CoAP, and produces an async
/// stream for implementing a [`crate::runner::MacController`].
///
/// # Caveats
///
/// The change notification around in `.next()` has the typical issues of multi-receiver
/// embassy_sync parts:
///
/// * It should only be used by one recipient (as the notification only goes to one reader)
/// * It is set to signalling already at the start, but that is only useful to a first consumer
///   (typically an impl of `MacController`).
pub struct ControllablePtConfiguration {
    // always only borrowed for short sync durations, for cloning.
    config: core::cell::RefCell<PtConfiguration>,
    changed: embassy_sync::signal::Signal<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        (),
    >,
}

impl ControllablePtConfiguration {
    pub fn new(initial_config: PtConfiguration) -> Self {
        let changed = embassy_sync::signal::Signal::new();
        // See Caveats in type docs
        changed.signal(());
        Self {
            config: core::cell::RefCell::new(initial_config),
            changed,
        }
    }

    pub fn get(&self) -> PtConfiguration {
        self.config.borrow().clone()
    }

    pub fn set(&self, config: PtConfiguration) {
        *self.config.borrow_mut() = config;
        self.changed.signal(());
    }

    pub async fn next(&self) -> PtConfiguration {
        self.changed.wait().await;
        self.get()
    }

    pub fn coap_server(&self) -> impl coap_handler::Handler + coap_handler::Reporting {
        use coap_message_utils::Error;

        struct ServeConfig<'a>(&'a ControllablePtConfiguration);

        impl<'a> GetRenderable for ServeConfig<'a> {
            type Get = PtConfiguration;

            fn get(&mut self) -> Result<Self::Get, Error> {
                Ok(self.0.get())
            }
        }

        impl<'a> PutRenderable for ServeConfig<'a> {
            type Put = PtConfiguration;

            fn put(&mut self, representation: &Self::Put) -> Result<(), Error> {
                self.0.changed.signal(());
                self.0.set(representation.clone());
                Ok(())
            }
        }

        wkc::ConstantSingleRecordReport::new(
            TypeHandler::new_minicbor_2(with_get_put(ServeConfig(self))),
            &[coap_handler::Attribute::ResourceType(
                "tag:ariel-os.org,2026:experimental-hophop-control",
            )],
        )
    }
}
