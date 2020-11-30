#![feature(ip)]

use std::net::IpAddr;
use serde_json::json;

fn main() {
    let input = std::env::args().skip(1).next().expect("missing input argument");
    let addr: IpAddr = input.parse().expect("failed to parse IP");

    let data = json!({
        "asV4": match addr {
            IpAddr::V4(addr) => json!(addr.to_string()),
            IpAddr::V6(addr) => json!(addr.to_ipv4().map(|addr| addr.to_string())),
        },
        "asV6": match addr {
            IpAddr::V4(addr) => json!(addr.to_ipv6_mapped().to_string()),
            IpAddr::V6(addr) => json!(addr.to_string()),
        },
    });

    println!("{}", data);
}
