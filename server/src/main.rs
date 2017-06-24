extern crate mio;

use std::net::SocketAddr;
use std::collections::VecDeque;
use mio::net::{UdpSocket, TcpListener, TcpStream};
use mio::{Token, Poll, Ready, PollOpt, Events};

/// A struct representing a client.
struct Client {
  /// The ID of this client.
  pub id: usize,
  /// The name of this client.
  pub name: String,

  /// The address of this client for UDP datagtrams.
  pub udp_addr: SocketAddr,
  /// A buffer of data not yet parsed by this client which arrived through UDP.
  pub udp_buf: VecDeque<u8>,

  /// The stream to write to to send TCP messages to this client.
  pub tcp_stream: TcpStream,
  /// A buffer of data not yet parsed by this client which arrived through TCP.
  pub tcp_buf: VecDeque<u8>,
}

impl Client {
  /// Function to create a new client with a name and TCP stream. The peer
  /// address of the TCP stream is used to generate a UDP address, by
  /// subtracting 1 from the value of the port.
  /// # Params
  /// * `id` - The ID of the client. Must be unique.
  /// * `name` - The name of this client - the client should pass this through
  ///            the TCP stream to 'register'.
  /// * `tcp_stream` - The TCP stream linked to the client.
  pub fn new(id: usize, name: &str, tcp_stream: TcpStream) -> Client {
    // UDP address port is always 1 below the TCP address, so get the udp address
    let mut udp_addr = tcp_stream.peer_addr().unwrap();
    let udp_port = udp_addr.port() - 1;
    udp_addr.set_port(udp_port);

    // Create and return the client object
    Client {
      id: id,
      name: name.to_owned(),
      udp_addr: udp_addr,
      udp_buf: VecDeque::new(),
      tcp_stream: tcp_stream,
      tcp_buf: VecDeque::new(),
    }
  }

  /// A function to check whether there are any packets to parse in the tcp or
  /// udp buffer.
  pub fn try_parse_packets(&mut self) {
    use std::mem::transmute;
    // Check TCP
    loop {
      if self.tcp_buf.len() < 7 { break; }

      let packet_size_bytes = [self.tcp_buf[0], self.tcp_buf[1], self.tcp_buf[2], self.tcp_buf[3]];
      let packet_type = [self.tcp_buf[4], self.tcp_buf[5], self.tcp_buf[6]];
      let packet_size = unsafe { transmute::<[u8; 4], u32>(packet_size_bytes) };
      if self.tcp_buf.len() - 7 < packet_size as usize { break; }
      // Consume these bytes, and get the body of the packet
      let packet_body = self.tcp_buf.drain(0..(packet_size+7) as usize)
        .skip(7).collect::<Vec<u8>>();
      if packet_type[..] == *("reg".as_bytes()) {
        use std::str::from_utf8;
        println!("Received a registration packet with data {}", from_utf8(&packet_body[..]).unwrap());
      }
    }
  }
}

pub enum DeserialiseError {
  DataBad,
  DataIncomplete,
}

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

  /// Deserialise this packet, from bytes stripped of the length and tag (first
  /// 7 bytes). 
  pub fn deserialise(buf: &[u8]) -> Result<RegPacket, DeserialiseError> {
    use std::str::from_utf8;
    let name = try!(from_utf8(buf).map_err(|_| DeserialiseError::DataBad));
    Ok(RegPacket::new(name))
  }
}

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