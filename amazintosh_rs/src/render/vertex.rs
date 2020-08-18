use super::inner_gl::types::GLenum;
use super::GlDataType;
use crate::render::RenderHandler;

#[derive(Debug, Clone)]
pub struct VertexAttribPointer {
    pub index: usize,
    pub size: usize,
    pub data_type: GLenum,
    pub normalized: bool,
    pub offset: usize,
}

impl VertexAttribPointer {
    pub fn new<DataType: GlDataType>(
        index: usize,
        size: usize,
        normalized: bool,
        offset: usize,
    ) -> Self {
        Self {
            index,
            size,
            data_type: DataType::gl_data_type(),
            normalized,
            offset,
        }
    }
}

pub trait Vertex: Sized {
    fn attrib_pointers() -> Vec<VertexAttribPointer>;

    fn render<RHType: RenderHandler>(render_handler: &mut RHType, elements: usize);
}
