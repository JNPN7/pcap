use super::datalink::Datalink;
use super::network::{Network, protocols::Protocol};
use super::transport::Transport;
use super::application::Application;

#[allow(dead_code)]
pub struct Packet {
    datalink: Datalink,
    network: Network,
    transport: Option<Transport>,
    application: Option<Application>
}


impl Packet {
    pub fn new(pac: &[u8]) -> Self {
        let mut start: usize = 0;

        let datalink = Datalink::new(&pac[start..14]);
        
        start += datalink.header_len();

        let network = Network::new(&pac[start..], datalink.typ());

        start += network.header_len();

        let transport: Option<Transport> = match network.protocol() {
            Protocol::Arp(_) => {
                None
            },
            Protocol::Ipv4(ipv4) => {
                Some(Transport::new(&pac[start..], ipv4.protocol()))
            },
            Protocol::Ipv6(ipv6) => {
                Some(Transport::new(&pac[start..], ipv6.nxt_header()))
            },
        };
        
        let application: Option<Application> = match &transport {
            None => None,
            Some(t) => {
                start += t.header_len();
                let app = if pac.len() > start+1 {
                    Some(Application::new(&pac[start..]))
                } else { None };
                app
            }
        };

        Self { 
            datalink, 
            network, 
            transport,
            application 
        }
    }

    pub fn datalink(&self) -> &Datalink {
        &self.datalink
    }

    pub fn network(&self) -> &Network {
        &self.network
    }

    pub fn transport(&self) -> &Option<Transport> {
        &self.transport
    }

    pub fn application(&self) -> &Option<Application> {
        &self.application
    }
}
