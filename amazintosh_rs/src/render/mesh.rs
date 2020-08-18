use super::buffer::{Buffer, BufferType, BufferUsage};
use super::vertex::Vertex;
use crate::render::vertex::VertexAttribPointer;
use crate::render::{GlDataType, RenderHandler};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MeshMode {
    Triangles,
}

pub trait MeshHandler: Clone {
    fn gen_vertex_array(&mut self) -> u32;

    fn delete_vertex_array(&mut self, handle: u32);

    fn vertex_attrib_pointer<VertexType: Vertex>(&mut self, pointer: &VertexAttribPointer);

    fn enable_attrib_array(&mut self, index: usize);

    fn disable_attrib_array(&mut self, index: usize);

    fn bind_vertex_array(&mut self, handle: u32);

    fn draw_elements<IndexType: GlDataType>(&mut self, mode: MeshMode, indices: usize);
}

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

pub struct Mesh<RHType: RenderHandler, VertexType: Vertex, IndexType: GlDataType> {
    render_handler: RHType,
    vao: u32,
    vbo: Buffer<RHType, VertexType>,
    ebo: Buffer<RHType, IndexType>,
    elements: usize,
}

impl<RHType: RenderHandler, VertexType: Vertex, IndexType: GlDataType>
    Mesh<RHType, VertexType, IndexType>
{
    pub fn new(render_handler: &mut RHType) -> Self {
        let mut mesh = Self {
            vao: render_handler.gen_vertex_array(),
            vbo: Buffer::new(render_handler, BufferType::ArrayBuffer),
            ebo: Buffer::new(render_handler, BufferType::ElementArrayBuffer),
            render_handler: render_handler.clone(),
            elements: 0,
        };

        mesh.bind();
        mesh.vbo.bind();

        VertexType::attrib_pointers()
            .iter()
            .for_each(|ptr| mesh.render_handler.vertex_attrib_pointer::<VertexType>(ptr));

        mesh
    }

    pub fn set_vertices(&mut self, vertices: Vec<VertexType>, usage: BufferUsage) {
        self.bind();

        self.vbo.buffer_data(vertices, usage);
    }

    pub fn set_indices(&mut self, indices: Vec<IndexType>, usage: BufferUsage) {
        self.bind();

        let element_count = indices.len();

        self.ebo.buffer_data(indices, usage);

        self.elements = element_count;
    }

    pub fn render(&mut self) {
        // No need to try to render
        if self.elements < 1 {
            return;
        }

        self.bind();
        self.ebo.bind();

        VertexType::render(&mut self.render_handler, self.elements);
    }

    pub fn bind(&mut self) {
        self.render_handler.bind_vertex_array(self.vao);
    }
}

impl<RHType: RenderHandler, VertexType: Vertex, IndexType: GlDataType> Drop
    for Mesh<RHType, VertexType, IndexType>
{
    fn drop(&mut self) {
        println!("Dropped VAO: {}", self.vao);

        self.render_handler.delete_vertex_array(self.vao)
    }
}
