use crate::graphics::core::GraphicsCore;

use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::{DepthTexture2dMultisample, SrgbTexture2dMultisample};
use glium::{uniforms as u, BlitTarget, Surface};

use failure::ResultExt;

#[derive(Default)]
pub struct CurrentFrame {
    window_frame: Option<glium::Frame>,
    gameplay_frame: Option<SimpleFrameBuffer<'static>>,
    gameplay_cbuffer: Option<SrgbTexture2dMultisample>,
    gameplay_dbuffer: Option<DepthTexture2dMultisample>,
}

impl CurrentFrame {
    pub fn gameplay_frame(&mut self) -> Option<&mut SimpleFrameBuffer<'static>> {
        self.gameplay_frame.as_mut()
    }

    pub fn begin_frame(&mut self, core: &GraphicsCore) -> Result<(), failure::Error> {
        self.gameplay_frame = None;
        self.window_frame = None;

        let frame = core.display.draw();

        if self
            .gameplay_cbuffer
            .as_ref()
            .map(|f| f.dimensions() != frame.get_dimensions())
            .unwrap_or(true)
        {
            let (width, height) = frame.get_dimensions();
            let cbuffer = SrgbTexture2dMultisample::empty(&core.display, width, height, 4)
                .context("creating gameplay buffer color attachment")?;
            let dbuffer = DepthTexture2dMultisample::empty(&core.display, width, height, 4)
                .context("creatime gameplay buffer depth attachment")?;
            self.gameplay_cbuffer = Some(cbuffer);
            self.gameplay_dbuffer = Some(dbuffer);
        }

        let mut gameplay_frame = {
            // x.x it's a bad trick, but this struct encapsulates the invariants,
            // including the drop order, so it should be ~fiiiiiine~
            let cb = unsafe { &*(self.gameplay_cbuffer.as_ref().unwrap() as *const _) };
            let db = unsafe { &*(self.gameplay_dbuffer.as_ref().unwrap() as *const _) };
            SimpleFrameBuffer::with_depth_buffer(&core.display, cb, db)
                .context("creating gameplay framebuffer")?
        };
        
        gameplay_frame.clear_color_srgb_and_depth((0.5, 0.5, 0.5, 1.0), 0.0);

        self.window_frame = Some(frame);
        self.gameplay_frame = Some(gameplay_frame);

        Ok(())
    }

    pub fn end_frame(&mut self) -> Result<(), failure::Error> {
        let frame = self.window_frame.take().unwrap();

        // Just blit the frame if we didn't do some kind of post-process effect with it already
        if let Some(buf) = self.gameplay_frame.take() {
            let (width, height) = frame.get_dimensions();
            let target = BlitTarget {
                left: 0,
                bottom: 0,
                height: height as i32,
                width: width as i32,
            };
            buf.blit_whole_color_to(&frame, &target, u::MagnifySamplerFilter::Nearest);
        }

        frame.finish().context("finishing frame")?;

        Ok(())
    }
}
