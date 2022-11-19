use crate::anyname::helper::{concat_bits16, concat_bits32};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Tcp {
    src_port: u16,
    dst_port: u16,
    seq_no: u32, //[u8; 4], // sequence number relative and raw 
    ack_no: u32, // [u8; 4],
    header_len: u8, // need only 4 bit tho need another good data type
    flag: u16, // need only 12 bit tho 
    window: u16,
    checksum: u16,
    urgent_pointer: u32,

    // options 12 byte struct is good or this don't know
    // options changes with request from no option to 24(may be more found upto 0, 12, 24)

    // nop1: u8,
    // nop2: u8,
    // timestamp_kind: u8,
    // timestamp_len: u8,
    // timestamp_val1: u32,
    // timestamp_val2: u32
}

impl Tcp {
    pub fn new(pac: &[u8]) -> Self {
        let src_port: u16 = concat_bits16(&pac[..2]);
        
        let dst_port: u16 = concat_bits16(&pac[2..4]);

        let seq_no: u32 = concat_bits32(&pac[4..8]);

        let ack_no: u32 = concat_bits32(&pac[8..12]);

        let header_len: u8 = pac[12].clone() >> 4;

        let flag: u16 = concat_bits16(&pac[12..14]) & 4095;
        
        let window: u16 = concat_bits16(&pac[14..16]);

        let checksum: u16 = concat_bits16(&pac[16..18]);

        let urgent_pointer: u32 = concat_bits32(&pac[18..20]);


        // Option gets change so need good way to handle
        // Will continue after getting all the data about options that can be obtained from the packet

        // let nop1: u8 = pac[20].clone();

        // let nop2: u8 = pac[21].clone();
        
        // let timestamp_kind: u8 = pac[22].clone();

        // let timestamp_len: u8 = pac[23].clone(); 

        // let timestamp_val1: u32 = concat_bits32(&pac[24..28]);

        // let timestamp_val2: u32 = concat_bits32(&pac[28..32]);

        Self { 
            src_port,
            dst_port,
            seq_no,
            ack_no,
            header_len,
            flag,
            window,
            checksum,
            urgent_pointer,
            // nop1,
            // nop2,
            // timestamp_kind,
            // timestamp_len,
            // timestamp_val1,
            // timestamp_val2
        }
    }

    pub fn src_port(&self) -> &u16 {
        &self.src_port
    }

    pub fn dst_port(&self) -> &u16 {
        &self.dst_port
    }

    pub fn seq_no(&self) -> &u32 {
        &self.seq_no
    }

    pub fn ack_no(&self) -> &u32 {
        &self.ack_no
    }

    pub fn header_len(&self) -> &u8 {
        &self.header_len
    }

    pub fn flag(&self) -> &u16 {
        &self.flag
    }

    pub fn window(&self) -> &u16 {
        &self.window
    }

    pub fn checksum(&self) -> &u16 {
        &self.checksum
    }

    pub fn urgent_pointer(&self) -> &u32 {
        &self.urgent_pointer
    }

    // pub fn nop1(&self) -> &u8 {
    //     &self.nop1
    // }

    // pub fn nop2(&self) -> &u8 {
    //     &self.nop2
    // }

    // pub fn timestamp_kind(&self) -> &u8 {
    //     &self.timestamp_kind
    // }

    // pub fn timestamp_len(&self) -> &u8 {
    //     &self.timestamp_len
    // }

    // pub fn timestamp_val1(&self) -> &u32 {
    //     &self.timestamp_val1
    // }

    // pub fn timestamp_val2(&self) -> &u32 {
    //     &self.timestamp_val2
    // }

    // one of the flag
    pub fn nonce(&self) -> bool {
        // if &self.flag & (1 << 9) {true} else {false}
        let flag = &self.flag & (1 << 8);
        if flag > 0 { true } else { false }
    }

    // Congestion Window Reduced
    pub fn cwr(&self) -> bool {
        let flag = &self.flag & (1 << 7);
        if flag > 0 { true } else { false }
    }

    pub fn ecn_echo(&self) -> bool {
        let flag = &self.flag & (1 << 6);
        if flag > 0 { true } else { false }
    }

    pub fn urgent(&self) -> bool {
        let flag = &self.flag & (1 << 5);
        if flag > 0 { true } else { false }
    }

    pub fn ack(&self) -> bool {
        let flag = &self.flag & (1 << 4);
        if flag > 0 { true } else { false }
    }

    pub fn push(&self) -> bool {
        let flag = &self.flag & (1 << 3);
        if flag > 0 { true } else { false }
    }

    pub fn reset(&self) -> bool {
        let flag = &self.flag & (1 << 2);
        if flag > 0 { true } else { false }
    }

    pub fn syn(&self) -> bool {
        let flag = &self.flag & (1 << 1);
        if flag > 0 { true } else { false }
    }

    pub fn fin(&self) -> bool {
        let flag = &self.flag & 1;
        if flag > 0 { true } else { false }
    }
}