mod config;
mod input;
mod particle;
mod render;
mod ui;

use std::sync::Arc;
use std::time::Instant;

use clap::Parser;
use glam::Vec3;
use winit::application::ApplicationHandler;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowId};

use crate::config::{AppConfig, Cli, Preset};
use crate::input::InputState;
use crate::particle::{Scene, gravity_from_arrows};
use crate::render::{Camera, RenderResult, Renderer};
use crate::ui::{HudState, update_window_title};

const STAR_PNG: &[u8] = include_bytes!("../assets/star.png");
const HUD_PNG: &[u8] = include_bytes!("../assets/hud.png");

struct App {
    config: AppConfig,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    scene: Scene,
    camera: Camera,
    input: InputState,
    hud: HudState,
    last_frame: Instant,
    fps_accum: f32,
    fps_frames: u32,
    title_timer: f32,
}

impl App {
    fn new(config: AppConfig) -> Self {
        let scene = Scene::new(config.preset, config.particles_per_system, config.seed);
        Self {
            config,
            window: None,
            renderer: None,
            scene,
            camera: Camera::default(),
            input: InputState::default(),
            hud: HudState::default(),
            last_frame: Instant::now(),
            fps_accum: 0.0,
            fps_frames: 0,
            title_timer: 0.0,
        }
    }

    fn ensure_gpu(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attrs = Window::default_attributes()
            .with_title("Particles Demo — arrows gravity, Space reset, H HUD")
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.config.width,
                self.config.height,
            ));

        let window = Arc::new(
            event_loop
                .create_window(attrs)
                .expect("failed to create window"),
        );

        let renderer = pollster::block_on(Renderer::new(
            window.clone(),
            self.config.vsync,
            STAR_PNG,
            HUD_PNG,
        ));

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.last_frame = Instant::now();
    }

    fn handle_frame(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        let now = Instant::now();
        let raw_dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        let dt = raw_dt.clamp(0.0, 0.05);

        self.fps_accum += raw_dt;
        self.fps_frames += 1;
        if self.fps_accum >= 0.5 {
            self.hud.fps = self.fps_frames as f32 / self.fps_accum;
            self.fps_accum = 0.0;
            self.fps_frames = 0;
        }

        if self.input.reset_particles {
            self.scene.reinit_all();
        }
        if self.input.clear_gravity {
            self.scene.clear_gravity();
        }
        if self.input.toggle_pause {
            self.hud.paused = !self.hud.paused;
        }
        if self.input.toggle_hud {
            renderer.hud.set_visible(!renderer.hud.visible());
        }
        if self.input.toggle_fullscreen {
            self.hud.fullscreen = !self.hud.fullscreen;
            window.set_fullscreen(if self.hud.fullscreen {
                Some(Fullscreen::Borderless(None))
            } else {
                None
            });
        }
        if self.input.slower {
            self.hud.time_scale = (self.hud.time_scale * 0.5).max(0.0625);
        }
        if self.input.faster {
            self.hud.time_scale = (self.hud.time_scale * 2.0).min(8.0);
        }
        if self.input.count_down {
            self.scene.adjust_particle_count(0.5);
        }
        if self.input.count_up {
            self.scene.adjust_particle_count(2.0);
        }
        if let Some(d) = self.input.preset_digit {
            if let Some(preset) = Preset::from_digit(d) {
                self.scene.apply_preset(preset);
            }
        }

        let g = gravity_from_arrows(
            self.input.up,
            self.input.down,
            self.input.left,
            self.input.right,
        );
        if g != Vec3::ZERO {
            self.scene.add_gravity(g * (dt * 60.0));
        }

        if self.input.orbit_dragging {
            self.camera.orbit(
                self.input.mouse_delta_x as f32,
                self.input.mouse_delta_y as f32,
            );
        }
        if self.input.scroll_delta != 0.0 {
            self.camera.zoom(self.input.scroll_delta);
        }

        if self.input.lmb {
            let world = self.camera.screen_to_world_on_target_plane(
                self.input.mouse_x as f32,
                self.input.mouse_y as f32,
                renderer.size.width as f32,
                renderer.size.height as f32,
            );
            let strength = if self.input.shift { -0.08 } else { 0.08 };
            self.scene.apply_attractor(world, strength, dt);
        }

        if !self.hud.paused {
            self.scene.update(dt * self.hud.time_scale);
        }

        self.title_timer += raw_dt;
        if self.title_timer >= 0.25 {
            self.title_timer = 0.0;
            update_window_title(
                window.as_ref(),
                &self.scene,
                &self.hud,
                self.scene.shared_gravity(),
            );
        }

        match renderer.render(&self.scene, &self.camera) {
            RenderResult::Ok | RenderResult::Skip => {}
            RenderResult::NeedsResize => {
                renderer.resize(window.inner_size(), window.scale_factor() as f32);
            }
            RenderResult::Fatal => {
                self.input.want_quit = true;
            }
        }

        self.input.begin_frame();
        window.request_redraw();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.ensure_gpu(event_loop);
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let (Some(window), Some(renderer)) = (&self.window, self.renderer.as_mut()) {
                    renderer.resize(size, window.scale_factor() as f32);
                }
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                if let (Some(window), Some(renderer)) = (&self.window, self.renderer.as_mut()) {
                    renderer.resize(window.inner_size(), window.scale_factor() as f32);
                }
            }
            WindowEvent::RedrawRequested => self.handle_frame(),
            WindowEvent::KeyboardInput { event, .. } => {
                if !event.repeat {
                    self.input.on_key(event.physical_key, event.state);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.input.on_mouse_button(button, state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input.on_cursor_moved(position.x, position.y);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => (p.y as f32) * 0.02,
                };
                self.input.on_scroll(y);
            }
            _ => {}
        }

        if self.input.want_quit {
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let config = AppConfig::from(cli);

    let event_loop = EventLoop::new().expect("event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(config);
    event_loop.run_app(&mut app).expect("run app");
}
