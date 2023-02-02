use crate::*;

pub fn device_ports(config: &Config) -> Result<(), CursedErrorHandle> {
    let open_ports: Vec<u16> = match sys::scan_ports(config) {
        Ok(open_ports) => open_ports,
        Err(err) => return Err(err),
    };

    println!("Got {} open ports:", open_ports.len());
    for port in open_ports {
        println!("\t{}", port)
    }

    Ok(())
}

pub fn network_devices(config: &Config) -> Result<(), CursedErrorHandle> {
    let devices: Vec<(Ipv4, Mac)> = match sys::scan_network(config) {
        Ok(devices) => devices,
        Err(err) => return Err(err),
    };

    println!("Got {} devices that responsed:", devices.len());
    for device in devices {
        println!("\t{}\t{}", device.0, device.1)
    }

    Ok(())
}

pub fn who_has(config: &Config) -> Result<(), CursedErrorHandle> {
    let (ip_addr, mac_addr): (Ipv4, Mac) = match sys::who_has(config) {
        Ok(device) => device,
        Err(err) => return Err(err),
    };

    println!("{}\t{}", ip_addr, mac_addr);

    Ok(())
}

pub fn is_at(config: &Config) -> Result<(), CursedErrorHandle> {
    sys::is_at(config)
}

pub fn spoof(config: &Config) -> Result<(), CursedErrorHandle> {
    sys::spoof(config)
}

pub fn help(_: &Config) -> Result<(), CursedErrorHandle> {
    println!("COMMANDS:");
    for command in COMMANDS {
        println!("\t{}\t{}", command.name, command.description)
    }

    Ok(())
}
