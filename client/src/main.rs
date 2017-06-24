#[macro_use]
extern crate glium;
extern crate specs;
extern crate nalgebra;

mod renderer;
mod component;

use std::io::prelude::*;
use std::net::{TcpStream, UdpSocket, SocketAddr};

/// A packet for registration.
pub struct RegPacket {
  pub name: String,
}
impl RegPacket {
  pub fn new(name: &str) -> RegPacket {
    RegPacket { name: name.to_owned() }
  }

  pub fn serialise(&self) -> Vec<u8> {
    use std::mem::transmute;
    let payload = &self.name;
    let payload_len = payload.len() as u32;
    let mut ret = Vec::with_capacity(payload_len as usize + 7);
    unsafe { ret.extend_from_slice(&transmute::<u32, [u8; 4]>(payload_len)[..]) };
    ret.extend_from_slice("reg".as_bytes());
    ret.extend_from_slice(&payload.as_bytes());
    return ret;
  }
}

use glium::backend::glutin_backend::GlutinFacade;

fn setup_display() -> GlutinFacade {
  use glium::DisplayBuild;
  glium::glutin::WindowBuilder::new().build_glium().unwrap()
}

fn main() {
  let display = setup_display();
  let renderer = renderer::Renderer::new(&display);
  let this_addr : SocketAddr = "127.0.0.1:0".parse().unwrap();

  let server_udp_addr : SocketAddr = "127.0.0.1:12345".parse().unwrap();
  let server_tcp_addr : SocketAddr = "127.0.0.1:12346".parse().unwrap();

  {
    // Send some UDP data
    let socket = UdpSocket::bind(this_addr).unwrap();
    socket.connect(server_udp_addr).unwrap(); // Connect to the server on localhost
    socket.send(&[65, 0]).unwrap();
  }

  {
    // Connect to the TCP listener
    let mut stream = TcpStream::connect(server_tcp_addr).unwrap();
    // Register us with the name 'John'
    let reg_packet = RegPacket::new("John");
    stream.write(&reg_packet.serialise()).unwrap();
  }
}

