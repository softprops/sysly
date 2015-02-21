#![feature(old_io)]
//#![feature(path)]

extern crate time;

use std::old_io::IoError;
use std::old_io::net::udp::UdpSocket;
use std::old_io::net::ip::{ Ipv4Addr, SocketAddr };
//use std::old_io::net::pipe::UnixStream;


struct Syslog {
  host: SocketAddr,
  socket: UdpSocket
}

impl Syslog {
   fn new(host: SocketAddr) -> Syslog {
     Syslog {
       host: host,
       socket: match UdpSocket::bind(SocketAddr { ip: Ipv4Addr(0,0,0,0), port: 0 }) {
         Err(e) => panic!("error binding to local addr {}", e),
         Ok(s) => s
       }
     }
   }

   fn log(&mut self, msg: &str) -> Result<(), IoError> {
     self.socket.send_to(msg.as_bytes(), self.host)
   }
}

fn main() {
  let msg = format!("<19>5 {} mbp hi from Rust", time::now().rfc3339());
  println!("msg -> {}", msg);
  let host = SocketAddr { ip: Ipv4Addr(127,0,0,1), port: 514 };    
  let mut syslog = Syslog::new(host);
  match syslog.log(&msg) {
    Err(e) => panic!("error sending -- {}", e),
    Ok(_) => println!("sent")
  };
  //let path = Path::new("unix:///var/run/syslog");
  //let mut stream = match UnixStream::connect(&path) {
  //  Err(e) => panic!("error connecting to path -- {}", e),
  //  Ok(s) => s,
  //};

  //let stream_result = stream.write_line(
  //  "2012-07-22T00:06:26-00:00 somehost Rust console 2 Hi from Rust");
  //match stream_result {
  //    Err(e) => panic!("err {}", e),
  //    Ok(_) => println!("stream send result ok")
  //};

 
}
