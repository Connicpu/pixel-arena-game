use crate::DataHelper;

#[derive(Default, System)]
#[process]
pub struct WindowEvents;

fn process(_: &mut WindowEvents, data: &mut DataHelper) {
    let events = &mut data.services.graphics.core.events_loop;
    let quit_flag = &mut data.services.quit_flag;
    let display = &data.services.graphics.core.display;
    let is_fullscreen = &mut data.services.graphics.is_fullscreen;

    events.poll_events(|event| {
        use winit::{
            ElementState::Pressed,
            Event::WindowEvent,
            VirtualKeyCode as VK,
            WindowEvent::{CloseRequested, KeyboardInput},
        };

        if let WindowEvent { event, .. } = event {
            match event {
                CloseRequested => *quit_flag = true,

                KeyboardInput { input, .. } => match (input.virtual_keycode, input.state) {
                    (Some(VK::Return), Pressed) if input.modifiers.alt => {
                        *is_fullscreen = !*is_fullscreen;
                        let win = display.gl_window();
                        if *is_fullscreen {
                            win.set_fullscreen(Some(win.get_current_monitor()));
                        } else {
                            win.set_fullscreen(None);
                        }
                    }
                    _ => (),
                },

                _ => (),
            }
        }
    });
}
