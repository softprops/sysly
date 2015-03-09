# sysly

[![Build Status](https://travis-ci.org/softprops/sysly.svg?branch=master)](https://travis-ci.org/softprops/sysly)

> syslog, srsly

a [syslog](https://tools.ietf.org/html/rfc5424) `udp` and `unix domain socket` appender.

## install

Add the following to your `Cargo.toml`

```toml
[dependencies]
sysly = "0.1.0"
```

## usage

The interface is straight forward. First create a new `Syslog` instance optionally configuring with a
`Facility` and `tag`, then start logging messages with methods which correlate to severities including: 
`debug`, `info`, `notice`, `warn`, `err`, `critical`, `alert`, and `emergency`.

```rust
#![feature(net)]
extern crate sysly;

use sysly::{ Facility, Syslog };
use std::net::{ IpAddr, SocketAddr };

fn main() {
  let host = SocketAddr::new(IpAddr::new_v4(127,0,0,1), 514);
  let mut syslog = Syslog::udp(host).facility(Facility::LOCAL3).host("foo.local").app("test");
  syslog.info("Hello syslog. I'm rust. Please to meet you")
}
```

Doug Tangren (softprops) 2015
