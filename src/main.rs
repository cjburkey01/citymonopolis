use amazintosh_rs::nalgebra;
use amazintosh_rs::nalgebra::{Affine3, Isometry3, Matrix4, Perspective3, Similarity3, Vector3};
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
    position: Vector3<f32>,
    color: Vector3<f32>,
}

impl PosVert {
    pub fn new(position: Vector3<f32>, color: Vector3<f32>) -> Self {
        Self { position, color }
    }
}

// TODO: AUTO-IMPLEMENT
impl Vertex for PosVert {
    fn attrib_pointers() -> Vec<VertexAttribPointer> {
        vec![
            VertexAttribPointer::new::<f32>(0, 3, false, 0),
            VertexAttribPointer::new::<f32>(1, 3, false, std::mem::size_of::<Vector3<f32>>()),
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
            vec!["projection", "view", "object"],
        )
        .expect("failed to link program")
    };

    let test_mesh = {
        let mut test_mesh = Mesh::new(&mut render);

        let mesh_verts = vec![
            PosVert::new(Vector3::new(0.0, 0.5, 0.0), Vector3::new(1.0, 0.0, 0.0)),
            PosVert::new(Vector3::new(-0.5, -0.5, 0.0), Vector3::new(0.0, 1.0, 0.0)),
            PosVert::new(Vector3::new(0.5, -0.5, 0.0), Vector3::new(0.0, 0.0, 1.0)),
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

                // TODO:
                let (width, height) = window.size();
                let projection = Perspective3::new(
                    width as f32 / height as f32,
                    std::f32::consts::PI / 2.0,
                    0.1,
                    100.0,
                )
                .to_homogeneous();
                let view: Matrix4<f32> = nalgebra::convert(Isometry3::new(
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(0.0, 0.0, 0.0),
                ));
                let object: Matrix4<f32> = nalgebra::convert(Similarity3::new(
                    Vector3::new(0.0, 0.0, -1.0),
                    Vector3::new(0.0, 0.0, 0.0),
                    1.0,
                ));

                app_state.test_shaders.bind();
                app_state.test_shaders.uniform("projection", projection);
                app_state.test_shaders.uniform("view", view);
                app_state.test_shaders.uniform("object", object);

                app_state.test_mesh.render();
                // END TODO

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
