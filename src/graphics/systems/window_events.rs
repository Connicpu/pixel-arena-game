use crate::Data;

#[derive(Default, System)]
#[process]
pub struct WindowEvents;

fn process(_: &mut WindowEvents, data: &mut Data) {
    let events = &mut data.services.graphics.core.events_loop;
    let quit_flag = &mut data.services.quit_flag;
    let display = &data.services.graphics.core.display;
    let camera = &mut data.services.graphics.camera;
    let is_fullscreen = &mut data.services.graphics.is_fullscreen;
    let jump = &mut data.services.jump;

    events.poll_events(|event| {
        use winit::{
            ElementState::{Pressed, Released},
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
                    (Some(VK::D), Pressed) => {
                        camera.position.x += 1.4;
                    }
                    (Some(VK::A), Pressed) => {
                        camera.position.x -= 1.4;
                    }
                    (Some(VK::Space), Pressed) => {
                        *jump = true;
                    }
                    (Some(VK::Space), Released) => {
                        *jump = false;
                    }
                    _ => (),
                },

                _ => (),
            }
        }
    });
}
