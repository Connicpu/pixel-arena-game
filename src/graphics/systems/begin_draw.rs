use crate::Data;

use failure::ResultExt;

#[derive(Default, conniecs::System)]
#[process]
pub struct BeginDraw;

fn process(_: &mut BeginDraw, data: &mut Data) {
    let graphics = &mut data.services.graphics;

    graphics.camera.update_aspect(&graphics.core);
    graphics.camera.upload();

    graphics
        .frame
        .begin_frame(&graphics.core)
        .context("BeginDraw::process")
        .unwrap();
}
