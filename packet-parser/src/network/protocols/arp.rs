use crate::Mac;
use crate::{IpAddr, Ipv4Addr};
// use super::*;

pub struct Arp {
    hardware_type: [u8; 2],
    protocol: [u8; 2], // IPV4 or IPV6
    hardware_size: u8,
    protocol_size: u8,
    opcode: [u8; 2],
    sender_mac: Mac,
    sender_ip: IpAddr,
    target_mac: Mac, // 00:00:00:00:00:00 in request
    target_ip: IpAddr,
    // typ: bool, // request or response later enum if needed
}

// impl Proto for Arp {}

impl Arp {
    pub fn new(pac: &[u8]) -> Self {
        let mut hardware_type: [u8; 2] = [0; 2];
        hardware_type.copy_from_slice(&pac[0..2]);

        let mut protocol: [u8; 2] = [0; 2];
        protocol.copy_from_slice(&pac[2..4]);

        let hardware_size: u8 = pac[4].clone();
       
        let protocol_size: u8 = pac[5].clone();

        let mut opcode: [u8; 2] = [0; 2];
        opcode.copy_from_slice(&pac[6..8]);

        let mut mac: [u8; 6] = [0; 6];
        mac.copy_from_slice(&pac[8..14]);
        let sender_mac: Mac = Mac::new(mac);

        let mut ip: [u8; 4] = [0; 4];
        ip.copy_from_slice(&pac[14..18]);
        let sender_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(ip));
        
        mac.copy_from_slice(&pac[18..24]);
        let target_mac: Mac = Mac::new(mac);

        ip.copy_from_slice(&pac[24..28]);
        let target_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(ip));

        Self {
            hardware_type,
            protocol,
            hardware_size,
            protocol_size,
            opcode,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip
        }
    }

    pub fn hardware_type(&self) -> &[u8] {
       &self.hardware_type 
    }

    pub fn protocol(&self) -> &[u8] {
        &self.protocol
    }

    pub fn sender_mac(&self) -> &Mac {
        &self.sender_mac
    }

    pub fn sender_ip(&self) -> &IpAddr {
        &self.sender_ip
    }

    pub fn target_mac(&self) -> &Mac {
        &&self.target_mac
    }

    pub fn target_ip(&self) -> &IpAddr {
        &self.target_ip
    }

    pub fn opcode(&self) -> &[u8] {
        &self.opcode
    }

    pub fn hardware_size(&self) -> &u8 {
        &self.hardware_size
    }

    pub fn protocol_size(&self) -> &u8 {
        &self.protocol_size
    }

}