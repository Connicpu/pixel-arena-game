use crate::DataHelper;

use failure::ResultExt;

#[derive(Default, System)]
#[process]
pub struct EndDraw;

fn process(_: &mut EndDraw, data: &mut DataHelper) {
    data.services
        .graphics
        .frame
        .end_frame()
        .context("EndDraw::process")
        .unwrap();
}
