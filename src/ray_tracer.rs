use scene::Scene;
use color::Color;

pub struct RayTracer<'a> {
    scene: &'a Scene,
}

impl<'a> RayTracer<'a> {
    pub fn new(scene: &Scene) -> RayTracer {
        RayTracer { scene: scene }
    }

    pub fn trace_pixel(&self, x: u32, y: u32) -> Color {
        self.scene.background
    }
}
