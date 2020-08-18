use super::inner_gl;
use super::inner_gl::types::{GLchar, GLint, GLuint};
use crate::render::Gl;
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::{Debug, Display, Formatter};

macro_rules! gl_error_check {
    ($gl:expr, $handle:expr, $get_shader_iv:ident, $get_shader_info_log:ident, $error:ident) => {{
        // Get the number of characters in the shader's info log to check if
        // there is an error and prevent unnecessary allocation of a bigger buffer.
        let info_log_length: GLint = {
            let mut v: GLint = 0;
            unsafe {
                $gl.0
                    .$get_shader_iv($handle, inner_gl::INFO_LOG_LENGTH, &mut v);
            }
            v
        };

        // Check if the shader failed to compile
        if info_log_length > 0 {
            let error = unsafe {
                // Create a vector with the required length (including the ending null byte)
                let mut buffer: Vec<u8> = Vec::with_capacity(info_log_length as usize + 1);
                // Fill it with spaces
                buffer.extend([b' '].iter().cycle().take(info_log_length as usize));

                // Get the error from OpenGL into the CString
                $gl.0.$get_shader_info_log(
                    $handle,
                    info_log_length,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );

                // Convert the pointer back into a CString and then return a
                // compiler error with an owned string
                CString::from_vec_with_nul(buffer)
                    .map_err(|_| ShaderError::Unknown)?
                    .to_str()
                    .map_err(|_| ShaderError::Unknown)?
                    .to_owned()
            };

            // Return the error
            Err(ShaderError::$error(error.trim().to_owned()))
        } else {
            Ok(())
        }
    }};
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShaderType {
    Vertex,
    Geometry,
    Fragment,
}

impl ShaderType {
    fn gl_enum(self) -> inner_gl::types::GLenum {
        match self {
            Self::Vertex => inner_gl::VERTEX_SHADER,
            Self::Geometry => inner_gl::GEOMETRY_SHADER,
            Self::Fragment => inner_gl::FRAGMENT_SHADER,
        }
    }
}

#[derive(Debug)]
pub enum ShaderError {
    CreateShaderFailed,
    InvalidSourceString,
    Unknown,
    CompileError(String),
    CreateShaderProgramFailed,
    LinkError(String),
    ValidateError(String),
}

impl Display for ShaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Shader {
    gl: Gl,
    handle: GLuint,
}

impl Shader {
    pub fn create_shader(
        gl: &mut Gl,
        shader_type: ShaderType,
        source: &str,
    ) -> Result<Self, ShaderError> {
        // Create an instance of the shader to make sure that no matter what,
        // if this is dropped, it will be cleaned up.
        let shader = Self {
            gl: gl.clone(),
            handle: {
                // Create a GL shader and return an error if it fails
                let handle = unsafe { gl.0.CreateShader(shader_type.gl_enum()) };
                if handle == 0 {
                    return Err(ShaderError::CreateShaderFailed);
                }
                handle
            },
        };

        // Convert the input source into a format that the C OpenGL api can
        // understand
        let c_str = CString::new(source).map_err(|_| ShaderError::InvalidSourceString)?;

        unsafe {
            // Load the shader source into OpenGL
            gl.0.ShaderSource(shader.handle, 1, &c_str.as_ptr(), std::ptr::null());

            // Try to compile the shader from the provided source
            gl.0.CompileShader(shader.handle);
        }

        // Check for compilation errors
        gl_error_check!(
            gl,
            shader.handle,
            GetShaderiv,
            GetShaderInfoLog,
            CompileError
        )?;

        // Return the shader because it was successfully compiled
        Ok(shader)
    }
}

impl Drop for Shader {
    // Automatically delete this shader when this struct is dropped.
    fn drop(&mut self) {
        println!("Dropping shader {}", self.handle);

        unsafe {
            self.gl.0.DeleteShader(self.handle);
        }
    }
}

pub struct ShaderProgram {
    gl: Gl,
    handle: GLuint,
    _uniforms: HashMap<String, GLuint>,
}

impl ShaderProgram {
    pub fn from_shaders(
        gl: &mut Gl,
        vertex_shader: Option<Shader>,
        geometry_shader: Option<Shader>,
        fragment_shader: Option<Shader>,
    ) -> Result<Self, ShaderError> {
        let program = Self {
            gl: gl.clone(),
            handle: {
                // Create a GL shader and return an error if it fails
                let handle = unsafe { gl.0.CreateProgram() };
                if handle == 0 {
                    return Err(ShaderError::CreateShaderProgramFailed);
                }
                handle
            },
            _uniforms: HashMap::new(),
        };

        // Attach the shaders if they are provided
        macro_rules! attach_shader {
            ($shader:expr) => {
                // Use a reference so this macro doesn't consume the shader
                if let Some(shader) = &$shader {
                    unsafe {
                        gl.0.AttachShader(program.handle, shader.handle);
                    }
                    Some(shader.handle)
                } else {
                    None
                }
            };
        }
        let vs = attach_shader!(vertex_shader);
        let gs = attach_shader!(geometry_shader);
        let fs = attach_shader!(fragment_shader);

        // Link the program
        unsafe {
            gl.0.LinkProgram(program.handle);
        }

        // Check for link errors
        gl_error_check!(
            gl,
            program.handle,
            GetProgramiv,
            GetProgramInfoLog,
            ValidateError
        )?;

        // Detach the shaders so they can be deleted when this function
        // invocation ends
        macro_rules! detach_shader {
            ($shader:expr) => {
                if let Some(shader) = $shader {
                    unsafe {
                        gl.0.DetachShader(program.handle, shader);
                    }
                }
            };
        }
        detach_shader!(vs);
        detach_shader!(gs);
        detach_shader!(fs);

        /* The shaders will be dropped after this as they are no longer needed */

        // Return the program
        Ok(program)
    }

    /// Checks whether this program could be executed given the current OpenGL
    /// application state. If there are errors that occur that are difficult
    /// to trace, this method may reveal what the issue is other than just
    /// "Invalid operation."
    pub fn validate(&mut self) -> Result<(), ShaderError> {
        // Validate the program
        unsafe {
            self.gl.0.ValidateProgram(self.handle);
        }

        // Check for validation errors
        gl_error_check!(
            self.gl,
            self.handle,
            GetProgramiv,
            GetProgramInfoLog,
            ValidateError
        )
    }

    pub fn bind(&mut self) {
        unsafe {
            self.gl.0.UseProgram(self.handle);
        }
    }
}

impl Drop for ShaderProgram {
    // Automatically delete the program
    fn drop(&mut self) {
        println!("Dropping program {}", self.handle);

        unsafe {
            self.gl.0.DeleteProgram(self.handle);
        }
    }
}
