// use crate::NetworkType;
use self::protocols::*;
pub mod protocols;
pub mod typ;
pub use typ::NetworkType;

// pub struct Network {
//     pub protocol: Box<dyn protocols::Proto> 
// }

// impl Network {
//     pub fn new(pac: &[u8], typ: &Type) -> Self {
//         match typ {
//             Type::Arp => Self { protocol: Box::new(protocols::arp::Arp::new(&pac[14..42])) },
//             Type::Ipv4 => Self { protocol: Box::new(protocols::ipv4::Ipv4::new(&pac[14..34])) },
//             Type::Ipv6 => Self { protocol: Box::new(protocols::ipv6::Ipv6::new(&pac[14..54])) },
//             _ => panic!("Protocol selection")
//         }

//     }
// }

///////////////////// Genetic try
// pub struct Network <T> {
//     pub protocol: T 
// }

// impl<T> Network<T> {
//     pub fn new(pac: &[u8], typ: &Type) -> Self {
//         match typ {
//             Type::Arp => Self { protocol: protocols::arp::Arp::new(&pac[14..42]) }, 
//             Type::Ipv4 => Self { protocol: protocols::ipv4::Ipv4::new(&pac[14..42]) } 
//         }
//     } 
// }

// fn some() {
    
//     let ss = Network {
//         protocol: protocols::arp::Arp::new(&pac[14..42])
//     };
// }


/////////////////// enum

pub struct Network {
    protocol: Protocol
}

impl Network {
    pub fn new (pac: &[u8], typ: &NetworkType) -> Self{
        match typ {
            NetworkType::Arp => Self { protocol: Protocol::Arp(protocols::arp::Arp::new(&pac[..28])) },
            NetworkType::Ipv4 => Self { protocol: Protocol::Ipv4(protocols::ipv4::Ipv4::new(&pac[..20])) },
            NetworkType::Ipv6 => Self { protocol: Protocol::Ipv6(protocols::ipv6::Ipv6::new(&pac[..40])) },
            _ => panic!("Network protocol selection protocol not found | network/mod")
        }
    }

    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    pub fn header_len(&self) -> usize {
        match &self.protocol {
            Protocol::Arp(_) => 28,
            Protocol::Ipv4(ipv4) => ipv4.ihl() as usize * 4,
            Protocol::Ipv6(_) => 40,
        }
    }

    pub fn protocol_string(&self) -> String {
        self.protocol.protcol()
    }
}