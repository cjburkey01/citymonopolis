use crate::render::types::RGBAColor;
use std::rc::Rc;

/// Contains some generic types that are useful when rendering.
pub mod types;

/// Contains safer implementations to allow OpenGL shader management.
pub mod shader;

/// OpenGL mesh data
pub mod mesh;

/// OpenGL vertex buffers
pub mod vertex;

/// Contains the raw OpenGL calls that this renderer needs to use.
pub mod inner_gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
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
