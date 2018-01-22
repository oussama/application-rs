use events::*;
use stdweb::*;
use std::cell::RefCell;
use std::rc::Rc;
use AppConfig;

pub struct App {
    pub events: Vec<Event>,
    pub _events: Rc<RefCell<Vec<Event>>>,
    pub callback: Option<Box<FnMut(&mut App)>>,
}

impl App {

    pub fn new(config: AppConfig) -> App {
        use stdweb::web::*;

        let _ = initialize();
        let canvas = document().create_element("canvas");

        js!{ (@{&canvas}).width = @{config.size.0} ; @{&canvas}.height = @{config.size.1};  };

        document()
            .query_selector("body")
            .unwrap()
            .append_child(&canvas);

        use stdweb::web::event::*;
        
        use events::{ElementState, Event, KeyboardInput, WindowEvent};

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
         //   window: canvas,
            events: Vec::new(),
            _events,
            callback:None,
        }
    }

    pub fn canvas(&self) -> &str {
        "canvas"
    }
    
    pub fn run<F>(mut self, mut callback: F)  where F: 'static+FnMut(&mut App) -> () 
    {
        
        use stdweb::web::*;
        
        window().request_animation_frame(move |_t: f64| {
            let events = self._events.borrow().clone();
            self._events.borrow_mut().clear();
            self.events = events;
            callback(&mut self);
            self.run(callback);
        });

        event_loop();
    }
}