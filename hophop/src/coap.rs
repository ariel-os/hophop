// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! CoAP resouorces for managing RDs.

use crate::association::Association;

/// Builds a CoAP resource for viewing an `Association` (based on a getter)
///
/// To retain flexibility of the implementation, use this with `.below()` rather than `.at()`.
pub fn association_status<G>(getter: G) -> impl coap_handler::Handler + coap_handler::Reporting
where
    G: FnMut() -> Option<Association>,
{
    use coap_handler_implementations::*;
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
