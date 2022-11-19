use libc::{AF_PACKET, SOCK_RAW, socket, exit, perror};
use nix::sys::socket::{recv, MsgFlags};
use std::ffi::CString;
use packet_parser::*;
use super::logger::log;

pub fn sniff(log_file_path: &str) {
    let mut buffer: [u8; 2048] = [0; 2048];
    
    // socket(AF_INET6, SOCK_DGRAM, 17)
    // socket(AF_PACKET, SOCK_RAW, 8)
    // socket(AF_PACKET, SOCK_RAW, 768)
    // socket(AF_PACKET, SOCK_RAW, IPPROTO_TCP)
    // socket(AF_INET6 , SOCK_RAW , IPPROTO_TCP)
    let sock = unsafe {
        socket(AF_PACKET, SOCK_RAW, 768)
    };

    if sock < 0 {
        println!("Error occured...");
        unsafe {
            let error_msg = CString::new("Socket").expect("CString::new failed");
            perror(error_msg.as_ptr());
            exit(1);
        }
    }

    loop {
        let no_of_bytes = match recv(sock, &mut buffer, MsgFlags::empty()) {
            Ok(val) => val,
            Err(val) => {
                println!("Error: {:?}", val);
                continue;
            }
        };

        let packet: Packet = Packet::new(&buffer[..no_of_bytes]);
        
        log(&packet, &log_file_path).expect("error error danger");
    }
}