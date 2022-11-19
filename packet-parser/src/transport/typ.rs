use core::panic;

#[derive(Debug)]
pub enum TransportType {
    Tcp,
    Udp,
    Icmp,
    Icmpv6
}


pub fn find_type(typ: &u8) -> TransportType {
    match typ {
        1 => TransportType::Icmp,
        6 => TransportType::Tcp,
        17 => TransportType::Udp,
        58 => TransportType::Icmpv6,
        _ => panic!("Transport type not found. | tranport/typ | type => {}", &typ)
    }
}