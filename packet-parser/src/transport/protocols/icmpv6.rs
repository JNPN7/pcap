// icmpv6 is network layer protocol not tranport layer
// need to correct this in the future
use crate::anyname::{ip::{IpAddr, Ipv6Addr}, mac::Mac, helper::{concat_bits32, concat_bits16}};

pub struct Icmpv6 {
    typ: u8,
    code: u8,
    checksum: u16,
    flag: u32,
    target_address: IpAddr,
    opt_typ: Option<u8>,
    opt_len: Option<u8>,
    opt_mac: Option<Mac>
}

impl Icmpv6 {
    pub fn new(pac: &[u8]) -> Self {
        let typ: u8 = pac[0].clone();
        let code: u8 = pac[1].clone();
        let checksum: u16 = concat_bits16(&pac[2..4]);
        let flag: u32 = concat_bits32(&pac[4..8]);

        let mut ip: [u8; 16] = [0; 16];
        ip.copy_from_slice(&pac[8..24]);
        let target_address: IpAddr = IpAddr::V6(Ipv6Addr::new(ip));


        let mut opt_typ = None;
        let mut opt_len = None;
        let mut opt_mac = None;
        match &typ {
            136 =>  {},  
            135 =>  {
                opt_typ = Some(pac[24].clone());
                opt_len = Some(pac[25].clone());

                let mut mac: [u8; 6] = [0; 6];
                mac.copy_from_slice(&pac[26..32]);
                opt_mac= Some(Mac::new(mac));

            },
            _ => panic!("Icmpv6 error type not found | transport/protocols/icmpv6 | type => {}", &typ)
        }
        
        Self {
            typ,
            code,
            checksum,
            flag,
            target_address,
            opt_typ,
            opt_len,
            opt_mac
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

    pub fn flag(&self) -> &u32 {
        &self.flag
    }

    pub fn target_address(&self) -> &IpAddr {
        &self.target_address
    }

    pub fn opt_typ(&self) -> &Option<u8> {
        &self.opt_typ
    }

    pub fn opt_len(&self) -> &Option<u8> {
        &self.opt_len
    }

    pub fn opt_mac(&self) -> &Option<Mac> {
        &self.opt_mac
    }
}