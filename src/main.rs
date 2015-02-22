#![feature(old_io)]
#![feature(old_path)]
#![feature(box_syntax)]
#![feature(box_patterns)]

extern crate time;

use std::old_io::IoError;
use std::old_io::net::udp::UdpSocket;
use std::old_io::net::ip::{ Ipv4Addr, SocketAddr };
use time::Tm;
use std::old_io::net::pipe::UnixStream;

// https://tools.ietf.org/html/rfc5424#section-6.2.1

enum Facility {
  KERN     = 0 << 3,
  USER     = 1 << 3,
  MAIL     = 2 << 3,
  DAEMON   = 3 << 3,
  AUTH     = 4 << 3,
  SYSLOG   = 5 << 3,
  LPR      = 6 << 3,
  NEWS     = 7 << 3,
  UUCP     = 8 << 3,
  CRON     = 9 << 3,
  AUTHPRIV = 10 << 3,
  FTP      = 11 << 3,
  LOCAL0   = 16 << 3,
  LOCAL1   = 17 << 3,
  LOCAL2   = 18 << 3,
  LOCAL3   = 19 << 3,
  LOCAL4   = 20 << 3,
  LOCAL5   = 21 << 3,
  LOCAL6   = 22 << 3,
  LOCAL7   = 23 << 3
}

enum Severity {
  EMERG,
  ALERT,
  CRIT,
  ERR,
  WARNING,
  NOTICE,
  INFO,
  DEBUG
}

trait Transport {
  fn send(&mut self, line: &str) -> Result<(), IoError>;
}

impl Transport for (UdpSocket, SocketAddr) {
  fn send(&mut self, line: &str) -> Result<(), IoError> {
    self.0.send_to(line.as_bytes(), self.1)
  }
}

impl Transport for UnixStream {
  fn send(&mut self, line: &str) -> Result<(), IoError> {
    self.write_line(line)
  }
}

struct Syslog {
  transport: Box<Transport>
}

impl Syslog {
   fn udp(host: SocketAddr) -> Syslog {
     let socket =
       match UdpSocket::bind(SocketAddr { ip: Ipv4Addr(0,0,0,0), port: 0 }) {
         Err(e) => panic!("error binding to local addr {}", e),
         Ok(s) => s
       };
      let tup = (socket, host);
      Syslog {
        transport: box tup
      }
  }

  fn unix(path: Path) -> Syslog {
    let stream =
      match UnixStream::connect(&path) {
        Err(_) => panic!("failed to connect to socket"),
        Ok(s)  => s
      };
    Syslog {
       transport: box stream
    }
  }

  fn log(&mut self, facility: Facility, severity: Severity,  msg: &str) -> Result<(), IoError> {
    let formatted = Syslog::line(facility, severity, time::now(), msg);
    println!("sending {}", formatted);
    (*self.transport).send(&formatted)
  }

  fn debug(&mut self, msg: &str) -> Result<(), IoError> {
     self.log(Facility::LOCAL3, Severity::DEBUG, msg)
  }

  fn info(&mut self, msg: &str) -> Result<(), IoError> {
     self.log(Facility::LOCAL3, Severity::INFO, msg)
  }

  fn notice(&mut self, msg: &str) -> Result<(), IoError> {
     self.log(Facility::LOCAL3, Severity::NOTICE, msg)
  }

  fn line(facility: Facility, severity: Severity, timestamp: Tm, msg: &str) -> String {
    format!("<{:?}> {} {}", Syslog::priority(facility, severity), timestamp.rfc3339(), msg)
  }

  // computes the priority of a message based on a facility and severity
  fn priority(facility: Facility, severity: Severity) -> u8 {
    facility as u8 | severity as u8
  }
}

fn main() {
  let host = SocketAddr { ip: Ipv4Addr(127,0,0,1), port: 514 };    
  let mut syslog = Syslog::udp(host);
  match syslog.info("Hello syslog. I'm rust.") {
    Err(e) => panic!("error sending -- {}", e),
    Ok(_) => println!("sent")
  };
}
