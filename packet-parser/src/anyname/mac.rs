pub struct Mac {
    addr: [u8; 6]
}

impl Mac {
    pub fn new(addr: [u8; 6]) -> Self {
        Mac {addr}
    }
    pub fn get_addr(&self) -> &[u8] {
        &self.addr
    }
    pub fn get_addr_str(&self) -> String {
        let mut mac = String::new();
        for i in self.addr {
            let m = format!("{:02x}:", i);
            mac.push_str(&m);
        }
        mac.pop();
        mac
    }
}