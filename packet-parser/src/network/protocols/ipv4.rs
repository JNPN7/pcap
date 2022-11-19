use crate::{IpAddr, Ipv4Addr, TransportType, transport::find_type};

// use super::*;

pub struct Ipv4 {
    header_len: u8,
    ecn: u8, // Explicit Congestion Notifcation
    total_len: [u8; 2], // packet length
    id: [u8; 2], // identification
    fragment_offset: [u8; 2],
    ttl: u8, // Time To Live
    protocol: TransportType,
    checksum: [u8; 2], // header checksum
    src: IpAddr,
    dst: IpAddr,
    options: Vec<u8>
}

// impl Proto for Ipv4 {}

// contains print function
impl Ipv4 {
    pub fn new(pac: &[u8]) -> Self {
        let header_len: u8 = pac[0].clone();

        let ecn: u8 = pac[1].clone();

        let mut total_len: [u8; 2] = [0; 2];
        total_len.copy_from_slice(&pac[2..4]);

        let mut id: [u8; 2] = [0; 2];
        id.copy_from_slice(&pac[4..6]);

        let mut fragment_offset: [u8; 2] = [0; 2];
        fragment_offset.clone_from_slice(&pac[6..8]);

        let ttl: u8 = pac[8].clone();

        let protocol: TransportType = find_type(&pac[9]);

        let mut checksum: [u8; 2] = [0; 2];
        checksum.copy_from_slice(&pac[10..12]);

        let mut ip: [u8; 4] = [0; 4];
        ip.copy_from_slice(&pac[12..16]);
        let src: IpAddr = IpAddr::V4(Ipv4Addr::new(ip));

        ip.copy_from_slice(&pac[16..20]);
        let dst: IpAddr = IpAddr::V4(Ipv4Addr::new(ip));

        let idh: usize = (header_len & 15 * 4).into();
        let mut options: Vec<u8> = Vec::new();

        if idh > 20 {
            println!("options fo ipv4");
            // options.extend(pac[20..idh].iter().cloned());
            options.extend_from_slice(&pac[20..idh]);
        } 

        Self { 
            header_len, 
            ecn,
            total_len,
            id,
            fragment_offset,
            ttl,
            protocol,
            checksum,
            src,
            dst,
            options
        }
    }

    pub fn header_len(&self) -> &u8 {
        &self.header_len
    }

    pub fn ecn(&self) -> &u8 {
        &self.ecn
    }

    pub fn total_len(&self) -> &[u8] {
        &self.total_len
    }

    pub fn id(&self) -> &[u8] {
        &self.id
    }

    pub fn fragment_offset(&self) -> &[u8] {
        &self.fragment_offset
    }

    pub fn ttl(&self) -> &u8 {
        &self.ttl
    }

    pub fn protocol(&self) -> &TransportType {
        &self.protocol
    }

    pub fn checksum(&self) -> &[u8] {
        &self.checksum
    }

    pub fn src(&self) -> &IpAddr {
        &self.src
    }

    pub fn dst(&self) -> &IpAddr {
        &self.dst
    }

    pub fn options(&self) -> &Vec<u8> {
        &self.options
    }

    pub fn version(&self) -> u8 {
        &self.header_len >> 4
    }

    pub fn ihl(&self) -> u8 {
        &self.header_len & 15
    }
}