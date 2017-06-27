#[macro_use]
extern crate glium;
extern crate specs;
extern crate nalgebra;
extern crate common;
extern crate time;

#[allow(dead_code)]
mod renderer;
#[allow(dead_code)]
mod component;
#[allow(dead_code)]
mod state;

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
  let mut renderer = renderer::Renderer::new(&display);

  let mut global_state = state::GlobalState { delta: 0, prev_time: time::precise_time_ns() };

  // Create ECS
  let mut planner : specs::Planner<state::GlobalState> = {
    use component::*;
    let mut w = specs::World::new();
    w.register::<CompAABB>();
    w.register::<CompBody>();
    w.register::<CompColor>();
    w.create_now().with(CompAABB([0.0, 0.0, 32.0, 32.0]))
      .with(CompColor([0.0, 1.0, 0.0, 1.0]))
      .with(CompBody{vel: [0.0, 0.0], acc: [0.5, 0.3], mass: 5.0, flags: BODY_GRAVITY})
      .build();
    specs::Planner::new(w)
  };

  planner.add_system::<renderer::SysRenderer>(renderer::SysRenderer::new(&renderer), "render", 0);

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

  loop {
    // Check input
    for ev in display.poll_events() {
      use glium::glutin::Event;
      match ev {
        Event::Closed => return,
          _ => ()
      }
    }

    // Calculate frame delta, store in global state object
    global_state.delta = time::precise_time_ns() - global_state.prev_time;
    global_state.prev_time = time::precise_time_ns();

    // Dispatch ECS with the global state object
    planner.dispatch(global_state.clone());
    planner.wait();

    // Receive any vertex data sent by the ECS
    renderer.recv_data();

    // Render everything
    use glium::Surface;
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 1.0);
    renderer.render(&mut frame);
    frame.finish().unwrap();
  }
}

