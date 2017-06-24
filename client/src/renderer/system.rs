use renderer::{Renderer, RendererController};

use specs;
use component::*;

/// The ECS system, which controls the buffering of vertex data into the
/// Renderer via a system of channels.
#[derive(Clone)]
pub struct SysRenderer {
  /// Vertex data sender - used to send to the renderer for rendering from
  /// a different thread.
  r_controller: RendererController,
}

impl SysRenderer {
  /// Create a new renderer system, which will buffer data to the given
  /// renderer when ran.
  pub fn new(r: &Renderer) -> SysRenderer {
    SysRenderer { r_controller: r.get_renderer_controller() }
  }
}

impl<'a, T> specs::System<T> for SysRenderer {
  fn run(&mut self, arg: specs::RunArg, _: T) {
    let (all_col, all_aabb) = arg.fetch(|w|  {
      (w.read::<CompColor>(), w.read::<CompAABB>())
    });

    use specs::Join;
    for (col, aabb) in (&all_col, &all_aabb).join() {
      self.r_controller.rect(&aabb.0, &col.0);
    }
  }
}

