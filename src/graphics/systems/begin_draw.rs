use crate::DataHelper;

use failure::ResultExt;

#[derive(Default, System)]
#[process]
pub struct BeginDraw;

fn process(_: &mut BeginDraw, data: &mut DataHelper) {
    let graphics = &mut data.services.graphics;

    graphics.camera.upload(&graphics.core);

    graphics
        .frame
        .begin_frame(&graphics.core)
        .context("BeginDraw::process")
        .unwrap();
}
