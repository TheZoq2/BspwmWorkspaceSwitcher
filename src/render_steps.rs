use glium::texture::texture2d::Texture2d;
use glium::backend::Facade;
use glium::{Program, VertexBuffer};
use glium::framebuffer::SimpleFrameBuffer;
use glium;
use glium::Surface;

use std::collections::{HashSet};

use glium_types::Vertex;

use rendering::RenderTargets;


pub const DEFAULT_FRAGMENT_SHADER: &'static str = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D diffuse_texture;
        uniform sampler2D emissive_texture;
        void main() {
            vec4 emissive_color = vec4(0., 0., 0., 0.);

            float blur_amount = 3;
            for(float x = -blur_amount; x <= blur_amount; x++)
            {
                for(float y = -blur_amount; y <= blur_amount; y++)
                {
                    vec2 coords = v_tex_coords + vec2(x, y) * 0.001;
                    emissive_color += texture(emissive_texture, coords);
                }
            }
            emissive_color = emissive_color / (blur_amount * blur_amount * 2 * 2);

            //emissive_color = texture(emissive_texture, v_tex_coords);

            vec4 diffuse_color = texture(diffuse_texture, v_tex_coords);
            //vec4 diffuse_color = vec4(0., 0., 0., 0.);
            //emissive_color = vec4(0., 0., 0., 0.);
            color = diffuse_color  + emissive_color;
        }
    "#;


#[derive(Clone, Eq, PartialEq, Hash)]
pub enum RenderSteps
{
    Diffuse,
    Emissive,
}

//TODO: Make a trait for this
impl RenderSteps
{
    pub fn get_hash_set() -> HashSet<RenderSteps>
    {
        let mut set = HashSet::new();

        set.insert(RenderSteps::Diffuse);
        set.insert(RenderSteps::Emissive);

        set
    }
}

pub struct RenderParameters
{
    diffuse_texture: Texture2d,
    emissive_texture: Texture2d,
    ambient: f32,
}

impl RenderParameters
{
    pub fn new(facade: &Facade, resolution: (u32, u32)) -> RenderParameters
    {
        RenderParameters {
            diffuse_texture: Texture2d::empty(facade, resolution.0, resolution.1)
                .unwrap(),
            emissive_texture: Texture2d::empty(facade, resolution.0, resolution.1)
                .unwrap(),
            ambient: 0.
        }
    }
}

impl RenderTargets<RenderSteps> for RenderParameters
{
    fn get_render_target(&self, target: &RenderSteps) -> SimpleFrameBuffer
    {
        match *target
        {
            RenderSteps::Diffuse => self.diffuse_texture.as_surface(),
            RenderSteps::Emissive => self.emissive_texture.as_surface()
        }
    }
}


pub fn default_render_function(
            target: &mut glium::Frame,
            uniforms: &RenderParameters,
            vertex_buffer: &VertexBuffer<Vertex>,
            shader: &Program
        )
{
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let uniform_object = uniform!{
        diffuse_texture: &uniforms.diffuse_texture,
        emissive_texture: &uniforms.emissive_texture,
        ambient: uniforms.ambient
    };


    target.draw(vertex_buffer, &indices, shader, &uniform_object,
                &Default::default()).unwrap();
}
