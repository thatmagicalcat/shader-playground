use super::*;

use eframe::glow;
use glow::HasContext;

#[rustfmt::skip]
const VERTICES: [f32; 12] = [
     0.5,  0.5, 0.0,
     0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0,
    -0.5, 0.5, 0.0,
];

const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

pub struct OpenGLRenderer {
    gl: std::sync::Arc<glow::Context>,
    program: Program,
    vertex_array: glow::VertexArray,
}

// SAFETY: trust me bro
unsafe impl std::marker::Send for OpenGLRenderer {}

impl OpenGLRenderer {
    pub fn new(
        gl: std::sync::Arc<glow::Context>,
        vert_shader_src: &str,
        frag_shader_src: &str,
    ) -> Self {
        let vertex_array = unsafe { gl.create_vertex_array().unwrap() };

        unsafe {
            gl.bind_vertex_array(Some(vertex_array));

            let [vbo, ebo] =
                std::array::from_fn(|_| gl.create_buffer().expect("Failed to create buffer"));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&VERTICES),
                glow::STATIC_DRAW,
            );

            // position
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                3 * std::mem::size_of::<f32>() as i32,
                0,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&INDICES),
                glow::STATIC_DRAW,
            );

            gl.bind_vertex_array(None);
        }

        Self {
            program: Program::new(&gl, vert_shader_src, frag_shader_src).unwrap(),
            vertex_array,
            gl,
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program.get_program());
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    pub fn recompile_fragment_shader(&mut self, frag_shader_src: &str) {
        self.program
            .recompile_fragment_shader(&self.gl, frag_shader_src);
    }

    pub fn get_error(&self) -> Option<&str> {
        self.program.get_error()
    }

    pub fn paint(&self, rect: eframe::egui::Rect) {
        let gl = &self.gl;
        unsafe {
            gl.viewport(
                rect.min.x as _,
                rect.min.y as _,
                rect.width() as _,
                rect.width() as _,
            );

            gl.use_program(Some(self.program.get_program()));

            let resolution_location =
                gl.get_uniform_location(self.program.get_program(), "u_resolution");

            if let Some(location) = resolution_location {
                gl.uniform_2_f32(Some(&location), rect.width(), rect.height());
            }

            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_elements(glow::TRIANGLES, INDICES.len() as _, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }
}
