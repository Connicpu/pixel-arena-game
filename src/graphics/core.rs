use failure::Error;

pub struct GraphicsCore {
    pub events_loop: winit::EventsLoop,
    pub display: glium::Display,
}

pub fn init_graphics() -> Result<GraphicsCore, Error> {
    let (events_loop, display) = init_window()?;

    Ok(GraphicsCore {
        events_loop,
        display,
    })
}

fn init_window() -> Result<(winit::EventsLoop, glium::Display), Error> {
    let events_loop = winit::EventsLoop::new();

    let wb = winit::WindowBuilder::new()
        .with_dimensions((1280.0, 720.0).into())
        .with_min_dimensions((640.0, 480.0).into())
        .with_title("Pixel Arena Game (working title)");
    // TODO: Icon

    let cb = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Latest)
        .with_depth_buffer(24)
        .with_multisampling(4)
        .with_vsync(true)
        .with_srgb(true);

    let display = glium::Display::new(wb, cb, &events_loop).map_err(failure::SyncFailure::new)?;

    Ok((events_loop, display))
}

impl GraphicsCore {
    pub fn window(&self) -> std::cell::Ref<glutin::GlWindow> {
        self.display.gl_window()
    }
}
