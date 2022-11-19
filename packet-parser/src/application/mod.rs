// Naming is not appropriate // TSL (Transport Layer Security) 
pub struct Application {
    content_typ: u8,
    ver: u16,
    length: u16,
    data: Vec<u8>
}

impl Application {
    pub fn new(pac: &[u8]) -> Self {
        let content_typ = pac[0];
        let ver = concat_bits16(&pac[1..3]);
        let length = concat_bits16(&pac[3..5]);
        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(&pac[5..]);

        Self {
            content_typ,
            ver,
            length,
            data
        }
    }

    pub fn content_typ(&self) -> &u8 {
        &self.content_typ
    }

    pub fn ver(&self) -> &u16 {
        &self.ver
    }

    pub fn length(&self) -> &u16 {
        &self.length
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

fn concat_bits16(a: &[u8]) -> u16 {
    a.iter()
        .fold(0, |acc, x| acc << 8 | x.clone() as u16)
}