use std::{error::Error, fs::{File, self}, io::Write};

use packet_parser::*;

fn get_csv(pac: &Packet) -> String {
    // let mut csv = String::from("\n");

    let src_mac = pac.datalink().srcmac_string();
    let dst_mac = pac.datalink().dstmac_string();
    let src_ip;
    let dst_ip;
    let protocol;

    let network= pac.network().protocol();
    

    match network {
        Protocol::Arp(_) => {
            src_ip = String::new();
            dst_ip = String::new();
        },
        Protocol::Ipv4(ipv4) => {
            // println!("ipv4");
            src_ip = ipv4.src().get_addr_str();
            dst_ip = ipv4.dst().get_addr_str();
        },
        Protocol::Ipv6(ipv6) => {
            // println!("ipv6");
            src_ip = ipv6.src().get_addr_str();
            dst_ip = ipv6.dst().get_addr_str();
        },
    }

    let transport = pac.transport();
    
    protocol = match transport {
        None => "NONE".to_string(),
        Some(t) => {
            t.protocol_string()
        }
    };

    println!("{}, {}, {}, {}, {}", src_mac, dst_mac, src_ip, dst_ip, protocol);
    format!("\n{}, {}, {}, {}, {}", src_mac, dst_mac, src_ip, dst_ip, protocol)
}

pub fn log(pac: &Packet, filepath: &str) -> Result<(), Box<dyn Error>>{
    let log =  get_csv(pac);

    if !std::path::Path::new(filepath).exists() {
        let header = "src_mac, dst_mac, src_ip, dst_ip, protocol";
        let mut file = File::create(filepath)?;
        file.write_all(header.as_bytes())?;
    }

    let mut f = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(filepath)?;

    f.write_all(log.as_bytes())?;
    Ok(())
}
