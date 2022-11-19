use crate::anyname::helper::{concat_bits16};
pub struct Udp {
    src_port: u16,
    dst_port: u16,
    length: u16,
    checksum: u16
}

impl Udp {
    pub fn new(pac: &[u8]) -> Self {
        let src_port: u16 = concat_bits16(&pac[..2]);
        
        let dst_port: u16 = concat_bits16(&pac[2..4]);

        let length: u16 = concat_bits16(&pac[4..6]);

        let checksum: u16 = concat_bits16(&pac[6..8]);

        Self {
            src_port,
            dst_port,
            length,
            checksum
        }
    } 

    pub fn src_port(&self) -> &u16 {
        &self.src_port
    }

    pub fn dst_port(&self) -> &u16 {
        &self.dst_port
    }

    pub fn length(&self) -> &u16 {
        &self.length
    }

    pub fn checksum(&self) -> &u16 {
        &self.checksum
    }
}