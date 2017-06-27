//! A packet for joining a game. Currently no arguments are needed - there
//! is only 1 game running on the server, and the client will join that
//! game when sending the game_join packet to the server.

use net::{Packet, DeserialiseError, TAG_GAME_JOIN};

/// A packet for registration.
pub struct GameJoinPacket;

impl Packet for GameJoinPacket {
  fn serialise(&self) -> Vec<u8> {
    use std::mem::transmute;
    let mut ret = Vec::with_capacity(7);
    unsafe { ret.extend_from_slice(&transmute::<u32, [u8; 4]>(0)[..]) };
    ret.extend_from_slice(TAG_GAME_JOIN.as_bytes());
    return ret;
  }

  /// Deserialise this packet, from bytes stripped of the length and tag (first
  /// 7 bytes). 
  fn deserialise(_: &[u8]) -> Result<GameJoinPacket, DeserialiseError> {
    Ok(GameJoinPacket)
  }
}

