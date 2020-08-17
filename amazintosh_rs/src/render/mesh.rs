use super::inner_gl;
use super::inner_gl::types::{GLint, GLuint, GLushort, GLvoid};
use super::Gl;
use crate::render::vertex::{Buffer, BufferType, BufferUsage, Vertex};
use std::convert::TryInto;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MeshError {
    FailedToUpdateData,
    FailedToBufferData,
    FailedToSetupAttribs,
}

impl Display for MeshError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for MeshError {}

pub struct Mesh<VertexType: Vertex> {
    gl: Gl,
    vao: GLuint,
    vbo: Option<Buffer<VertexType>>,
    ebo: Option<Buffer<GLushort>>,
    indices: GLint,
    attrib_indices: Vec<GLuint>,
}

impl<VertexType: Vertex> Mesh<VertexType> {
    pub fn new(gl: &mut Gl) -> Result<Self, MeshError> {
        Ok(Self {
            vao: {
                let mut vao: GLuint = 0;
                unsafe {
                    gl.0.GenVertexArrays(1, &mut vao);
                }
                Ok(vao)
            }?,
            gl: gl.clone(),
            vbo: None,
            ebo: None,
            indices: 0,
            attrib_indices: Vec::with_capacity(1),
        })
    }

    pub fn update_data(
        &mut self,
        vertices: Vec<VertexType>,
        indices: Vec<u16>,
        usage: BufferUsage,
    ) -> Result<(), MeshError> {
        self.bind();

        let indices_count = indices
            .len()
            .try_into()
            .map_err(|_| MeshError::FailedToUpdateData)?;

        let mut vbo = Buffer::new(&mut self.gl, BufferType::ArrayBuffer)
            .expect("failed to create vertex buffer");
        vbo.buffer_data(vertices, usage)
            .map_err(|_| MeshError::FailedToBufferData)?;

        let mut ebo = Buffer::new(&mut self.gl, BufferType::ElementArrayBuffer)
            .expect("failed to create element array buffer");
        ebo.buffer_data(indices, usage)
            .map_err(|_| MeshError::FailedToBufferData)?;

        for (index, attrib_pointer) in VertexType::setup_attrib_pointers().iter().enumerate() {
            self.attrib_indices.push(index as GLuint);

            unsafe {
                self.gl.0.VertexAttribPointer(
                    index
                        .try_into()
                        .map_err(|_| MeshError::FailedToSetupAttribs)?,
                    attrib_pointer
                        .size
                        .try_into()
                        .map_err(|_| MeshError::FailedToSetupAttribs)?,
                    attrib_pointer.data_type,
                    if attrib_pointer.normalized {
                        inner_gl::TRUE
                    } else {
                        inner_gl::FALSE
                    },
                    std::mem::size_of::<VertexType>()
                        .try_into()
                        .map_err(|_| MeshError::FailedToSetupAttribs)?,
                    attrib_pointer.offset as *const GLvoid,
                );
            }
        }

        self.vbo = Some(vbo);
        self.ebo = Some(ebo);
        self.indices = indices_count;

        Ok(())
    }

    pub fn render(&mut self) {
        self.bind();
        if let Some(ebo) = &mut self.ebo {
            ebo.bind();

            for attrib_pointers in self.attrib_indices.iter() {
                unsafe {
                    self.gl.0.EnableVertexAttribArray(*attrib_pointers);
                }
            }

            unsafe {
                self.gl.0.DrawElements(
                    inner_gl::TRIANGLES,
                    self.indices,
                    inner_gl::UNSIGNED_SHORT,
                    std::ptr::null(),
                );
            }

            for attrib_pointers in self.attrib_indices.iter() {
                unsafe {
                    self.gl.0.DisableVertexAttribArray(*attrib_pointers);
                }
            }
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            self.gl.0.BindVertexArray(self.vao);
        }
    }
}

impl<VertexType: Vertex> Drop for Mesh<VertexType> {
    fn drop(&mut self) {
        unsafe {
            self.gl.0.DeleteVertexArrays(1, &self.vao);
        }
    }
}
