use events::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use AppConfig;

pub struct App {
    pub events: Vec<Event>,
    pub _events: Rc<RefCell<Vec<Event>>>,
    pub callback: Option<Box<FnMut(&mut App)>>,
}

impl App {
    pub fn new(_config: AppConfig) -> App {
        let _events = Rc::new(RefCell::new(Vec::new()));

        App {
            //   window: canvas,
            events: Vec::new(),
            _events,
            callback: None,
        }
    }

    pub fn canvas(&self) -> &str {
        "canvas"
    }

    pub fn main_loop(&mut self, _t: f64) {
        let events = self._events.borrow().clone();
        self._events.borrow_mut().clear();
        self.events = events;
        let callback_option = self.callback.take();
        if let Some(mut callback) = callback_option {
            callback(self);
            self.callback = Some(callback);
        }
    }

    pub fn run<F>(mut self, callback: F)
    where
        F: 'static + FnMut(&mut App) -> (),
    {
        self.callback = Some(Box::new(callback));
    }
}
