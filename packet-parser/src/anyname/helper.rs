// use generic in future
// fn concat_bits(a: &u8, b: &u8) -> u16 {
//     (a.clone() as u16) << 8 | (b.clone() as u16)
// }

#[allow(dead_code)]
pub fn concat_bits16(a: &[u8]) -> u16 {
    a.iter()
        .fold(0, |acc, x| acc << 8 | x.clone() as u16)
}

#[allow(dead_code)]
pub fn concat_bits32(a: &[u8]) -> u32 {
    a.iter()
        .fold(0, |acc, x| acc << 8 | x.clone() as u32)
}