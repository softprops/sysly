#![feature(old_io)]
extern crate sysly;

use sysly::{ Facility, Syslog };
use std::old_io::net::ip::{ Ipv4Addr, SocketAddr };

fn main() {
  let host = SocketAddr { ip: Ipv4Addr(127,0,0,1), port: 514 };    
  let mut syslog = Syslog::udp(host).facility(Facility::LOCAL3).tag("test");
  match syslog.info("Hello syslog. I'm rust.") {
    Err(e) => panic!("error sending -- {}", e),
    Ok(_) => println!("sent")
  };
}
