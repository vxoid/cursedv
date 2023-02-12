use crate::*;

pub struct Config {
    interface: Option<String>,
    netmask: Option<Ipv4>,
    threads: Option<u64>,
    timeout: Option<u64>,
    amac: Option<Mac>,
    tmac: Option<Mac>,
    smac: Option<Mac>,
    gmac: Option<Mac>,
    wait: Option<u64>,
    gip: Option<Ipv4>,
    tip: Option<Ipv4>,
    sip: Option<Ipv4>,
    command: Command,
}

impl Config {
    pub fn from(args: &[String]) -> Result<Self, CursedErrorHandle> {
        let mut interface: Option<String> = None;
        let mut netmask: Option<Ipv4> = None;
        let mut timeout: Option<u64> = None;
        let mut threads: Option<u64> = None;
        let mut amac: Option<Mac> = None;
        let mut tmac: Option<Mac> = None;
        let mut smac: Option<Mac> = None;
        let mut gmac: Option<Mac> = None;
        let mut wait: Option<u64> = None;
        let mut gip: Option<Ipv4> = None;
        let mut tip: Option<Ipv4> = None;
        let mut sip: Option<Ipv4> = None;

        let command: Command = match args.get(1) {
            Some(command_name) => {
                let mut command: Option<Command> = None;

                for const_command in COMMANDS {
                    if const_command.name == command_name {
                        command = Some(const_command)
                    }
                }

                match command {
                    Some(command) => command,
                    None => {
                        return Err(CursedErrorHandle::new(
                            CursedError::InvalidCommand,
                            format!("{} is not valid command", command_name),
                        ))
                    }
                }
            }
            None => {
                return Err(CursedErrorHandle::new(
                    CursedError::NotEnought,
                    String::from("No command was given"),
                ))
            }
        };

        let mut i = 2;
        loop {
            if i >= args.len() {
                break;
            }

            match &args[i][..] {
                "--tip" => {
                    i += 1;

                    tip = match args.get(i) {
                        Some(ip) => match parse_ip(&ip[..]) {
                            Ok(ip) => Some(ip),
                            Err(err) => return Err(err),
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--tip is not specified"),
                            ))
                        }
                    }
                }
                "--tmac" => {
                    i += 1;

                    tmac = match args.get(i) {
                        Some(mac) => match parse_mac(mac) {
                            Ok(mac) => Some(mac),
                            Err(err) => return Err(err),
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--tmac is not specified"),
                            ))
                        }
                    }
                }
                "--sip" => {
                    i += 1;

                    sip = match args.get(i) {
                        Some(ip) => match parse_ip(&ip[..]) {
                            Ok(ip) => Some(ip),
                            Err(err) => return Err(err),
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--sip is not specified"),
                            ))
                        }
                    }
                }
                "--smac" => {
                    i += 1;

                    smac = match args.get(i) {
                        Some(mac) => match parse_mac(mac) {
                            Ok(mac) => Some(mac),
                            Err(err) => return Err(err),
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--smac is not specified"),
                            ))
                        }
                    }
                },
                "--gip" => {
                    i += 1;

                    gip = match args.get(i) {
                        Some(ip) => match parse_ip(&ip[..]) {
                            Ok(ip) => Some(ip),
                            Err(err) => return Err(err),
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--gip is not specified"),
                            ))
                        }
                    }
                },
                "--gmac" => {
                    i += 1;

                    gmac = match args.get(i) {
                        Some(mac) => match parse_mac(mac) {
                            Ok(mac) => Some(mac),
                            Err(err) => return Err(err),
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--gmac is not specified"),
                            ))
                        }
                    }
                },
                "--amac" => {
                    i += 1;

                    amac = match args.get(i) {
                        Some(mac) => match parse_mac(mac) {
                            Ok(mac) => Some(mac),
                            Err(err) => return Err(err),
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--amac is not specified"),
                            ))
                        }
                    }
                }
                "--threads" => {
                    i += 1;
                    threads = match args.get(i) {
                        Some(new_threads) => match new_threads.parse() {
                            Ok(threads) => Some(threads),
                            Err(err) => {
                                return Err(CursedErrorHandle::new(
                                    CursedError::Parse,
                                    format!(
                                        "{} is not valid integer ({})",
                                        new_threads,
                                        err.to_string()
                                    ),
                                ))
                            }
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--threads is not specified"),
                            ))
                        }
                    }
                }
                "--timeout" => {
                    i += 1;
                    timeout = match args.get(i) {
                        Some(timeout) => match timeout.parse() {
                            Ok(timeout) => Some(timeout),
                            Err(err) => {
                                return Err(CursedErrorHandle::new(
                                    CursedError::Parse,
                                    format!(
                                        "{} is not valid integer ({})",
                                        timeout,
                                        err.to_string()
                                    ),
                                ))
                            }
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--timeout is not specified"),
                            ))
                        }
                    };
                }
                "--wait" | "-w" => {
                    i += 1;

                    wait = match args.get(i) {
                        Some(wait) => match wait.parse() {
                            Ok(wait) => Some(wait),
                            Err(err) => {
                                return Err(CursedErrorHandle::new(
                                    CursedError::Parse,
                                    format!("{} is not valid integer ({})", wait, err),
                                ))
                            }
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--wait is not specified"),
                            ))
                        }
                    }
                }
                "--netmask" | "-n" => {
                    i += 1;

                    netmask = match args.get(i) {
                        Some(netmask) => match parse_ip(&netmask[..]) {
                            Ok(netmask) => Some(netmask),
                            Err(err) => return Err(err),
                        },
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--netmask is not specified"),
                            ))
                        }
                    }
                }
                "--interface" | "-i" => {
                    i += 1;

                    interface = match args.get(i) {
                        Some(interface) => Some(interface.clone()),
                        None => {
                            return Err(CursedErrorHandle::new(
                                CursedError::NotEnought,
                                String::from("--interface is not specified"),
                            ))
                        }
                    }
                },
                _ => {
                    return Err(CursedErrorHandle::new(
                        CursedError::InvalidOption,
                        format!("There isn\'t {} option", args[i]),
                    ))
                }
            }

            i += 1;
        }

        Ok(Self {
            command,
            threads,
            tip,
            sip,
            gip,
            gmac,
            tmac,
            smac,
            amac,
            timeout,
            netmask,
            interface,
            wait,
        })
    }

    getters!(
        pub get_tip(tip) -> Option<Ipv4>;
        pub get_sip(sip) -> Option<Ipv4>;
        pub get_gip(gip) -> Option<Ipv4>;
        pub get_amac(amac) -> Option<Mac>;
        pub get_tmac(tmac) -> Option<Mac>;
        pub get_smac(smac) -> Option<Mac>;
        pub get_gmac(gmac) -> Option<Mac>;
        pub get_wait(wait) -> Option<u64>;
        pub get_command(command) -> Command;
        pub get_threads(threads) -> Option<u64>;
        pub get_timeout(timeout) -> Option<u64>;
        pub get_netmask(netmask) -> Option<Ipv4>;
        pub get_interface(interface) -> Option<String>;
    );

    pub fn run(&self) -> Result<(), CursedErrorHandle> {
        (self.command.method)(self)
    }
}

fn parse_ip(str: &str) -> Result<Ipv4, CursedErrorHandle> {
    let mut ip: [u8; IPV4_LEN] = [0; IPV4_LEN];

    let ip_octets: Vec<&str> = str.split(".").collect();

    if ip_octets.len() != IPV4_LEN {
        return Err(CursedErrorHandle::new(
            CursedError::Parse,
            format!(
                "{} have to not enought or too many octets ({}!={})",
                str,
                ip_octets.len(),
                IPV4_LEN
            ),
        ));
    }

    for octet in 0..IPV4_LEN {
        ip[octet] = match ip_octets[octet].parse() {
            Ok(ip_part) => ip_part,
            Err(err) => {
                println!("Can\'t parse {} as v4 ip", str);
                return Err(CursedErrorHandle::new(
                    CursedError::Parse,
                    format!(
                        "{} is not valid integer ({})",
                        ip_octets[octet],
                        err.to_string()
                    ),
                ));
            }
        }
    }

    Ok(Handle::from(ip))
}

fn parse_mac(str: &str) -> Result<Mac, CursedErrorHandle> {
    let mut mac: [u8; MAC_LEN] = [0; MAC_LEN];

    let mac_octets: Vec<&str> = str.split(':').collect();

    if mac_octets.len() != MAC_LEN {
        return Err(CursedErrorHandle::new(
            CursedError::Parse,
            format!(
                "{} have to not enought or too many octets ({}!={})",
                str,
                mac_octets.len(),
                MAC_LEN
            ),
        ));
    }

    for octet in 0..MAC_LEN {
        let mut dec: u8 = 0;

        let bytes: &[u8] = mac_octets[octet].as_bytes();
        let length: usize = bytes.len();
        if length > 2 || length < 1 {
            return Err(CursedErrorHandle::new(
                CursedError::Parse,
                format!("{} is not u8 parsable hex", mac_octets[octet]),
            ));
        }

        for i in 0..length {
            let byte_dec: u8 = match hex_as_dec(bytes[i] as char) {
                Ok(byte_dec) => byte_dec,
                Err(err) => return Err(err),
            };

            dec |= byte_dec << (length - 1 - i) * 4
        }

        mac[octet] = dec
    }

    Ok(Handle::from(mac))
}

fn hex_as_dec(c: char) -> Result<u8, CursedErrorHandle> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'a' | 'A' => Ok(10),
        'b' | 'B' => Ok(11),
        'c' | 'C' => Ok(12),
        'd' | 'D' => Ok(13),
        'e' | 'E' => Ok(14),
        'f' | 'F' => Ok(15),
        _ => Err(CursedErrorHandle::new(
            CursedError::Parse,
            format!("Can\'t parse {} as hex", c),
        )),
    }
}
