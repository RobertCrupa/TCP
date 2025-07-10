pub struct State {}

impl Default for State {
    fn default() -> Self {
        State {}
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        ip_hdr: etherparse::Ipv4HeaderSlice<'a>,
        tcp_hdr: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        eprintln!(
            "{}:{} -> {}:{} TCP size: {}",
            ip_hdr.source_addr(),
            tcp_hdr.source_port(),
            ip_hdr.destination_addr(),
            tcp_hdr.destination_port(),
            data.len()
        );
    }
}
