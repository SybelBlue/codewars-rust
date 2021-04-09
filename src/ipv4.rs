type IPv4Address = (u16, u16, u16, u16);

pub fn ips_between(start: &str, end: &str) -> u32 {
    ip4_to_int(parse_ip4(end)) - ip4_to_int(parse_ip4(start))
}

fn parse_ip4(address: &str) -> IPv4Address {
    let v = String::from(address).split('.').map(|s| s.parse::<u16>().unwrap()).collect::<Vec<u16>>();
    (v[0], v[1], v[2], v[3])
}

fn ip4_to_int(address: IPv4Address) -> u32 {
    let (a0, a1, a2, a3) = address;
    a0 as u32 * 256 * 256 * 256 + a1 as u32 * 256 * 256 + a2 as u32 * 256 + a3 as u32
}
