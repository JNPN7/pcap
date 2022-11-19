use crate::network::typ::find_type;
use super::{anyname::mac::Mac, NetworkType};

pub struct Datalink {
    source: Mac,
    destination: Mac,
    typ: NetworkType
}

impl Datalink {
    pub fn new(pac: &[u8]) -> Self {
        let mut mac: [u8; 6] = [0; 6];
        mac.copy_from_slice(&pac[0..6]); // don't know difference between copy and clone later keep which is faster i think copy as membit is copied 
    
        let source = Mac::new(mac); 
        
        mac.clone_from_slice(&pac[6..12]);
        let destination = Mac::new(mac);
       
        let typ = find_type(&pac[12..14]);
        
        Self { 
            source, 
            destination, 
            typ 
        } 
    }

    pub fn srcmac(&self) -> &[u8] {
        &self.source.get_addr()
    }

    pub fn dstmac(&self) -> &[u8] {
        &self.destination.get_addr()
    }

    pub fn typ(&self) -> &NetworkType {
        &self.typ
    }
    pub fn header_len(&self) -> usize {
        14
    }

    pub fn srcmac_string(&self) -> String {
        self.source.get_addr_str()
    }

    pub fn dstmac_string(&self) -> String {
        self.destination.get_addr_str()
    }
}