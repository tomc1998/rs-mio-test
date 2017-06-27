//! A module for packets shared across the network, plus serialisation /
//! deserialisation methods.

mod reg;
mod game_join;

pub use self::reg::RegPacket;
pub use self::game_join::GameJoinPacket;

use std::{fmt, error};

/// A class for errors when deserialising bytes into a packet.
#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub enum DeserialiseError {
  DataBad,
}

impl fmt::Display for DeserialiseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:?}", self)
  }
}

impl error::Error for DeserialiseError {
  fn description(&self) -> &str {
    match *self {
      DeserialiseError::DataBad => r#"The data was of bad format, or was
        incomplete and impossible to parse in some way - i.e. a string was
        given with a length, but the data was not long enough for the entire
        string to be contained."#,
    }
  }
  fn cause(&self) -> Option<&error::Error> { None }
}

/// A trait for packets to implement, guaranteeing the serialise / deserialise
/// methods.
pub trait Packet {
  fn serialise(&self) -> Vec<u8>;
  fn deserialise(buf: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized;
}

pub const TAG_REGISTER : &'static str = "reg";
pub const TAG_GAME_JOIN : &'static str = "gmj";
