pub trait FenceRenderer {
    fn top_right<'a>(&self) -> &'a str;
    fn top_left<'a>(&self) -> &'a str;
    fn bottom_left<'a>(&self) -> &'a str;
    fn bottom_right<'a>(&self) -> &'a str;
    fn vertical_wall<'a>(&self) -> &'a str;
    fn horizontal_wall<'a>(&self) -> &'a str;
}

struct SolidWallFence;
struct FloatingWallFence;
pub struct Fence;

impl Fence {
    pub fn renderer(floating_walls_mode: bool) -> Box<dyn FenceRenderer> {
        if floating_walls_mode { Box::new(FloatingWallFence) } else { Box::new(SolidWallFence) }
    }
}

impl FenceRenderer for SolidWallFence {
    fn top_right<'a>(&self) -> &'a str { "╗" }
    fn top_left<'a>(&self) -> &'a str { "╔" }
    fn bottom_left<'a>(&self) -> &'a str { "╚" }
    fn bottom_right<'a>(&self) -> &'a str { "╝" }
    fn vertical_wall<'a>(&self) -> &'a str { "║" }
    fn horizontal_wall<'a>(&self) -> &'a str { "═" }
}

impl FenceRenderer for FloatingWallFence {
    fn top_right<'a>(&self) -> &'a str { "┐" }
    fn top_left<'a>(&self) -> &'a str { "┌" }
    fn bottom_left<'a>(&self) -> &'a str { "└" }
    fn bottom_right<'a>(&self) -> &'a str { "┘" }
    fn vertical_wall<'a>(&self) -> &'a str { "│" }
    fn horizontal_wall<'a>(&self) -> &'a str { "─" }
}
