# sysly

> syslog, srsly

a [syslog](https://tools.ietf.org/html/rfc5424) `udp` and `unix domain socket` appender.


## usage

The interface is straight forward. First create a new `Syslog` instance optionally configuring with a
`Facility` and `tag`, then start logging messages with methods which correlate to severities including: 
`debug`, `info`, `notice`, `warn`, `err`, `critical`, `alert`, and `emergency`.

```rust
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
```

Doug Tangren (softprops) 2015
