use glam::Vec3;
use winit::window::Window;

use crate::particle::Scene;

#[derive(Debug, Clone)]
pub struct HudState {
    pub fps: f32,
    pub time_scale: f32,
    pub paused: bool,
    pub fullscreen: bool,
}

impl Default for HudState {
    fn default() -> Self {
        Self {
            fps: 0.0,
            time_scale: 1.0,
            paused: false,
            fullscreen: false,
        }
    }
}

pub fn update_window_title(window: &Window, scene: &Scene, hud: &HudState, gravity: Vec3) {
    let pause = if hud.paused { "  PAUSED" } else { "" };
    window.set_title(&format!(
        "Particles Demo — {} | {} pts | {:>4.0} FPS | g=({:+.1},{:+.1}) | {:.2}x{}{}",
        scene.preset.name(),
        scene.total_particles(),
        hud.fps,
        gravity.x,
        gravity.y,
        hud.time_scale,
        pause,
        if hud.fullscreen { " | FS" } else { "" }
    ));
}
