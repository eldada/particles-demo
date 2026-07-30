mod overlay;
mod pipeline;
mod texture;

pub use overlay::HudOverlay;
pub use pipeline::ParticlePipeline;

use std::sync::Arc;

use glam::{Mat4, Vec3};
use winit::window::Window;

use crate::particle::Scene;

pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub pipeline: ParticlePipeline,
    pub hud: HudOverlay,
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            target: Vec3::new(0.0, 0.0, -2.0),
            distance: 2.8,
            yaw: 0.0,
            pitch: 0.15,
        }
    }
}

impl Camera {
    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.yaw += dx * 0.005;
        self.pitch = (self.pitch + dy * 0.005).clamp(-1.2, 1.2);
    }

    pub fn zoom(&mut self, scroll: f32) {
        self.distance = (self.distance * (1.0 - scroll * 0.08)).clamp(0.8, 12.0);
    }

    pub fn eye(&self) -> Vec3 {
        let cp = self.pitch.cos();
        let offset = Vec3::new(
            self.yaw.sin() * cp,
            self.pitch.sin(),
            self.yaw.cos() * cp,
        ) * self.distance;
        self.target + offset
    }

    pub fn view_proj(&self, aspect: f32) -> Mat4 {
        let view = glam::camera::rh::view::look_at_mat4(self.eye(), self.target, Vec3::Y);
        let proj = glam::camera::rh::proj::directx::perspective(
            45f32.to_radians(),
            aspect.max(0.1),
            0.1,
            100.0,
        );
        proj * view
    }

    pub fn screen_to_world_on_target_plane(
        &self,
        sx: f32,
        sy: f32,
        width: f32,
        height: f32,
    ) -> Vec3 {
        let aspect = (width / height.max(1.0)).max(0.1);
        let view_proj = self.view_proj(aspect);
        let inv = view_proj.inverse();

        let ndc_x = (sx / width) * 2.0 - 1.0;
        let ndc_y = 1.0 - (sy / height) * 2.0;

        let near = inv * glam::Vec4::new(ndc_x, ndc_y, 0.0, 1.0);
        let far = inv * glam::Vec4::new(ndc_x, ndc_y, 1.0, 1.0);
        let near = near.truncate() / near.w;
        let far = far.truncate() / far.w;
        let dir = (far - near).normalize_or_zero();

        let plane_z = self.target.z;
        if dir.z.abs() < 1e-5 {
            return Vec3::new(self.target.x, self.target.y, plane_z);
        }
        let t = (plane_z - near.z) / dir.z;
        near + dir * t
    }
}

impl Renderer {
    pub async fn new(
        window: Arc<Window>,
        vsync: bool,
        star_bytes: &[u8],
        hud_bytes: &[u8],
    ) -> Self {
        let size = window.inner_size();
        let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
        desc.backends = wgpu::Backends::PRIMARY;
        let instance = wgpu::Instance::new(desc);

        let surface = instance
            .create_surface(window.clone())
            .expect("create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: false,
            })
            .await
            .expect("no suitable GPU adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .expect("request device");

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let present_mode = if vsync {
            if caps.present_modes.contains(&wgpu::PresentMode::Fifo) {
                wgpu::PresentMode::Fifo
            } else {
                caps.present_modes[0]
            }
        } else if caps.present_modes.contains(&wgpu::PresentMode::Mailbox) {
            wgpu::PresentMode::Mailbox
        } else if caps.present_modes.contains(&wgpu::PresentMode::Immediate) {
            wgpu::PresentMode::Immediate
        } else {
            caps.present_modes[0]
        };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let pipeline = ParticlePipeline::new(&device, &queue, format, star_bytes);
        let hud = HudOverlay::new(&device, &queue, format, hud_bytes, (380.0, 530.0));
        hud.update_layout(&queue, config.width, config.height, window.scale_factor() as f32);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            hud,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, scale: f32) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        self.hud
            .update_layout(&self.queue, self.config.width, self.config.height, scale);
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) -> RenderResult {
        let surface_tex = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return RenderResult::Skip;
            }
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                return RenderResult::NeedsResize;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                eprintln!("surface validation error");
                return RenderResult::Fatal;
            }
        };

        let view = surface_tex
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let aspect = self.config.width as f32 / self.config.height.max(1) as f32;
        let view_proj = camera.view_proj(aspect);
        self.pipeline
            .upload_scene(&self.device, &self.queue, scene, view_proj);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame-encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            self.pipeline.draw(&mut pass);
            self.hud.draw(&mut pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(surface_tex);
        RenderResult::Ok
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderResult {
    Ok,
    Skip,
    NeedsResize,
    Fatal,
}
