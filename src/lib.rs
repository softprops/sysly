#![feature(old_io)]
#![feature(old_path)]

extern crate time;

use std::old_io::IoError;
use std::old_io::net::udp::UdpSocket;
use std::old_io::net::ip::{ Ipv4Addr, SocketAddr };
use std::result;
use time::Tm;
use std::old_io::net::pipe::UnixStream;

// https://tools.ietf.org/html/rfc5424#section-6.2.1

pub type Result = result::Result<(), IoError>;

#[derive(Copy,Clone)]
pub enum Facility {
  KERN     = 0,
  USER     = 1 << 3,
  MAIL     = 2 << 3,
  DAEMON   = 3 << 3,
  AUTH     = 4 << 3,
  SYSLOG   = 5 << 3,
  LINEPTR  = 6 << 3,
  NEWS     = 7 << 3,
  UUCP     = 8 << 3,
  CLOCK    = 9 << 3,
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

pub enum Severity {
  EMERGENCY,
  ALERT,
  CRITICAL,
  ERROR,
  WARNING,
  NOTICE,
  INFO,
  DEBUG
}

trait Transport {
  fn send(&mut self, line: &str) -> Result;
}

impl Transport for (UdpSocket, SocketAddr) {
  fn send(&mut self, line: &str) -> Result {
    self.0.send_to(line.as_bytes(), self.1)
  }
}

impl Transport for UnixStream {
  fn send(&mut self, line: &str) -> Result {
    self.write_line(line)
  }
}

// a rust interface for syslog
pub struct Syslog {
  facility: Facility,
  tag: Option<String>,
  transport: Box<Transport>
}

impl Syslog {
   pub fn udp(host: SocketAddr) -> Syslog {
     let socket =
       match UdpSocket::bind(SocketAddr { ip: Ipv4Addr(0,0,0,0), port: 0 }) {
         Err(e) => panic!("error binding to local addr {}", e),
         Ok(s) => s
       };
      let tup = (socket, host);
      Syslog {
        facility: Facility::USER,
        tag: None,
        transport: Box::new(tup)
      }
  }

  pub fn unix(path: Path) -> Syslog {
    let stream =
      match UnixStream::connect(&path) {
        Err(_) => panic!("failed to connect to socket"),
        Ok(s)  => s
      };
    Syslog {
      facility: Facility::USER,
      tag: None,
      transport: Box::new(stream)
    }
  }

  pub fn tag(self, tag: &str) -> Syslog {
    Syslog {
      facility: self.facility,
      tag: Some(tag.to_string()),
      transport: self.transport
    }
  }

  pub fn facility(self, facility: Facility) -> Syslog {
    Syslog {
      facility: facility,
      tag: self.tag,
      transport: self.transport
    }
  }

  fn log(&mut self, severity: Severity,  msg: &str) -> Result {
    let formatted = Syslog::line(self.facility.clone(), severity, time::now(), msg);
    self.transport.send(&formatted)
  }

  pub fn debug(&mut self, msg: &str) -> Result {
    self.log(Severity::DEBUG, msg)
  }

  pub fn info(&mut self, msg: &str) -> Result {
    self.log(Severity::INFO, msg)
  }

  pub fn notice(&mut self, msg: &str) -> Result {
    self.log(Severity::NOTICE, msg)
  }

  pub fn warn(&mut self, msg: &str) -> Result {
    self.log(Severity::WARNING, msg)
  }

  pub fn err(&mut self, msg: &str) -> Result {
    self.log(Severity::ERROR, msg)
  }

  pub fn critical(&mut self, msg: &str) -> Result {
    self.log(Severity::CRITICAL, msg)
  }

  pub fn alert(&mut self, msg: &str) -> Result {
    self.log(Severity::ALERT, msg)
  }

  pub fn emergency(&mut self, msg: &str) -> Result {
    self.log(Severity::EMERGENCY, msg)
  }

  fn line(facility: Facility, severity: Severity, timestamp: Tm, msg: &str) -> String {
    format!("<{:?}> {} {}", Syslog::priority(facility, severity), timestamp.rfc3339(), msg)
  }

  // computes the priority of a message based on a facility and severity
  fn priority(facility: Facility, severity: Severity) -> u8 {
    facility as u8 | severity as u8
  }
}

#[cfg(test)]
mod tests {
  use super::{Syslog, Facility, Severity};
  use time;
  #[test]
  fn test_syslog_line() {
    let ts = time::now();
    assert_eq!(Syslog::line(Facility::LOCAL0, Severity::INFO, ts, "yo"),
               format!("<134> {} yo", ts.rfc3339()));
  }
}
