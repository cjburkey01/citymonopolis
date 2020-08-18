use amazintosh_rs::glm::Vec3;
use amazintosh_rs::render::buffer::BufferUsage;
use amazintosh_rs::render::mesh::{Mesh, MeshMode};
use amazintosh_rs::render::shader::{Shader, ShaderProgram, ShaderType};
use amazintosh_rs::render::types::RGBAColor;
use amazintosh_rs::render::vertex::{Vertex, VertexAttribPointer};
use amazintosh_rs::render::{Gl, RenderHandler};
use amazintosh_rs::sdl2::event::Event;
use amazintosh_rs::sdl2::event::WindowEvent;
use amazintosh_rs::sdl2::keyboard::Keycode;
use amazintosh_rs::window::{AWindow, SdlWindow};

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
    fn attrib_pointers() -> Vec<VertexAttribPointer> {
        vec![
            VertexAttribPointer::new::<f32>(0, 3, false, 0),
            VertexAttribPointer::new::<f32>(1, 3, false, std::mem::size_of::<Vec3>()),
        ]
    }

    fn render<RHType: RenderHandler>(render_handler: &mut RHType, elements: usize) {
        render_handler.enable_attrib_array(0);
        render_handler.enable_attrib_array(1);
        render_handler.draw_elements::<u16>(MeshMode::Triangles, elements);
        render_handler.disable_attrib_array(0);
        render_handler.disable_attrib_array(1);
    }
}

struct AppState {
    test_shaders: ShaderProgram,
    test_mesh: Mesh<Gl, PosVert, u16>,
}

fn main() {
    let mut window = SdlWindow::new(
        concat!("CityMonopolis v", env!("CARGO_PKG_VERSION")),
        300,
        300,
    )
    .expect("failed to create window");

    let mut render = window.ctx().expect("failed to get OpenGL context");

    let test_shaders = {
        let vertex_shader = Shader::create_shader(
            &mut render,
            ShaderType::Vertex,
            include_str!("./vertex_test.glsl"),
        )
        .expect("failed to compile vertex shader");
        let fragment_shader = Shader::create_shader(
            &mut render,
            ShaderType::Fragment,
            include_str!("./fragment_test.glsl"),
        )
        .expect("failed to compile fragment shader");

        ShaderProgram::from_shaders(
            &mut render,
            Some(vertex_shader),
            None,
            Some(fragment_shader),
        )
        .expect("failed to link program")
    };

    let test_mesh = {
        let mut test_mesh = Mesh::new(&mut render);

        let mesh_verts = vec![
            PosVert::new(Vec3::new(0.0, 0.5, 0.0), Vec3::new(1.0, 0.0, 0.0)),
            PosVert::new(Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            PosVert::new(Vec3::new(0.5, -0.5, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        ];
        let mesh_inds = vec![0, 1, 2];

        test_mesh.set_vertices(mesh_verts, BufferUsage::StaticDraw);
        test_mesh.set_indices(mesh_inds, BufferUsage::StaticDraw);

        test_mesh
    };

    render.set_clear_color(RGBAColor::from_rgb(0.2, 0.3, 0.4));

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
