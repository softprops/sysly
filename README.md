# sysly

[![Build Status](https://travis-ci.org/softprops/sysly.svg?branch=master)](https://travis-ci.org/softprops/sysly)

> syslog, srsly

a [syslog](https://tools.ietf.org/html/rfc5424) `udp` and `unix domain socket` appender.

## install

Add the following to your `Cargo.toml`

```toml
[dependencies]
sysly = "0.2.0"
```

## usage

The interface is straight forward. First create a new `Syslog` instance optionally configuring with a
`Facility` and `tag`, then start logging messages with methods which correlate to severities including: 
`debug`, `info`, `notice`, `warn`, `err`, `critical`, `alert`, and `emergency`.

```rust
extern crate sysly;

use sysly::{ Facility, Syslog };
use std::net::{ Ipv4Addr, SocketAddr, SocketAddrV4 };

#[cfg(not(test))]
fn main() {
  let host = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 514));
  let mut syslog = Syslog::udp(host).facility(Facility::LOCAL3).host("foo.local").app("test");
  syslog.info("Hello syslog. I'm rust. Pleased to meet you")
}
```

Doug Tangren (softprops) 2015
