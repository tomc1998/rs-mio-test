use net::{Packet, DeserialiseError};

/// A packet for registration.
pub struct RegPacket {
  pub name: String,
}
impl RegPacket {
  pub fn new(name: &str) -> RegPacket {
    RegPacket { name: name.to_owned() }
  }
}

impl Packet for RegPacket {
  fn serialise(&self) -> Vec<u8> {
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
  fn deserialise(buf: &[u8]) -> Result<RegPacket, DeserialiseError> {
    use std::str::from_utf8;
    let name = try!(from_utf8(buf).map_err(|_| DeserialiseError::DataBad));
    Ok(RegPacket::new(name))
  }
}

