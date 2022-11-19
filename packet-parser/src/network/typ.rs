#[derive(Debug)]
pub enum NetworkType {
    Ipv4,
    Ipv6,
    Arp,
    Vlan,
    MplsUni,
    MplsMulti,
    Rarp
}


// he&lper function to find the type
// Name is not standard for now need to change
pub fn find_type(typ: &[u8]) -> NetworkType {
    match typ {
        [8, 0] => NetworkType::Ipv4, // (0x0800)
        [134, 221] => NetworkType::Ipv6, // (0x86dd)
        [8, 6] => NetworkType::Arp, //
        [129, 0] => NetworkType::Vlan, //
        [136, 71] => NetworkType::MplsUni,
        [136, 72] => NetworkType::MplsMulti,
        [128, 53] => NetworkType::Rarp,
        _ => panic!("NetworkType not found Datalink layer | network/typ | typ => [{}, {}]", &typ[0], &typ[1])
    }
}