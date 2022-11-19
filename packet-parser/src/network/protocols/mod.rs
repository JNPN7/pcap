pub mod ipv4;
pub mod arp;
pub mod ipv6;

// pub trait Proto{
// }
// pub enum Type {
//     Ipv4,
//     Arp,
//     Ipv6 
// }
pub enum Protocol {
    Arp(arp::Arp),
    Ipv4(ipv4::Ipv4),
    Ipv6(ipv6::Ipv6),
}

impl Protocol {
    pub fn protcol(&self) -> String {
        match &self {
            Protocol::Arp(_) => "ARP".to_string(),
            Protocol::Ipv4(_) => "IPv4".to_string(),
            Protocol::Ipv6(_) => "IPv6".to_string(),
        }
    }
}