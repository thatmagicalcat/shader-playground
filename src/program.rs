#![allow(unused)]

use std::sync::Arc;

use eframe::glow;
use glow::HasContext;

pub struct Program {
    program: glow::WebProgramKey,

    vert_shader: glow::WebShaderKey,
    frag_shader: glow::WebShaderKey,

    error: Option<String>,
}

impl Program {
    pub fn new(
        gl: &glow::Context,
        vert_shader_src: &str,
        frag_shader_src: &str,
    ) -> std::io::Result<Self> {
        let vert_shader = compile_shader(gl, glow::VERTEX_SHADER, vert_shader_src)
            .expect("Failed to compile vertex shader");
        let frag_shader = compile_shader(gl, glow::FRAGMENT_SHADER, frag_shader_src)
            .expect("Failed to compile fragment shader");

        let program = unsafe { gl.create_program().unwrap() };

        unsafe {
            gl.attach_shader(program, vert_shader);
            gl.attach_shader(program, frag_shader);
            gl.link_program(program);
            gl.validate_program(program);
        };

        Ok(Self {
            program,
            vert_shader,
            frag_shader,
            error: None
        })
    }

    pub fn get_error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn recompile_fragment_shader(&mut self, gl: &glow::Context, frag_shader_src: &str) {
        unsafe {
            let frag_shader = compile_shader(gl, glow::FRAGMENT_SHADER, frag_shader_src);

            if let Err(e) = frag_shader {
                self.error = Some(e);
                return;
            }

            let frag_shader = frag_shader.unwrap();
            _ = self.error.take();

            gl.detach_shader(self.program, self.frag_shader);
            gl.delete_shader(self.frag_shader);

            gl.attach_shader(self.program, frag_shader);
            gl.link_program(self.program);
            gl.validate_program(self.program);

            self.frag_shader = frag_shader;
        }
    }

    pub fn get_program(&self) -> glow::WebProgramKey {
        self.program
    }
}

fn compile_shader(gl: &glow::Context, ty: u32, source: &str) -> Result<glow::WebShaderKey, String> {
    let shader = unsafe { gl.create_shader(ty).unwrap() };

    unsafe {
        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            let mut result: i32 = 0;
            let info = gl.get_shader_info_log(shader);
            gl.delete_shader(shader);

            return Err(info);
        }
    }

    Ok(shader)
}
