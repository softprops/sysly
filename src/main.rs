#![feature(net)]
extern crate sysly;

use sysly::{ Facility, Syslog };
use std::net::{ IpAddr, SocketAddr };

fn main() {
  let host = SocketAddr::new(IpAddr::new_v4(127,0,0,1), 514);
  let mut syslog = Syslog::udp(host).facility(Facility::LOCAL3).host("foo.local").app("test");
  match syslog.info("Hello syslog. I'm rust.") {
    Err(e) => panic!("error sending -- {}", e),
    Ok(_) => println!("sent")
  };
}
