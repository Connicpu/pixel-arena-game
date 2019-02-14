use crate::DataHelper;

#[derive(Default, System)]
#[process]
pub struct WindowEvents;

fn process(_: &mut WindowEvents, data: &mut DataHelper) {
    let events = &mut data.services.graphics.core.events_loop;
    let quit_flag = &mut data.services.quit_flag;

    events.poll_events(|event| {
        use winit::{Event::WindowEvent, WindowEvent::CloseRequested};
        if let WindowEvent { event, .. } = event {
            match event {
                CloseRequested => *quit_flag = true,
                
                _ => (),
            }
        }
    });
}
