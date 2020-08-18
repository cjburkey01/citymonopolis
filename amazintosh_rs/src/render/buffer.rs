use std::fmt::Debug;
use std::marker::PhantomData;

// Handler for creating and managing buffers
pub trait BufferHandler: Clone {
    fn gen_buffer(&mut self) -> u32;

    fn bind_buffer(&mut self, buffer_type: BufferType, handle: u32);

    fn buffer_data<DataType: Sized>(
        &mut self,
        buffer_type: BufferType,
        usage: BufferUsage,
        data: &[DataType],
    );

    fn delete_buffer(&mut self, handle: u32);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BufferType {
    ArrayBuffer,
    ElementArrayBuffer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BufferUsage {
    StaticDraw,
}

pub struct Buffer<BHType: Clone + BufferHandler, DataType: Sized> {
    buffer_handler: BHType,
    buffer_type: BufferType,
    handle: u32,
    _phantom: PhantomData<DataType>,
}

impl<BHType: BufferHandler, DataType: Sized> Buffer<BHType, DataType> {
    pub fn new(buffer_handler: &mut BHType, buffer_type: BufferType) -> Self {
        Self {
            handle: buffer_handler.gen_buffer(),
            buffer_handler: buffer_handler.clone(),
            buffer_type,
            _phantom: PhantomData,
        }
    }

    pub fn bind(&mut self) {
        self.buffer_handler
            .bind_buffer(self.buffer_type, self.handle);
    }

    pub fn buffer_data(&mut self, data: Vec<DataType>, usage: BufferUsage) {
        self.bind();
        self.buffer_handler
            .buffer_data(self.buffer_type, usage, &data)
    }
}

impl<BHType: BufferHandler, DataType: Sized> Drop for Buffer<BHType, DataType> {
    fn drop(&mut self) {
        println!(
            "Dropping buffer {} of type {:?}",
            self.handle, self.buffer_type
        );

        self.buffer_handler.delete_buffer(self.handle);
    }
}
