use amazintosh_rs::glm::Vec3;
use amazintosh_rs::render::inner_gl;
use amazintosh_rs::render::inner_gl::types::GLvoid;
use amazintosh_rs::render::mesh::Mesh;
use amazintosh_rs::render::shader::{Shader, ShaderProgram, ShaderType};
use amazintosh_rs::render::types::RGBAColor;
use amazintosh_rs::render::vertex::{BufferUsage, Vertex, VertexAttribPointer};
use amazintosh_rs::render::Gl;
use amazintosh_rs::sdl2::event::Event;
use amazintosh_rs::sdl2::event::WindowEvent;
use amazintosh_rs::sdl2::keyboard::Keycode;
use amazintosh_rs::window::{AWindow, SdlWindow};
use std::convert::TryInto;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PosVert {
    position: Vec3,
    color: Vec3,
}

impl PosVert {
    pub fn new(position: Vec3, color: Vec3) -> Self {
        Self { position, color }
    }
}

// TODO: AUTO-IMPLEMENT
impl Vertex for PosVert {
    fn setup_attrib_pointers() -> Vec<VertexAttribPointer> {
        vec![
            VertexAttribPointer::new::<f32>(3, false, 0),
            VertexAttribPointer::new::<f32>(3, false, std::mem::size_of::<Vec3>()),
        ]
    }
}

struct AppState {
    test_shaders: ShaderProgram,
    test_mesh: Mesh<PosVert>,
}

fn main() {
    let mut window = SdlWindow::new(
        concat!("CityMonopolis v", env!("CARGO_PKG_VERSION")),
        300,
        300,
    )
    .expect("failed to create window");

    let mut gl = window.ctx().expect("failed to get OpenGL context");

    gl.set_clear_color(RGBAColor::from_rgb(0.2, 0.3, 0.4));

    let test_shaders = {
        let vertex_shader = Shader::create_shader(
            &mut gl,
            ShaderType::Vertex,
            include_str!("./vertex_test.glsl"),
        )
        .expect("failed to compile vertex shader");
        let fragment_shader = Shader::create_shader(
            &mut gl,
            ShaderType::Fragment,
            include_str!("./fragment_test.glsl"),
        )
        .expect("failed to compile fragment shader");

        ShaderProgram::from_shaders(&mut gl, Some(vertex_shader), None, Some(fragment_shader))
            .expect("failed to link program")
    };

    let test_mesh = {
        let mut test_mesh = Mesh::new(&mut gl).expect("failed to create test mesh");

        let mesh_verts = vec![
            PosVert::new(Vec3::new(0.0, 0.5, 0.0), Vec3::new(1.0, 0.0, 0.0)),
            PosVert::new(Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            PosVert::new(Vec3::new(0.5, -0.5, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        ];
        let mesh_inds = vec![0, 1, 2];

        test_mesh
            .update_data(mesh_verts, mesh_inds, BufferUsage::StaticDraw)
            .expect("failed to upload test mesh data to GPU");

        test_mesh
    };

    window
        .start_loop(
            AppState {
                test_shaders,
                test_mesh,
            },
            |window, app_state| {
                if let Some(mut gl) = window.ctx() {
                    gl.clear(true, false);
                }

                app_state.test_shaders.bind();

                app_state.test_mesh.render();

                false
            },
            |_, _| false,
            |_, _, e| match e {
                Event::Window {
                    win_event: WindowEvent::Close,
                    ..
                } => true,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => true,
                _ => false,
            },
        )
        .expect("failed to start mod loop");
}
