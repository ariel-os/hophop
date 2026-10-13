// SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
// SPDX-License-Identifier: MIT OR Apache-2.0
//! The embedded-nal adaption layer copied from https://github.com/ariel-os/ariel-os/pull/1613

// FIXME copy-pasted, see module comment; only adjusted for changed MTU, and made some things
// pub(super)

/// The CoAP items in slipmux behave 1:1 like in CoAP-over-UDP. Emulating an embedded-nal stack to
/// exchange them so we can funnel the traffic into a CoAP server.
///
/// As CoAP is currently not async and we don't send messages on our own, this now goes into the
/// simpler embedded-nal rather than embedded-nal-async.
pub(super) struct PseudoStack<'a>(pub(super) &'a mut heapless::Vec<u8, 1501>);

pub(super) struct PseudoSocket;

#[derive(Debug)]
pub(super) struct DoesNotFit;

use core::net::SocketAddr;

impl embedded_nal::UdpClientStack for PseudoStack<'_> {
    type UdpSocket = PseudoSocket;
    type Error = DoesNotFit;
    fn socket(&mut self) -> Result<PseudoSocket, DoesNotFit> {
        panic!("Unused");
    }
    fn connect(&mut self, _: &mut PseudoSocket, _: SocketAddr) -> Result<(), DoesNotFit> {
        panic!("Unused");
    }
    fn send(
        &mut self,
        _: &mut PseudoSocket,
        _: &[u8],
    ) -> Result<(), embedded_nal::nb::Error<DoesNotFit>> {
        panic!("Unused");
    }
    fn receive(
        &mut self,
        _: &mut PseudoSocket,
        outbuf: &mut [u8],
    ) -> Result<(usize, SocketAddr), embedded_nal::nb::Error<DoesNotFit>> {
        let len = self.0.len();
        let outslice = outbuf.get_mut(0..len).ok_or(DoesNotFit)?;
        outslice.copy_from_slice(self.0);

        // Mostly a precaution against sending the request back if ever the minimal CoAP server
        // should *not* send a single response.
        self.0.truncate(0);

        Ok((
            len,
            SocketAddr::V6(core::net::SocketAddrV6::new(
                core::net::Ipv6Addr::UNSPECIFIED,
                0,
                0,
                0,
            )),
        ))
    }
    fn close(&mut self, _: PseudoSocket) -> Result<(), DoesNotFit> {
        panic!("Unused");
    }
}

impl embedded_nal::UdpFullStack for PseudoStack<'_> {
    fn bind(&mut self, _: &mut PseudoSocket, _: u16) -> Result<(), Self::Error> {
        panic!("Unused")
    }
    fn send_to(
        &mut self,
        _: &mut PseudoSocket,
        _: SocketAddr,
        data: &[u8],
    ) -> Result<(), embedded_nal::nb::Error<Self::Error>> {
        // FIXME: Abusing the configuration buffer seems like a convenient thing to do, given we're
        // in full control of .0 for the duration of processing the request -- anything more
        // elegant?

        self.0.clear();
        self.0.extend_from_slice(data).map_err(|_| DoesNotFit)?;
        Ok(())
    }
}
