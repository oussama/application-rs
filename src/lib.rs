#![feature(nll)]


#[macro_use]
extern crate lazy_static;


#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(target_arch = "wasm32")]
pub mod events;

#[cfg(not(target_arch = "wasm32"))]
extern crate glutin;

#[cfg(not(target_arch = "wasm32"))]
pub mod events {
    pub use glutin::*;
}

#[cfg(not(target_arch = "wasm32"))]
use std::os::raw::c_void;
/*

lazy_static! {
    static ref X: Mutex<Option<FnMut()> = Mutex::new(None);
}
*/

pub struct AppConfig {
    pub title: String,
    pub size: (u32, u32),
    pub vsync: bool,
}

impl AppConfig {
    pub fn new<T: Into<String>>(title: T, size: (u32, u32)) -> AppConfig {
        AppConfig {
            title: title.into(),
            size,
            vsync: true,
        }
    }
}

/*
#[cfg(target_arch = "wasm32")]
fn request_animation_frame() {
    js!{ window.requestAnimationFrame(function(){
        var event = new Event("animationFrame");
        window.dispatchEvent(event);
    }) };
}
*/

use events::*;

#[cfg(not(target_arch = "wasm32"))]
pub struct App {
    window: glutin::GlWindow,
    events_loop: glutin::EventsLoop,
    pub events: Vec<Event>,
}

#[cfg(not(target_arch = "wasm32"))]
impl App {
    pub fn new(config: AppConfig) -> App {
        use glutin::*;
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(config.title)
            .with_dimensions(config.size.0, config.size.1);
        let context = glutin::ContextBuilder::new().with_vsync(config.vsync);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }
        App {
            window: gl_window,
            events_loop,
            events: Vec::new(),
        }
    }

    pub fn window(&self) -> &glutin::GlWindow {
        &self.window
    }

    pub fn get_proc_address(&self, name: &str) -> *const c_void {
        use glutin::GlContext;
        self.window().get_proc_address(name) as *const c_void
    }

    pub fn canvas(&self) -> &isize {
        &0
    }

    pub fn run<F>(mut self, mut callback: F)
    where
        F: FnMut(&mut App) -> (),
    {
        use glutin::*;
        let mut running = true;
        while running {
            self.events.clear();
            {
                let (window, events_loop, events) =
                    (&self.window, &mut self.events_loop, &mut self.events);
                events_loop.poll_events(|event| {
                    match event {
                        glutin::Event::WindowEvent { ref event, .. } => match event {
                            &glutin::WindowEvent::Closed => running = false,
                            &glutin::WindowEvent::Resized(w, h) => window.resize(w, h),
                            _ => (),
                        },
                        _ => (),
                    };
                    events.push(event);
                });
            }

            callback(&mut self);
            self.window.swap_buffers().unwrap();
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
pub struct App {
    window: stdweb::web::Element,
    pub events: Vec<Event>,
    pub _events: Rc<RefCell<Vec<Event>>>,
}

#[cfg(target_arch = "wasm32")]
impl App {
    pub fn new(config: AppConfig) -> App {
        use stdweb::web::*;

        let _ = stdweb::initialize();
        let canvas = document().create_element("canvas");

        js!{ (@{&canvas}).width = @{config.size.0} ; @{&canvas}.height = @{config.size.1};  };

        document()
            .query_selector("body")
            .unwrap()
            .append_child(&canvas);

        use stdweb::web::event::*;
        use events::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

        let _events = Rc::new(RefCell::new(Vec::new()));
        let evs1 = _events.clone();
        window().add_event_listener(move |ev: KeydownEvent| {
            let input = KeyboardInput::from_keyboard_event(&ev, ElementState::Pressed);
            let event = WindowEvent::KeyboardInput {
                device_id: DeviceId,
                input,
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        });

        let evs1 = _events.clone();
        window().add_event_listener(move |ev: KeyupEvent| {
            let input = KeyboardInput::from_keyboard_event(&ev, ElementState::Released);
            let event = WindowEvent::KeyboardInput {
                device_id: DeviceId,
                input,
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        });


        let evs1 = _events.clone();
        canvas.add_event_listener(move |ev: MouseDownEvent| {
            use events::MouseButton;
            let event = WindowEvent::MouseInput {
                device_id: DeviceId,
                state: ElementState::Pressed,
                button: MouseButton::from_mouse_button(ev.button()),
                modifiers:ModifiersState::default()
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        });

        let evs1 = _events.clone();
        canvas.add_event_listener(move |ev: MouseUpEvent| {
            use events::MouseButton;
            let event = WindowEvent::MouseInput {
                device_id: DeviceId,
                state: ElementState::Released,
                button: MouseButton::from_mouse_button(ev.button()),
                modifiers:ModifiersState::default()
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        });

        let evs1 = _events.clone();
        canvas.add_event_listener(move |ev: MouseMoveEvent| {
            let event = WindowEvent::CursorMoved {
                device_id: DeviceId,
                position: (ev.client_x() as _, ev.client_y() as _),
                modifiers: ModifiersState::default(),
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        });

        App {
            window: canvas,
            events: Vec::new(),
            _events,
        }
    }

    pub fn canvas(&self) -> &stdweb::web::Element {
        &self.window
    }

    pub fn run<'a, F>(mut self, mut callback: F)
    where
        F: 'static + FnMut(&mut App) -> (),
    {
        use stdweb;
        use stdweb::web::*;

        use stdweb::web::event::*;
        use events::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
        /*
        let evs1 = evs.clone();
        self.window.add_event_listener(move|ev:KeydownEvent|{
            log("key.down");
            let input = KeyboardInput::from_keyboard_event(&ev,ElementState::Pressed);
            let event = WindowEvent::KeyboardInput{device_id:DeviceId,input};
            evs1.borrow_mut().push(Event::WindowEvent{window_id:WindowId,event});
        });*/

        window().request_animation_frame(move |_t: f64| {
            let events = self._events.borrow().clone();
            self._events.borrow_mut().clear();
            self.events = events;
            //log(self.events.len().to_string());
            callback(&mut self);
            self.run(callback);
        });
        /*
        let mut game_loop = move |_: Value| {
            callback();
            request_animation_frame();
        };

        js!{ var gameLoop = @{game_loop}; window.addEventListener("animationFrame",gameLoop)};

        request_animation_frame();
        */
        //stdweb::event_loop();
    }
}

#[cfg(target_arch = "wasm32")]
pub fn log<T: Into<String>>(msg: T) {
    // js!{ console.log(@{msg.into()})};
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log<T: Into<String>>(msg: T) {
    // println!("{}",msg.into() );
}

#[no_mangle]
pub fn update(time:f32){

}