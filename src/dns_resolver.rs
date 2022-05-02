use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

pub const IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(223, 5, 5, 5)),
    IpAddr::V4(Ipv4Addr::new(223, 6, 6, 6)),
    IpAddr::V4(Ipv4Addr::new(119, 29, 29, 29)),
    IpAddr::V6(Ipv6Addr::new(0x2400, 0x3200, 0, 0, 0, 0, 0, 1)),
    IpAddr::V6(Ipv6Addr::new(0x2400, 0x3200, 0xbaba, 0, 0, 0, 0, 1)),
    IpAddr::V6(Ipv6Addr::new(0x2402, 0x4e00, 0, 0, 0, 0, 0, 0)),
];

pub fn lookup(host: &str) -> IpAddr {
    let resolver = Resolver::new(
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(IPS, 53, true),
        ),
        ResolverOpts::default(),
    )
    .expect("Unable to read DNS config!");
    let response = resolver.lookup_ip(host).unwrap();
    response.iter().next().expect("no addresses returned!")
}
