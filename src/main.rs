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
    
    // Read in from the tunnel
    loop {
        let read = nic.recv(&mut buff)?;

        if let Ok(packet) = etherparse::Ipv4HeaderSlice::from_slice(&buff[..read]) {
            let source = packet.source_addr();
            let dest = packet.destination_addr();

            if let Ok(packet) = etherparse::TcpHeaderSlice::from_slice(&buff[packet.slice().len()..]) {
                let src_port = packet.source_port();
                let dest_port = packet.destination_port();

                eprintln!("{} -> {} TCP {} {} -> {}", source, dest, packet.slice().len(), src_port, dest_port);
            }
        }
    }
}
