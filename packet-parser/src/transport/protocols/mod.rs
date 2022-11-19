pub mod tcp;
pub mod icmp;
pub mod icmpv6;
pub mod udp;

pub enum Protocol {
    Tcp(tcp::Tcp),
    Icmp(icmp::Icmp),
    Icmpv6(icmpv6::Icmpv6),
    Udp(udp::Udp)
}

impl Protocol {
    pub fn protocol(&self) -> String {
        match &self {
            Protocol::Tcp(_) => "TCP".to_string(),
            Protocol::Icmp(_) => "ICMP".to_string(),
            Protocol::Icmpv6(_) => "ICMPv6".to_string(),
            Protocol::Udp(_) => "UDP".to_string(),
        }
    }
}