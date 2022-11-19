mod logger;
mod sniffer;
use std::env::{args};

fn main() {
    let mut arguments = args();
    arguments.next();
    let log_file_path: String = match arguments.next() {
        Some(args) => args,
        None => panic!("File path of log path missing")
    };
    sniffer::sniff(&log_file_path);
}


// fn test() {
//     let a = [1, 2, 3, 4, 5];
//     let res = a.iter()
//         .fold(0, |acc: i32, x| {
//             println!("{}, {}", acc, x);
//             acc + x
//         });
//     println!("{}", res);
// }