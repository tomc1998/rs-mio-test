#[derive(Clone)]
pub struct GlobalState { 
  /// Previous time in ns - unspecified epoch
  pub prev_time: u64, 
  /// Frame delta in ns
  pub delta: u64, 

}

impl GlobalState {
  /// Get the current frame delta in seconds.
  pub fn get_delta_in_s(&self) -> f32 {
    return self.delta as f32 / 1000000000.0f32;
  }
}


