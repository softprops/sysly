#![feature(old_io)]
#![feature(old_path)]

extern crate time;

use std::old_io::IoError;
use std::old_io::net::udp::UdpSocket;
use std::old_io::net::ip::{ Ipv4Addr, SocketAddr };
use std::result;
use time::Tm;
use std::old_io::net::pipe::UnixStream;

/// A type alias for `Result<(), IoError>`, the result of writing a log message
pub type Result = result::Result<(), IoError>;

static NIL: &'static str = "-";

/// Syslog Facilites as defined by [rfc5424](https://tools.ietf.org/html/rfc5424#page-10)
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

/// Syslog Severity as defined by [rfc5424](https://tools.ietf.org/html/rfc5424#page-11)
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

/// A rust interface for Syslog, a standard unix system logging service
pub struct Syslog {
  /// A Syslog facility to target when logging
  facility: Facility,
  /// A Syslog host entry as defined by
  /// [rfc5424#section-6.2.4](https://tools.ietf.org/html/rfc5424#section-6.2.4)
  host: Option<String>,
  /// An optional app-name appended to Syslog messages as defined by 
  /// [rfc5424#section-6.2.5](https://tools.ietf.org/html/rfc5424#section-6.2.5)
  app: Option<String>,
  /// An optional proc-id appended to Syslog messages as defined by
  /// [rfc5424#section-6.2.6](https://tools.ietf.org/html/rfc5424#section-6.2.6)
  pid: Option<String>,
  /// An optional msg-id appended to Syslog messages as defined by 
  /// [rfc5424#section-6.2.7](https://tools.ietf.org/html/rfc5424#section-6.2.7)
  msgid: Option<String>,
  transport: Box<Transport>
}

impl Syslog {
   /// Factory for a Syslog appender that writes to
   /// remote Syslog daemon listening a SocketAddr
   pub fn udp(host: SocketAddr) -> Syslog {
     let socket =
       match UdpSocket::bind(SocketAddr { ip: Ipv4Addr(0,0,0,0), port: 0 }) {
         Err(e) => panic!("error binding to local addr {}", e),
         Ok(s) => s
       };
      let tup = (socket, host);
      Syslog {
        facility: Facility::USER,
        host: None,
        app: None,
        pid: None,
        msgid: None,
        transport: Box::new(tup)
      }
  }

  /// Same as udp with providing local loopback address with the standard syslog port
  pub fn localudp() -> Syslog {
    Syslog::udp(SocketAddr { ip: Ipv4Addr(127,0,0,1), port: 514 })
  }

  /// Factory for a Syslog appender that writes
  /// to a host-local Syslog daemon listening on a unix socket domain
  /// hosted at the given Path
  pub fn unix(path: Path) -> Syslog {
    let stream =
      match UnixStream::connect(&path) {
        Err(_) => panic!("failed to connect to socket"),
        Ok(s)  => s
      };
    Syslog {
      facility: Facility::USER,
      host: None,
      app: None,
      pid: None,
      msgid: None,
      transport: Box::new(stream)
    }
  }
  /// Returns a new Syslog appender configured to append with
  /// the provided Facility
  pub fn facility(self, facility: Facility) -> Syslog {
    Syslog {
      facility: facility,
      host: self.host,
      app: self.app,
      pid: self.pid,
      msgid: self.msgid,
      transport: self.transport
    }
  }

  /// Returns a new Syslog appender configured to append with
  /// the provided host addr
  pub fn host(self, local: &str) -> Syslog {
    Syslog {
      facility: self.facility,
      host: Some(local.to_string()),
      app: self.app,
      pid: self.pid,
      msgid: self.msgid,
      transport: self.transport
    }
  }

  /// Returns a new Syslog appender, configured to append with
  /// the provided app-name
  pub fn app(self, app: &str) -> Syslog {
    Syslog {
      facility: self.facility,
      host: self.host,
      app: Some(app.to_string()),
      pid: self.pid,
      msgid: self.msgid,
      transport: self.transport
    }
  }

  pub fn pid(self, pid: &str) -> Syslog {
    Syslog {
      facility: self.facility,
      host: self.host,
      app: self.app,
      pid: Some(pid.to_string()),
      msgid: self.msgid,
      transport: self.transport
    }
  }

  pub fn msgid(self, id: &str) -> Syslog {
    Syslog {
      facility: self.facility,
      host: self.host,
      app: self.app,
      pid: self.pid,
      msgid: Some(id.to_string()),
      transport: self.transport
    }
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

  fn log(&mut self, severity: Severity,  msg: &str) -> Result {
    let formatted = Syslog::line(
        self.facility.clone(), severity, time::now(), self.host.clone(), self.app.clone(), self.pid.clone(), self.msgid.clone(), msg);
    self.transport.send(&formatted)
  }

  fn line(facility: Facility, severity: Severity, timestamp: Tm, host: Option<String>, app: Option<String>, pid: Option<String>, msgid: Option<String>, msg: &str) -> String {
    format!(
      "<{:?}>1 {} {} {} {} {} {}",
        Syslog::priority(facility, severity),
        timestamp.rfc3339(),
        host.unwrap_or(NIL.to_string()),
        app.unwrap_or(NIL.to_string()),
        pid.unwrap_or(NIL.to_string()),
        msgid.unwrap_or(NIL.to_string()),
        msg)
  }

  // computes the priority of a message based on a facility and severity
  fn priority(facility: Facility, severity: Severity) -> u8 {
    facility as u8 | severity as u8
  }
}

#[cfg(test)]
mod tests {
  use super::{Syslog, Facility, NIL, Severity};
  use time;
  #[test]
  fn test_syslog_line_defaults() {
    let ts = time::now();
    assert_eq!(Syslog::line(
      Facility::LOCAL0, Severity::INFO, ts, None, None, None, None, "yo"),
      format!("<134>1 {} - - - - yo", ts.rfc3339()));
  }

  #[test]
  fn test_syslog_line_host() {
    let ts = time::now();
    let host = "foo.local";
    assert_eq!(Syslog::line(
      Facility::LOCAL0, Severity::INFO, ts, Some(host.to_string()), None, None, None, "yo"),
      format!("<134>1 {} {} - - - yo", ts.rfc3339(), host));
  }

  #[test]
  fn test_syslog_line_app() {
    let ts = time::now();
    let app = "sysly";
    assert_eq!(Syslog::line(
      Facility::LOCAL0, Severity::INFO, ts, None, Some(app.to_string()), None, None, "yo"),
      format!("<134>1 {} - {} - - yo", ts.rfc3339(), app));
  }

  #[test]
  fn test_syslog_line_pid() {
    let ts = time::now();
    let pid = "16";
    assert_eq!(Syslog::line(
      Facility::LOCAL0, Severity::INFO, ts, None, None, Some(pid.to_string()), None, "yo"),
      format!("<134>1 {} - - {} - yo", ts.rfc3339(), pid));
  }

  #[test]
  fn test_syslog_line_msgid() {
    let ts = time::now();
    let msgid = "TCPIN";
    assert_eq!(Syslog::line(
      Facility::LOCAL0, Severity::INFO, ts, None, None, None, Some(msgid.to_string()), "yo"),
      format!("<134>1 {} - - - {} yo", ts.rfc3339(), msgid));
  }
}
