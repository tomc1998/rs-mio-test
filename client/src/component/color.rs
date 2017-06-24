use specs;

/// Color component. R, G, B, A format.
pub struct CompColor(pub [f32; 4]);
impl specs::Component for CompColor {
  type Storage = specs::VecStorage<CompColor>;
}
