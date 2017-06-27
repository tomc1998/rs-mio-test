#[macro_use]
extern crate glium;
extern crate specs;
extern crate nalgebra;
extern crate common;

mod renderer;
mod component;

use std::io::prelude::*;
use std::net::{TcpStream, UdpSocket, SocketAddr};
use glium::backend::glutin_backend::GlutinFacade;
use common::net::{Packet, RegPacket};

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

