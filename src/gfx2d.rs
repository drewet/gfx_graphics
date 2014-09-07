
use device;
use gfx;
use gfx::DeviceHelper;
use graphics::{
    BackEnd,
};

use Texture;

static BUFFER_SIZE: uint = 1024;

static VERTEX_SHADER: gfx::ShaderSource = shaders!{
    GLSL_120: b"
#version 120
attribute vec2 pos;
attribute vec3 color;
varying vec4 v_Color;
void main() {
    v_Color = vec4(color, 1.0);
    gl_Position = vec4(pos, 0.0, 1.0);
}
"
    GLSL_150: b"
#version 150 core
in vec2 pos;
in vec3 color;
out vec4 v_Color;
void main() {
    v_Color = vec4(color, 1.0);
    gl_Position = vec4(pos, 0.0, 1.0);
}
"
};

static FRAGMENT_SHADER: gfx::ShaderSource = shaders!{
    GLSL_120: b"
#version 120
varying vec4 v_Color;
void main() {
    gl_FragColor = v_Color;
}
"
    GLSL_150: b"
#version 150 core
in vec4 v_Color;
out vec4 o_Color;
void main() {
    o_Color = v_Color;
}
"
};

static VERTEX_SHADER_UV: gfx::ShaderSource = shaders!{
    GLSL_120: b"
#version 120
attribute vec2 pos;
attribute vec3 color;
attribute vec2 uv;
uniform sampler2D s_texture;
varying vec4 v_Color;
varying vec2 v_UV;
void main() {
    v_UV = uv;
    v_Color = color;
    gl_Position = a_v4Position;
}
"
    GLSL_150: b"
#version 150 core
in vec2 pos;
in vec3 color;
in vec2 uv;
uniform sampler2D s_texture;
out vec4 v_Color;
out vec2 v_UV;
void main() {
    v_UV = uv;
    v_Color = color;
    gl_Position = a_v4Position;
}
"
};

static FRAGMENT_SHADER_UV: gfx::ShaderSource = shaders!{
    GLSL_120: b"
#version 120
uniform sampler2D s_texture;
varying vec2 v_UV;
varying vec4 v_Color;
void main()
{
    gl_FragColor = texture(s_texture, v_UV) * v_Color;
}
"
    GLSL_150: b"
#version 150 core
out vec4 o_Color;
uniform sampler2D s_texture;
in vec2 v_UV;
in vec4 v_Color;
void main()
{
    o_Color = texture(s_texture, v_UV) * v_Color;
}
"
};

#[vertex_format]
struct Vertex {
    pos: [f32, ..2],
    color: [f32, ..4],
}

impl Vertex {
    fn new(pos: [f32, ..2], color: [f32, ..4]) -> Vertex {
        Vertex {
            pos: pos,
            color: color,
        }
    }
}

#[vertex_format]
struct VertexUV {
    pos: [f32, ..2],
    color: [f32, ..4],
    uv: [f32, ..2],
}

impl VertexUV {
    fn new(pos: [f32, ..2], color: [f32, ..4], uv: [f32, ..2]) -> VertexUV {
        VertexUV {
            pos: pos,
            color: color,
            uv: uv,
        }
    }
}

#[shader_param(BatchUV, OwnedBatchUV)]
struct ParamsUV {
    s_texture: gfx::shade::TextureParam,
}

/// The graphics back-end.
pub struct Gfx2d<C: gfx::CommandBuffer> {
    state: gfx::DrawState,
    program: device::Handle<u32,device::shade::ProgramInfo>,
    program_uv: device::Handle<u32,device::shade::ProgramInfo>,
    buffer: gfx::BufferHandle<Vertex>,
    buffer_uv: gfx::BufferHandle<VertexUV>,
    mesh: gfx::Mesh,
    mesh_uv: gfx::Mesh,
}

impl<C: gfx::CommandBuffer> Gfx2d<C> {
    /// Creates a new Gfx2d object.
    pub fn new<D: gfx::Device<C>>(device: &mut D) -> Gfx2d<C> {
        let buffer = device.create_buffer(BUFFER_SIZE, gfx::UsageDynamic);
        let buffer_uv = device.create_buffer(BUFFER_SIZE, gfx::UsageDynamic);
        Gfx2d {
            state: gfx::DrawState::new(),
            program: device.link_program(
                    VERTEX_SHADER.clone(),
                    FRAGMENT_SHADER.clone()
                ).unwrap(),
            program_uv: device.link_program(
                    VERTEX_SHADER_UV.clone(),
                    FRAGMENT_SHADER_UV.clone()
                ).unwrap(),
            buffer: buffer,
            buffer_uv: buffer_uv,
            mesh: gfx::Mesh::from_format(buffer, BUFFER_SIZE as u32),
            mesh_uv: gfx::Mesh::from_format(buffer, BUFFER_SIZE as u32),
        }
    }
}

impl<'a, C: gfx::CommandBuffer> BackEnd<Texture>
for (gfx::Renderer<C>, Gfx2d<C>) {
    fn supports_tri_list_xy_f32_rgba_f32(&self) -> bool { true }

    fn tri_list_xy_f32_rgba_f32(
        &mut self,
        vertices: &[f32],
        colors: &[f32]
    ) {
        let &(ref mut renderer, Gfx2d {
            ref mut buffer,
            ..
        }) = self;
        let mut vertex_data = Vec::new();
        let n = vertices.len() / 2;
        for i in range(0, n) {
            vertex_data.push(
                Vertex::new(
                    [vertices[2 * i], vertices[2 * i + 1]],
                    [
                        colors[4 * i],
                        colors[4 * i + 1],
                        colors[4 * i + 2],
                        colors[4 * i + 3]
                    ]
                )
            );
        }

        /*
        let n = vertex_data.len();
        renderer.update_buffer_vec(*buffer, vertex_data, 0);
        */
    }
}
