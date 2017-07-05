extern crate mio;
extern crate common;

mod client;

use client::Client;
use mio::net::{UdpSocket, TcpListener};
use mio::{Token, Poll, Ready, PollOpt, Events};

fn main() {
  // A list of clients.
  let mut client_list : Vec<Client> = Vec::new();

  // Set up a token to identify the UDP socket can be read.
  const TCP: Token = Token(0);
  const UDP: Token = Token(1);

  let udp_addr = "127.0.0.1:12345".parse().unwrap();
  let tcp_addr = "127.0.0.1:12346".parse().unwrap();

  // Setup the server socket
  let udp_server = UdpSocket::bind(&udp_addr).unwrap();
  let tcp_server = TcpListener::bind(&tcp_addr).unwrap();

  // Create a poll instance
  let poll = Poll::new().unwrap();

  // Start listening for incoming connections
  poll.register(&tcp_server, TCP, Ready::readable(), PollOpt::edge()).unwrap();
  poll.register(&udp_server, UDP, Ready::readable(), PollOpt::edge()).unwrap();

  // Create storage for events
  let mut events = Events::with_capacity(1024);

  let mut curr_client_id = 2;

  loop {
    poll.poll(&mut events, None).unwrap();

    for event in events.iter() {
      match event.token() {
        TCP => { 
          // Accept a socket connection, and add it to the list of clients.
          let stream = tcp_server.accept().unwrap().0;
          let id = curr_client_id;
          curr_client_id += 1;
          client_list.push(Client::new(id, "", stream));

          // Register poll to listen for this new TCP stream
          let c = client_list.last().unwrap();
          poll.register(&c.tcp_stream, Token(id), Ready::readable(), PollOpt::edge()).unwrap();
        }
        UDP => { // Received a UDP message
          println!("Received a UDP datagram"); 
        }
        Token(x) => { // Received a TCP message from client with ID x
          // Find the client this refers to
          let mut client = None;
          for c in &mut client_list {
            if c.id == x { client = Some(c); break; }
          }
          if client.is_none() { continue; }
          let client = client.unwrap();

          // Read messages from this TCP stream, and add to the tcp data queue
          // for this client
          use std::io::Read;
          let mut buf = Vec::new();
          client.tcp_stream.read_to_end(&mut buf).unwrap();
          client.tcp_buf.extend(buf.iter());
        }
      }
    }

    for c in &mut client_list {
      c.try_parse_packets();
    }
  }
}
