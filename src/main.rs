use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    // Set up the network interface
    let mut config = tun::Configuration::default();
    config
        .address((10, 0, 0, 6))
        .netmask((255, 255, 255, 0))
        .destination((10, 0, 0, 1))
        .up();

    #[cfg(target_os = "linux")]
    config.platform_config(|config| {
        // requiring root privilege to acquire complete functions
        config.ensure_root_privileges(true);
    });

    let mut nic = tun::create(&config)?;
    let mut buff = [0u8; 1504];
    
    // Read in from the tunnel
    loop {
        let read = nic.read(&mut buff)?;
        println!("read {} bytes: {:?}", read, &buff[..read]);
    }
}
