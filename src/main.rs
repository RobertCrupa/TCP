use std::collections::{hash_map, HashMap};
use std::net::Ipv4Addr;

use nix::libc::quad_t;
mod tcp;

#[derive(PartialEq, Eq, Hash)]
struct Quad {
    src: (Ipv4Addr, u16),
    dest: (Ipv4Addr, u16)
}
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    // Set up the network interface
    let mut config = tun::Configuration::default();
    config
        .address((10, 0, 0, 6))
        .netmask((255, 255, 255, 0))
        .destination((10, 0, 0, 1))
        .mtu(1504)
        .up();

    #[cfg(target_os = "linux")]
    config.platform_config(|config| {
        // requiring root privilege to acquire complete functions
        config.ensure_root_privileges(true);
    });

    let nic = tun::create(&config)?;
    let mut buff = [0u8; 1504];

    // TCP connection
    let mut connections: HashMap<Quad, tcp::State> = Default::default();
    
    // Read in from the tunnel
    loop {
        let read = nic.recv(&mut buff)?;
        
        // Process the IPv4 packet
        if let Ok(ip_hdr) = etherparse::Ipv4HeaderSlice::from_slice(&buff[..read]) {
            let source = ip_hdr.source_addr();
            let dest = ip_hdr.destination_addr();

            // Process TCP packets
            if let Ok(tcp_hdr) = etherparse::TcpHeaderSlice::from_slice(&buff[ip_hdr.slice().len()..]) {
                let src_port = tcp_hdr.source_port();
                let dest_port = tcp_hdr.destination_port();
                let header_size = ip_hdr.slice().len() + tcp_hdr.slice().len();

                connections.entry(Quad {
                    src: (source, src_port),
                    dest: (dest, dest_port)
                }).or_default().on_packet(ip_hdr, tcp_hdr, &buff[header_size..]);

            }
        }
    }
}
