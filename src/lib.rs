
#![feature(nll)]

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(not(target_arch = "wasm32"))]
extern crate glutin;

#[cfg(not(target_arch = "wasm32"))]
use std::os::raw::c_void;

pub struct AppConfig {
    pub title:String,
    pub size:(u32,u32),
    pub vsync:bool,
}

impl AppConfig {
    pub fn new<T:Into<String>>(title:T,size:(u32,u32)) -> AppConfig {
        AppConfig {
            title:title.into(),
            size,
            vsync:true
        }
    }
}


#[cfg(target_arch = "wasm32")]
fn request_animation_frame() {
    js!{ window.requestAnimationFrame(function(){
        var event = new Event("animationFrame");
        window.dispatchEvent(event);
    }) };
}

#[cfg(not(target_arch = "wasm32"))]
pub struct App {
    window: glutin::GlWindow,
    events_loop: glutin::EventsLoop,
}

#[cfg(not(target_arch = "wasm32"))]
impl App {
    pub fn new(config:AppConfig) -> App {
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
        }
    }

    pub fn window(&self) -> &glutin::GlWindow {
        &self.window
    }

    pub fn get_proc_address(&self,name:&str) -> *const c_void {
        use glutin::GlContext;
        self.window().get_proc_address(name) as *const c_void
    }

    pub fn canvas(&self) -> &isize {
        &0
    }

    pub fn run<F>(&mut self, mut callback: F)
    where
        F: FnMut() -> (),
    {
        let (window, events_loop) = (&self.window, &mut self.events_loop);
        use glutin::*;
        let mut running = true;
        while running {
            events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => window.resize(w, h),
                    _ => (),
                },
                _ => (),
            });

            callback();
            window.swap_buffers().unwrap();
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub struct App {
    window: stdweb::web::Element,
}

#[cfg(target_arch = "wasm32")]
impl App {
    pub fn new(config:AppConfig) -> App {
        use stdweb::web::*;

        let _ = stdweb::initialize();
        let canvas = document().create_element("canvas");

        js!{ (@{&canvas}).width = @{config.size.0} ; @{&canvas}.height = @{config.size.1};  };
        
        document()
            .query_selector("body")
            .unwrap()
            .append_child(&canvas);
        App {
            window: canvas,
        }
    }

    pub fn canvas(&self) -> &stdweb::web::Element {
        &self.window
    }

    pub fn run<'a, F>(&mut self, callback: F)
    where
        F: 'static + Fn() -> (),
    {
        use stdweb::Value;
        use stdweb;
        let game_loop = move |_: Value| {
            callback();
            request_animation_frame();
        };

        js!{ window.addEventListener("animationFrame",@{game_loop})};

        request_animation_frame();

        stdweb::event_loop();
    }
}
