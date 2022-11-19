use crate::{IpAddr, Ipv6Addr, TransportType, transport::find_type};
// use super::*;

pub struct Ipv6 {
    ver_traffic_ecn: [u8; 4], // need to be managed not this way need to parse 
    payload_len: [u8; 2],
    nxt_header: TransportType, // protocol
    hop_limit: u8,
    src: IpAddr,
    dst: IpAddr
}

// impl Proto for Ipv6 {}

impl Ipv6 {
    pub fn new(pac: &[u8]) -> Self {
        let mut ver_traffic_ecn: [u8; 4] = [0; 4];
        ver_traffic_ecn.copy_from_slice(&pac[0..4]);

        let mut payload_len: [u8; 2] = [0; 2];
        payload_len.copy_from_slice(&pac[4..6]);

        let nxt_header: TransportType = find_type(&pac[6]);
        
        let hop_limit: u8 = pac[7].clone();
        
        let mut  ip: [u8; 16] = [0; 16];
        ip.copy_from_slice(&pac[8..24]);
        let src: IpAddr = IpAddr::V6(Ipv6Addr::new(ip));


        ip.copy_from_slice(&pac[24..40]);
        let dst: IpAddr = IpAddr::V6(Ipv6Addr::new(ip));

        Self { 
            ver_traffic_ecn, 
            payload_len,
            nxt_header, 
            hop_limit, 
            src, 
            dst 
        }
    }

    pub fn ver_traffic_ecn(&self) ->  &[u8] {
        &self.ver_traffic_ecn
    }
    
    pub fn payload_len(&self) ->  &[u8] {
        &self.payload_len
    }
    
    pub fn nxt_header(&self) ->  &TransportType {
        &self.nxt_header
    }
    
    pub fn hop_limit(&self) -> &u8 {
        &self.hop_limit
    } 

    pub fn src(&self) -> &IpAddr {
        &self.src
    }

    pub fn dst(&self) -> &IpAddr {
        &self.dst
    }
}