//! A module for representing connected clients in memory, and storing their
//! associated data.

/// A struct representing a client.
pub struct Client {
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
        let reg_packet = RegPacket::deserialise(&packet_body[..]).ok().unwrap();
        println!("Received reg packet with name \"{}\"", reg_packet.name);
      }
    }
  }
}
