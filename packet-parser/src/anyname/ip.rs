pub struct Ipv4Addr {
    addr: [u8; 4]
}

pub struct Ipv6Addr {
    addr: [u8; 16]
}


pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

impl IpAddr {
    pub fn get_addr_str(&self) -> String {
        match &self {
            Self::V4(ip) => ip.get_addr_str(),
            Self::V6(ip) => ip.get_addr_str()
        }
    }
}

impl Ipv4Addr  {
    pub fn new(addr: [u8; 4]) -> Self {
        Self {addr}
    }
    pub fn get_addr(&self) -> &[u8] {
        &self.addr
    }
    pub fn get_addr_str(&self) -> String {
        let mut ip = String::new();
 
        for i in self.addr {
            let m = format!("{}.", i);
            ip.push_str(&m);
        }
        ip.pop();
        ip
    }
}

impl Ipv6Addr  {
    pub fn new(addr: [u8; 16]) -> Self {
        Self {addr}
    }
    pub fn get_addr(&self) -> &[u8] {
        &self.addr
    }
    pub fn get_addr_str(&self) -> String {
        let mut ip = String::new();
        for (n , i) in self.addr.iter().enumerate() {
            let m = format!("{:02x}:", i);
            ip.push_str(&m);
            if n % 2 == 0 {
                ip.pop();
            }
        } 
        ip.pop();
        ip
    }
}