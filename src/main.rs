use libc::{AF_PACKET, SOCK_RAW, socket, exit, perror, SIOCGIFFLAGS, AF_INET6, SOCK_DGRAM, IPPROTO_TCP, AF_INET};
use libc::{SOL_SOCKET, SO_BINDTODEVICE, setsockopt};
use libc::ioctl;
// use nix::NixPath;
use nix::sys::socket::{recv, MsgFlags, recvfrom};
use std::ffi::CString;
// use ifstructs::ifreq;

fn main() {

    
    sniff();
    println!("Hello, world!");
}

pub fn sniff() {
    let mut buffer: [u8; 2048] = [0; 2048];

    println!("{:?}", (IPPROTO_TCP, SOCK_RAW, AF_PACKET));
    
    let sock = unsafe {
        // socket(AF_INET6, SOCK_DGRAM, 17)
        // socket(AF_PACKET, SOCK_RAW, 8)
        socket(AF_PACKET, SOCK_RAW, 768)
        // socket(AF_PACKET, SOCK_RAW, IPPROTO_TCP)
        // socket(AF_INET6 , SOCK_RAW , IPPROTO_TCP)
    };

    // let ooo = "wlp3s0";

// let opt = "wlsp30" as *const char;
    // let or = CString::new("enp2s0f1").expect("Erroe");
    let or = "wlp3s0";
    // let opt = or.as_ref() ;
    unsafe {
        let opt = setsockopt(sock, SOL_SOCKET, SO_BINDTODEVICE, or.as_ptr() as *const _, or.len() as u32);
        if opt < 0 {
            println!("Error occured...");
            // unsafe {
                let error_msg = CString::new("Socket").expect("CString::new failed");
                perror(error_msg.as_ptr());
                exit(1);
            // }
        }
        println!("{}", opt);
    }
    // setsockopt(socket, sockopt::, SO_BINDTODEVICE, "wlp3s0", 6);
    
    unsafe{
        ioctl(sock, SIOCGIFFLAGS, );
    }

    if sock < 0 {
        println!("Error occured...");
        unsafe {
            let error_msg = CString::new("Socket").expect("CString::new failed");
            perror(error_msg.as_ptr());
            exit(1);
        }
    }
    // println!("{}", sock);

    loop {
        println!("domedome");
        let no_of_bytes = match recv(sock, &mut buffer, MsgFlags::empty()) {
            Ok(val) => val,
            Err(val) => {
                println!("Error: {:?}", val);
                continue;
            }
        };
        // let nob = recvfrom(sock, &mut buffer);
        // println!("{:?}", nob);
        // for i in 0..no_of_bytes {
        //     print!("{:X} ", buffer[i]);
        // }
        let pac: Packet = Packet::new(no_of_bytes, &buffer); 
        // println!("{:?}", (pac.payload));
        pac.header.mac();
        pac.header.ip();
        println!("\n-----------");
    }
}

struct Packet{
    header: Header,
    payload: Vec<u8>
}

impl Packet {
    fn new(no_of_bytes: usize, pac: &[u8]) -> Self{
        let header: Header = Header::new(pac);
        let mut payload: Vec<u8> = Vec::new();

        for i in 10..no_of_bytes {
            payload.push(pac[i]);
        }
        
        Self { 
            header: header,
            payload: payload
        }
    } 
}

struct Header {
    dest_mac: [u8; 6],
    source_mac: [u8; 6],
    typ: [u8; 2],
    source_ip: [u8; 4],
    dest_ip: [u8; 4]
    
}

impl Header {
   fn new(pac: &[u8]) -> Self{
        let mut dest_mac: [u8; 6] = [0; 6];
        dest_mac.clone_from_slice(&pac[0..6]);

        let mut source_mac: [u8; 6] = [0; 6];
        source_mac.clone_from_slice(&pac[6..12]);

        let mut typ: [u8; 2] = [0; 2];
        typ.clone_from_slice(&pac[12..14]);

        let mut source_ip: [u8; 4] = [0; 4];
        source_ip.clone_from_slice(&pac[26..30]);

        let mut dest_ip: [u8; 4] = [0; 4];
        dest_ip.clone_from_slice(&pac[30..34]);
        
        Self { 
            dest_mac: dest_mac, 
            source_mac: source_mac,
            typ: typ, 
            source_ip: source_ip,
            dest_ip: dest_ip
        }
   }
   fn mac(&self) {
       print!("Destination mac: ");
       for i in 0..6 {
           print!("{:X} ", &self.dest_mac[i]);
       }
       print!("\nSource mac: ");
       for i in 0..6 {
           print!("{:X} ", &self.source_mac[i]);    
       }
       print!("\nType: {} {}", &self.typ[0], &self.typ[1]);
   }
   fn ip(&self) {
       print!("\nSource ip: ");
       for i in 0..4 {
           print!("{} ", &self.source_ip[i]);
       }
       print!("\nDestination ip: ");
       for i in 0..4 {
           print!("{} ", &self.dest_ip[i]);
       }

   }
}