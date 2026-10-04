use bytemuck::{Pod, Zeroable};

use super::texture::Texture;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct OverlayVertex {
    position: [f32; 2],
    uv: [f32; 2],
}

pub struct HudOverlay {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    texture: Texture,
    visible: bool,
    panel_px: (f32, f32),
}

impl HudOverlay {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        hud_bytes: &[u8],
        panel_px: (f32, f32),
    ) -> Self {
        let texture = Texture::from_bytes(device, queue, hud_bytes, "hud");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("hud-shader"),
            source: wgpu::ShaderSource::Wgsl(HUD_WGSL.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("hud-pipeline-layout"),
            bind_group_layouts: &[Some(&texture.bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("hud-pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<OverlayVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("hud-vertices"),
            size: (4 * std::mem::size_of::<OverlayVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            texture,
            visible: true,
            panel_px,
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn update_layout(&self, queue: &wgpu::Queue, width: u32, height: u32, scale: f32) {
        let w = width.max(1) as f32;
        let h = height.max(1) as f32;
        let pw = (self.panel_px.0 * scale).min(w - 24.0);
        let ph = (self.panel_px.1 * scale).min(h - 24.0);
        let margin = 12.0 * scale;

        // Clip space: x/y in [-1, 1], y up
        let x0 = (margin / w) * 2.0 - 1.0;
        let x1 = ((margin + pw) / w) * 2.0 - 1.0;
        let y1 = 1.0 - (margin / h) * 2.0;
        let y0 = 1.0 - ((margin + ph) / h) * 2.0;

        let verts = [
            OverlayVertex {
                position: [x0, y0],
                uv: [0.0, 1.0],
            },
            OverlayVertex {
                position: [x1, y0],
                uv: [1.0, 1.0],
            },
            OverlayVertex {
                position: [x0, y1],
                uv: [0.0, 0.0],
            },
            OverlayVertex {
                position: [x1, y1],
                uv: [1.0, 0.0],
            },
        ];
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&verts));
    }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if !self.visible {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.texture.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..4, 0..1);
    }
}

const HUD_WGSL: &str = r#"
struct VsIn {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

struct VsOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0) @binding(0) var hud_tex: texture_2d<f32>;
@group(0) @binding(1) var hud_sampler: sampler;

@vertex
fn vs_main(in: VsIn) -> VsOut {
    var out: VsOut;
    out.clip_position = vec4<f32>(in.position, 0.0, 1.0);
    out.uv = in.uv;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return textureSample(hud_tex, hud_sampler, in.uv);
}
"#;
