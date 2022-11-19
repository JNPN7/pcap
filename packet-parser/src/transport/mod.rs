use self::protocols::Protocol;
pub mod protocols;
mod typ;
pub use typ::*;

pub struct Transport {
    protocol: Protocol
}

impl Transport {
    pub fn new(pac: &[u8], typ: &TransportType)  -> Self {
        match typ {
            TransportType::Tcp => Self { protocol: Protocol::Tcp(protocols::tcp::Tcp::new(&pac)) },
            TransportType::Icmp => Self { protocol: Protocol::Icmp(protocols::icmp::Icmp::new(&pac)) },
            TransportType::Icmpv6 => Self { protocol: Protocol::Icmpv6(protocols::icmpv6::Icmpv6::new(&pac)) },
            TransportType::Udp => Self { protocol: Protocol::Udp(protocols::udp::Udp::new(&pac)) },
            // _ => panic!("Remains in transport/mod")
        }
    }

    pub fn protocol(&self) -> &Protocol{
        &self.protocol
    }
    
    pub fn header_len(&self) -> usize {
        match &self.protocol {
            Protocol::Tcp(tcp) => tcp.header_len().clone() as usize,
            Protocol::Icmp(_) => 0,
            Protocol::Icmpv6(_) => 0, 
            Protocol::Udp(_) => 8
        }
    }

    pub fn protocol_string(&self) -> String {
        self.protocol.protocol()
    }   
}