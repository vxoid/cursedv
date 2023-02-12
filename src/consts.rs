use crate::{commands, Command};

pub const OPEN_PORTS: Command = Command {
    method: commands::device_ports,
    description: "Scans device for open ports|\t--tip, --timeout, --threads",
    name: "op",
};
pub const NETWORK_DEVICES: Command = Command {
    method: commands::network_devices,
    description: "Scans network for devices using arp protocol|\t--netmask, --tip, --interface, --timeout, --threads, --wait",
    name: "netdevices"
};
pub const WHO_HAS: Command = Command {
    method: commands::who_has,
    description: "Does an arp who has request|\t--interface, --tip, --timeout",
    name: "whohas",
};
pub const IS_AT: Command = Command {
    method: commands::is_at,
    description: "Does an arp is at request|\t--interface, --tip, --tmac, --sip, --smac",
    name: "isat",
};
pub const ARP_SPOOF: Command = Command {
    method: commands::arp_spoof,
    description: "Does an arp spoofing/poisoning attack|\t--interface, --timeout, --tip, --tmac, --sip, --smac, --amac, --wait",
    name: "arpspoof"
};
pub const ICMP_DDOS: Command = Command {
    method: commands::icmp_ddos,
    description: "Does an icmp ddos attack|\t--interface, --timeout, --tip, --gip, --gmac",
    name: "icmpddos"
};
pub const HELP: Command = Command {
    method: commands::help,
    description: "Gives information about all commands|",
    name: "help",
};
pub const COMMANDS_COUNT: usize = 7;
pub const COMMANDS: [Command; COMMANDS_COUNT] =
    [OPEN_PORTS, NETWORK_DEVICES, WHO_HAS, IS_AT, ARP_SPOOF, ICMP_DDOS, HELP];
