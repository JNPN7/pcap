use libc::{AF_PACKET, SOCK_RAW, socket, exit, perror};
use nix::sys::socket::{recv, MsgFlags};
use std::ffi::CString;

pub fn sniff() {
    let mut buffer: [u8; 2048] = [0; 2048];
    
    let sock = unsafe {
        socket(AF_PACKET, SOCK_RAW, 8)
    };

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
        let no_of_bytes = match recv(sock, &mut buffer, MsgFlags::empty()) {
            Ok(val) => val,
            Err(val) => {
                println!("Error: {:?}", val);
                continue;
            }
        };
        for i in 0..no_of_bytes {
            print!("{:X} ", buffer[i]);
        }
        println!("\n-----------");
    }
}