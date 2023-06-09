pub trait FenceRenderer {
    fn top_right(&self) -> &str;
    fn top_left(&self) -> &str;
    fn bottom_left(&self) -> &str;
    fn bottom_right(&self) -> &str;
    fn vertical_wall(&self) -> &str;
    fn horizontal_wall(&self) -> &str;
}

struct SolidWallFence;
struct FloatingWallFence;
pub struct Fence;

impl Fence {
    pub fn renderer(floating_walls_mode: bool) -> Box<dyn FenceRenderer> {
        if floating_walls_mode { Box::new(FloatingWallFence {}) } else { Box::new(SolidWallFence {}) }
    }
}

impl FenceRenderer for SolidWallFence {
    fn top_right(&self) -> &str { "╗" }
    fn top_left(&self) -> &str { "╔" }
    fn bottom_left(&self) -> &str { "╚" }
    fn bottom_right(&self) -> &str { "╝" }
    fn vertical_wall(&self) -> &str { "║" }
    fn horizontal_wall(&self) -> &str { "═" }
}

impl FenceRenderer for FloatingWallFence {
    fn top_right(&self) -> &str { "┐" }
    fn top_left(&self) -> &str { "┌" }
    fn bottom_left(&self) -> &str { "└" }
    fn bottom_right(&self) -> &str { "┘" }
    fn vertical_wall(&self) -> &str { "│" }
    fn horizontal_wall(&self) -> &str { "─" }
}
