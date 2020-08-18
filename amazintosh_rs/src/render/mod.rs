/// Contains some generic types that are useful when rendering.
pub mod types;

/// Contains safer implementations to allow OpenGL shader management.
pub mod shader;

/// OpenGL mesh data
pub mod mesh;

/// OpenGL buffers
pub mod buffer;

/// OpenGL vertex buffers
pub mod vertex;

/// Contains the raw OpenGL calls that this renderer needs to use.
pub mod inner_gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use crate::render::mesh::{MeshHandler, MeshMode};
use crate::render::vertex::{Vertex, VertexAttribPointer};
use buffer::{BufferHandler, BufferType, BufferUsage};
use inner_gl::types::{GLenum, GLsizei, GLuint, GLvoid};
use std::rc::Rc;
use types::RGBAColor;

pub trait RenderHandler: BufferHandler + MeshHandler {}

pub trait GlType {
    fn gl_type(&self) -> GLenum;
}

impl GlType for BufferType {
    fn gl_type(&self) -> GLenum {
        match *self {
            Self::ArrayBuffer => inner_gl::ARRAY_BUFFER,
            Self::ElementArrayBuffer => inner_gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

impl GlType for BufferUsage {
    fn gl_type(&self) -> GLenum {
        match *self {
            Self::StaticDraw => inner_gl::STATIC_DRAW,
        }
    }
}

impl GlType for MeshMode {
    fn gl_type(&self) -> GLenum {
        match *self {
            Self::Triangles => inner_gl::TRIANGLES,
        }
    }
}

pub trait GlDataType {
    fn gl_data_type() -> GLenum;
}

impl GlDataType for f32 {
    fn gl_data_type() -> GLenum {
        inner_gl::FLOAT
    }
}

impl GlDataType for u16 {
    fn gl_data_type() -> GLenum {
        inner_gl::UNSIGNED_SHORT
    }
}

/// Acts as a safer wrapper around the OpenGL api.
#[derive(Clone)]
pub struct Gl(pub Rc<inner_gl::Gl>);

impl Gl {
    /// Creates a new OpenGL wrapper from a closure that returns a pointer to a
    /// function based on its name.
    pub fn new<F: FnMut(&'static str) -> *const std::os::raw::c_void>(f: F) -> Self {
        Self(Rc::new(inner_gl::Gl::load_with(f)))
    }

    /// Sets the color used to clear the screen.
    pub fn set_clear_color<Color: Into<RGBAColor>>(&mut self, color: Color) {
        let color: [f32; 4] = color.into().into();
        unsafe {
            self.0.ClearColor(color[0], color[1], color[2], color[3]);
        }
    }

    /// Clears the color buffer and/or the depth buffer.
    pub fn clear(&mut self, color: bool, depth: bool) {
        // Nothing needs to be updated, just skip this call.
        if !color && !depth {
            return;
        }

        // Determine all of the buffers to clear
        let mut clear_bits = 0u32;
        if color {
            clear_bits |= inner_gl::COLOR_BUFFER_BIT;
        }
        if depth {
            clear_bits |= inner_gl::DEPTH_BUFFER_BIT;
        }

        unsafe {
            self.0.Clear(clear_bits);
        }
    }
}

impl BufferHandler for Gl {
    fn gen_buffer(&mut self) -> u32 {
        let mut handle = 0;
        unsafe {
            self.0.GenBuffers(1, &mut handle);
        }
        handle
    }

    fn bind_buffer(&mut self, buffer_type: BufferType, handle: u32) {
        unsafe {
            self.0.BindBuffer(buffer_type.gl_type(), handle);
        }
    }

    fn buffer_data<DataType: Sized>(
        &mut self,
        buffer_type: BufferType,
        usage: BufferUsage,
        data: &[DataType],
    ) {
        // Get the buffer size in bytes
        let buffer_size = (data.len() * std::mem::size_of::<DataType>()) as isize;

        unsafe {
            // Invalidate the buffer
            self.0.BufferData(
                buffer_type.gl_type(),
                buffer_size,
                std::ptr::null(),
                usage.gl_type(),
            );

            // Buffer the new data
            self.0.BufferData(
                buffer_type.gl_type(),
                buffer_size,
                data.as_ptr() as *const GLvoid,
                usage.gl_type(),
            );
        }
    }

    fn delete_buffer(&mut self, handle: u32) {
        unsafe {
            self.0.DeleteBuffers(1, &handle);
        }
    }
}

impl MeshHandler for Gl {
    fn gen_vertex_array(&mut self) -> u32 {
        let mut vao: GLuint = 0;
        unsafe {
            self.0.GenVertexArrays(1, &mut vao);
        }
        vao
    }

    fn delete_vertex_array(&mut self, handle: u32) {
        unsafe {
            self.0.DeleteVertexArrays(1, &handle);
        }
    }

    fn vertex_attrib_pointer<VertexType: Vertex>(&mut self, pointer: &VertexAttribPointer) {
        unsafe {
            self.0.VertexAttribPointer(
                pointer.index as GLuint,
                pointer.size as GLsizei,
                pointer.data_type,
                if pointer.normalized {
                    inner_gl::TRUE
                } else {
                    inner_gl::FALSE
                },
                std::mem::size_of::<VertexType>() as GLsizei,
                pointer.offset as *const GLvoid,
            );
        }
    }

    fn enable_attrib_array(&mut self, index: usize) {
        unsafe {
            self.0.EnableVertexAttribArray(index as GLuint);
        }
    }

    fn disable_attrib_array(&mut self, index: usize) {
        unsafe {
            self.0.DisableVertexAttribArray(index as GLuint);
        }
    }

    fn bind_vertex_array(&mut self, handle: u32) {
        unsafe {
            self.0.BindVertexArray(handle);
        }
    }

    fn draw_elements<IndexType: GlDataType>(&mut self, mode: MeshMode, indices: usize) {
        unsafe {
            self.0.DrawElements(
                mode.gl_type(),
                indices as GLsizei,
                IndexType::gl_data_type(),
                std::ptr::null(),
            );
        }
    }
}

impl RenderHandler for Gl {}
