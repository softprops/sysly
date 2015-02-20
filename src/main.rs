#![feature(io)]
//#![feature(path)]

extern crate time;

use std::old_io::net::udp::UdpSocket;
use std::old_io::net::ip::{ Ipv4Addr, SocketAddr };
//use std::old_io::net::pipe::UnixStream;

fn main() {
  let msg = format!("{} hi from Rust", time::now().rfc3339());

  let addr = SocketAddr { ip: Ipv4Addr(127,0,0,1), port: 514 };    
  let client_addr = SocketAddr { ip: Ipv4Addr(0,0,0,0), port: 0 };
  let mut udp = match UdpSocket::bind(client_addr) {
    Err(e) => panic!("error binding to {} -- {}", addr, e),
    Ok(s) => s
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

  let udp_result = udp.send_to(msg.as_bytes(), addr);
  match udp_result {
      Err(e) => panic!("err {}", e),
      Ok(_) => println!("upd send result ok")
  };
}
