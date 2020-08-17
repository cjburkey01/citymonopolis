use super::inner_gl;
use super::inner_gl::types::{GLenum, GLint, GLuint, GLvoid};
use super::Gl;
use glm::Vec3;
use std::convert::TryInto;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BufferType {
    ArrayBuffer,
    ElementArrayBuffer,
}

impl BufferType {
    fn gl_type(self) -> GLenum {
        match self {
            Self::ArrayBuffer => inner_gl::ARRAY_BUFFER,
            Self::ElementArrayBuffer => inner_gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BufferUsage {
    StaticDraw,
}

impl BufferUsage {
    fn gl_usage(self) -> GLenum {
        match self {
            Self::StaticDraw => inner_gl::STATIC_DRAW,
        }
    }
}

pub struct Buffer<DataType: Sized> {
    gl: Gl,
    buffer_type: BufferType,
    handle: GLuint,
    _phantom: PhantomData<DataType>,
}

impl<DataType: Sized> Buffer<DataType> {
    pub fn new(gl: &mut Gl, buffer_type: BufferType) -> Result<Self, ()> {
        Ok(Self {
            handle: {
                let mut handle: GLuint = 0;
                unsafe {
                    gl.0.GenBuffers(1, &mut handle);
                }
                Ok(handle)
            }?,
            gl: gl.clone(),
            buffer_type,
            _phantom: PhantomData,
        })
    }

    pub fn bind(&mut self) {
        unsafe {
            self.gl
                .0
                .BindBuffer(self.buffer_type.gl_type(), self.handle);
        }
    }

    pub fn buffer_data(&mut self, data: Vec<DataType>, usage: BufferUsage) -> Result<(), ()> {
        self.bind();

        unsafe {
            self.gl.0.BufferData(
                self.buffer_type.gl_type(),
                (data.len() * std::mem::size_of::<DataType>())
                    .try_into()
                    .map_err(|_| ())?,
                data.as_ptr() as *const GLvoid,
                usage.gl_usage(),
            );
        }

        Ok(())
    }
}

impl<DataType: Sized> Drop for Buffer<DataType> {
    fn drop(&mut self) {
        println!(
            "Dropping buffer {} of type {:?}",
            self.handle, self.buffer_type
        );

        unsafe {
            self.gl.0.DeleteBuffers(1, &self.handle);
        }
    }
}

pub trait GlType {
    fn gl_type() -> GLenum;
}

impl GlType for f32 {
    fn gl_type() -> GLenum {
        inner_gl::FLOAT
    }
}

pub struct VertexAttribPointer {
    pub size: usize,
    pub data_type: GLenum,
    pub normalized: bool,
    pub offset: usize,
}

impl VertexAttribPointer {
    pub fn new<DataType: GlType>(size: usize, normalized: bool, offset: usize) -> Self {
        Self {
            size,
            data_type: DataType::gl_type(),
            normalized,
            offset,
        }
    }
}

pub trait Vertex: Sized {
    fn setup_attrib_pointers() -> Vec<VertexAttribPointer>;
}
