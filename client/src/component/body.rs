use specs;

pub const BODY_GRAVITY : u32 = 1;

/// A component representing a physical body in the world. Should be coupled
/// with an AABB component.
pub struct CompBody {
  /// Acceleration vector
  pub acc: [f32; 2],
  /// Velocity vector
  pub vel: [f32; 2],
  /// Mass in KG
  pub mass: f32,
  /// Bitflags indicating properties of this body
  /// * BIT 0 - Gravity. 1 for this body to be affected by gravity, 0 for not.
  pub flags: u32,
}

impl specs::Component for CompBody {
  type Storage = specs::VecStorage<CompBody>;
}

/// AABB component - X, Y, W, H format.
pub struct CompAABB(pub [f32; 4]);
impl specs::Component for CompAABB {
  type Storage = specs::VecStorage<CompAABB>;
}

pub struct CompVel(pub [f32; 2]);
impl specs::Component for CompVel {
  type Storage = specs::VecStorage<CompVel>;
}



