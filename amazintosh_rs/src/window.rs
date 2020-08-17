use crate::render::Gl;
use sdl2::event::{Event, WindowEvent};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub trait AWindow<ContextType, WindowEventType> {
    type ErrorType;

    fn set_title(&mut self, title: &str) -> Result<(), Self::ErrorType>;

    fn set_size(&mut self, size: (usize, usize)) -> Result<(), Self::ErrorType>;

    fn hide(&mut self) -> Result<(), Self::ErrorType>;

    fn ctx(&mut self) -> Option<ContextType>;

    fn start_loop<
        DataType,
        PreRender: Fn(&mut Self, &mut DataType) -> bool,
        PostRender: Fn(&mut Self, &mut DataType) -> bool,
        EventHandler: Fn(&mut Self, &mut DataType, WindowEventType) -> bool,
    >(
        self,
        data: DataType,
        pre_render: PreRender,
        post_render: PostRender,
        event_handler: EventHandler,
    ) -> Result<(), Self::ErrorType>;
}

#[derive(Debug)]
pub enum SdlWindowError {
    ContextInitFailed(String),
    VideoSubsystem(String),
    WindowCreateFailed(sdl2::video::WindowBuildError),
    GlContextCreateFailed(String),
    IntegerOverflow,
    InitEventPumpFailed(String),
    NulError,
    FullscreenErr(String),
}

impl Display for SdlWindowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SdlWindowError {}

pub struct SdlWindow {
    sdl_context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    _gl_ctx: sdl2::video::GLContext,
    gl: Gl,
}

impl SdlWindow {
    pub fn new(title: &str, width: usize, height: usize) -> Result<Self, SdlWindowError> {
        // Initialize SDL2
        let sdl_context = sdl2::init().map_err(|e| SdlWindowError::ContextInitFailed(e))?;

        // Get SDL2's video subsystem to get information about video/rendering
        let video_subsystem = sdl_context
            .video()
            .map_err(|e| SdlWindowError::VideoSubsystem(e))?;

        // Set the GL versions
        let gl_attrs = video_subsystem.gl_attr();
        gl_attrs.set_accelerated_visual(true);
        gl_attrs.set_context_version(3, 3);
        gl_attrs.set_context_profile(sdl2::video::GLProfile::Core);

        // Create the window
        let mut window = video_subsystem
            .window(title, width as u32, height as u32)
            .resizable()
            .opengl()
            .build()
            .map_err(|e| SdlWindowError::WindowCreateFailed(e))?;

        window
            .set_fullscreen(sdl2::video::FullscreenType::Off)
            .map_err(|e| SdlWindowError::FullscreenErr(e))?;

        // Get the OpenGL context for the window
        let gl_ctx = window
            .gl_create_context()
            .map_err(|e| SdlWindowError::GlContextCreateFailed(e))?;

        // Create an Amazintosh GL wrapper
        let gl = super::render::Gl::new(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        // Return this wrapper and keep everything alive as long as it needs to be
        Ok(Self {
            sdl_context,
            _video_subsystem: video_subsystem,
            window,
            _gl_ctx: gl_ctx,
            gl,
        })
    }
}

impl AWindow<Gl, sdl2::event::Event> for SdlWindow {
    type ErrorType = SdlWindowError;

    fn set_title(&mut self, title: &str) -> Result<(), Self::ErrorType> {
        self.window
            .set_title(title)
            .map_err(|_| SdlWindowError::NulError)
    }

    fn set_size(&mut self, size: (usize, usize)) -> Result<(), Self::ErrorType> {
        self.window
            .set_size(size.0 as u32, size.1 as u32)
            .map_err(|_| SdlWindowError::IntegerOverflow)
    }

    fn hide(&mut self) -> Result<(), Self::ErrorType> {
        self.window.hide();

        // No errors possible
        Ok(())
    }

    fn ctx(&mut self) -> Option<Gl> {
        Some(self.gl.clone())
    }

    fn start_loop<
        DataType,
        PreRender: Fn(&mut Self, &mut DataType) -> bool,
        PostRender: Fn(&mut Self, &mut DataType) -> bool,
        EventHandler: Fn(&mut Self, &mut DataType, sdl2::event::Event) -> bool,
    >(
        mut self,
        mut data: DataType,
        pre_render: PreRender,
        post_render: PostRender,
        event_handler: EventHandler,
    ) -> Result<(), Self::ErrorType> {
        // Initialize the event pump to poll events from SDL
        let mut event_pump = self
            .sdl_context
            .event_pump()
            .map_err(|e| SdlWindowError::InitEventPumpFailed(e))?;

        // Start the loop and label it to allow breaking out of it
        'running: loop {
            // Run the pre-mod callback
            if pre_render(&mut self, &mut data) {
                break 'running;
            }

            // Swap the buffers
            self.window.gl_swap_window();

            // Run the post-mod callback
            if post_render(&mut self, &mut data) {
                break 'running;
            }

            // Run the event handler for all the events
            for event in event_pump.poll_iter() {
                match &event {
                    // Update GL viewport on resize
                    &Event::Window {
                        win_event: WindowEvent::Resized(w, h),
                        ..
                    } => unsafe {
                        self.gl.0.Viewport(0, 0, w, h);
                    },
                    _ => {}
                }

                if event_handler(&mut self, &mut data, event) {
                    break 'running;
                }
            }
        }

        // Done with the loop and no errors
        Ok(())
    }
}
