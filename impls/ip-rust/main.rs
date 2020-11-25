use std::{
    fmt,
    env,
    net::IpAddr,
};

use serde_json::json;

fn main() {
    let addr = env::args().skip(1).next().expect("missing input argument");

    println!("{}", check(addr));
}

fn check(input: impl AsRef<str>) -> impl fmt::Display {
    let input = input.as_ref();

    let addr: IpAddr = input.parse().expect("failed to parse IP");

    let as_v4 = match addr {
        IpAddr::V4(addr) => json!(addr.to_string()),
        IpAddr::V6(addr) => json!(addr.to_ipv4().map(|addr| addr.to_string())),
    };

    let as_v6 = match addr {
        IpAddr::V4(addr) => json!(addr.to_ipv6_mapped().to_string()),
        IpAddr::V6(addr) => json!(addr.to_string()),
    };

    json!({
        "input": addr,
        "rendered": addr.to_string(),
        "asV4": as_v4,
        "asV6": as_v6,
    })
}
