// icmp is network layer protocol so it should not be here
// need to correct this in the future
use crate::anyname::helper::{concat_bits16, concat_bits32};

pub struct Icmp {
    typ: u8,
    code: u8,
    checksum: u16,
    identifier_be: u16,
    identifier_le: u16,
    sequence_no_be: u16,
    sequence_no_le: u16,
    timestamp: u32,
    data: Vec<u8>
}

impl Icmp {
    pub fn new(pac: &[u8]) -> Self {
        let typ: u8 = pac[0].clone();
        let code: u8 = pac[1].clone();
        let checksum: u16 = concat_bits16(&pac[2..4]);
        let identifier_be: u16 = concat_bits16(&pac[4..6]);
        let identifier_le: u16 = identifier_be.clone();  // something need to be done don't know what
        let sequence_no_be: u16 = concat_bits16(&pac[6..8]);
        let sequence_no_le: u16 = sequence_no_be.clone(); // something need to be done don't know what
        let timestamp: u32 = concat_bits32(&pac[8..16]);

        let mut data: Vec<u8> = Vec::new();
        
        if pac.len() > 12 {
            data.extend_from_slice(&pac[16..]);
        }

        Self {
            typ,
            code,
            checksum,
            identifier_be,
            identifier_le,
            sequence_no_be,
            sequence_no_le,
            timestamp,
            data
        }
    }

    pub fn typ(&self) -> &u8 {
        &self.typ
    }

    pub fn code(&self) -> &u8 {
        &self.code
    }

    pub fn checksum(&self) -> &u16 {
        &self.checksum
    }

    pub fn identifier_be(&self) -> &u16 {
        &self.identifier_be
    }

    pub fn identifier_le(&self) -> &u16 {
        &self.identifier_le
    }

    pub fn sequence_no_be(&self) -> &u16 {
        &self.sequence_no_be
    }

    pub fn sequence_no_le(&self) -> &u16 {
        &self.sequence_no_le
    }

    pub fn timestamp(&self) -> &u32 {
        &self.timestamp
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}