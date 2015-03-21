extern crate sysly;

use sysly::{ Facility, Syslog };
use std::net::{ Ipv4Addr, SocketAddr, SocketAddrV4 };

#[cfg(not(test))]
fn main() {
  let host = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 514));
  let mut syslog = Syslog::udp(host).facility(Facility::LOCAL3).host("foo.local").app("test");
  match syslog.info("Hello syslog. I'm rust.") {
    Err(e) => panic!("error sending -- {}", e),
    Ok(_) => println!("sent")
  };
}
