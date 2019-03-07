use crate::Data;

use failure::ResultExt;

#[derive(Default, conniecs::System)]
#[process]
pub struct EndDraw;

fn process(_: &mut EndDraw, data: &mut Data) {
    data.services
        .graphics
        .frame
        .end_frame()
        .context("EndDraw::process")
        .unwrap();
}
