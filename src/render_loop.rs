/*** EVENT HANDLING ***/
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use app::App;
use log;

pub struct RenderLoop {
    app: Rc<RefCell<App>>,
    window: web_sys::Window,
    pub callback: Option<Box<FnMut(&mut App)>>,
    animation_id: Option<i32>,
    pub closure: Option<Closure<Fn(f64)>>,
}

impl RenderLoop {
    pub fn new(
        window: web_sys::Window,
        app: Rc<RefCell<App>>,
        callback: Option<Box<FnMut(&mut App)>>,
    ) -> RenderLoop {
        RenderLoop {
            app,
            window,
            callback,
            animation_id: None,
            closure: None,
        }
    }

    pub fn render_loop(&mut self, _time: f64) {
        if let Some(ref mut callback) = self.callback {
            let app = &mut self.app.borrow_mut();
            callback(app);
            app._events.borrow_mut().clear();
        }

        self.animation_id = if let Some(ref closure) = self.closure {
            Some(
                self.window
                    .request_animation_frame(closure.as_ref().unchecked_ref())
                    .expect("cannot set animation frame"),
            )
        } else {
            None
        }
    }

    pub fn is_paused(&self) -> bool {
        self.animation_id.is_none()
    }

    pub fn play(&mut self) -> Result<(), JsValue> {
        self.render_loop(0.0);
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), JsValue> {
        if let Some(id) = self.animation_id {
            self.window.cancel_animation_frame(id)?;
            self.animation_id = None;
        }
        Ok(())
    }

    pub fn play_pause(&mut self) -> Result<(), JsValue> {
        if self.is_paused() {
            self.play()?;
        } else {
            self.pause()?;
        }
        Ok(())
    }

    pub fn run<F>(&mut self, mut callback: F)
    where
        F: 'static + FnMut(&mut App) -> (),
    {
        log("APP.RUN");
        self.callback = Some(Box::new(callback));
        self.play();
    }
}
