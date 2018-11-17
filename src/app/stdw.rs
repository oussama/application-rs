use events::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use AppConfig;

use std::cmp;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// use web_sys::console;

#[wasm_bindgen]
/** Interface handle to event loop, from javascript, also provides for memory management
 * by keeping references to structures.
 */
pub struct RenderLoopHandle {
    /** handle to inner render loop state structure */
    #[allow(dead_code)]
    render_loop: Rc<RefCell<RenderLoop>>,
    /** holds on to the closures until they can be dropped */
    #[allow(dead_code)]
    closures: Vec<Box<Drop>>,
}

impl RenderLoopHandle {
    pub fn render_loop(&self) -> Rc<RefCell<RenderLoop>> {
        self.render_loop.clone()
    }
}

use render_loop::*;

pub struct App {
    pub canvas: HtmlCanvasElement,
    pub events: Vec<Event>,
    pub _events: Rc<RefCell<Vec<Event>>>,
    pub render_loop: Option<Rc<RefCell<RenderLoop>>>,
}

impl App {
    pub fn new(config: AppConfig) -> Result<(Rc<RefCell<App>>, RenderLoopHandle), JsValue> {
        let window = window().expect("window not found");
        let document = window.document().expect("document not found");

        let mut closures: Vec<Box<Drop>> = vec![];

        let canvas: HtmlCanvasElement = document
            .create_element("canvas")
            .expect("canvas creation failed")
            .dyn_into::<HtmlCanvasElement>()
            .expect("Not canvas");

        let node: Node = document.body().expect("body").into();
        node.append_child(&canvas.clone().into());

        canvas.set_width(config.size.0);
        canvas.set_height(config.size.1);

        use events::{ElementState, Event, KeyboardInput, WindowEvent};

        let _events = Rc::new(RefCell::new(Vec::<Event>::new()));

        let evs1 = _events.clone();

        let onkeyup = Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            let input = KeyboardInput::from_keyboard_event(&ev, ElementState::Pressed);
            let event = WindowEvent::KeyboardInput {
                device_id: DeviceId,
                input,
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        }) as Box<FnMut(KeyboardEvent)>);

        window.set_onkeyup(Some(&onkeyup.as_ref().unchecked_ref()));

        let evs1 = _events.clone();

        let onkeydown = Closure::wrap(Box::new(move |ev: KeyboardEvent| {
            let input = KeyboardInput::from_keyboard_event(&ev, ElementState::Released);
            let event = WindowEvent::KeyboardInput {
                device_id: DeviceId,
                input,
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        }) as Box<FnMut(KeyboardEvent)>);

        window.set_onkeyup(Some(&onkeydown.as_ref().unchecked_ref()));

        let evs1 = _events.clone();

        let onmousedown = Closure::wrap(Box::new(move |ev: MouseEvent| {
            use events::MouseButton;
            let event = WindowEvent::MouseInput {
                device_id: DeviceId,
                state: ElementState::Pressed,
                button: MouseButton::from_mouse_button(ev.button()),
                modifiers: ModifiersState::default(),
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        }) as Box<FnMut(MouseEvent)>);

        window.set_onmousedown(Some(&onmousedown.as_ref().unchecked_ref()));

        let evs1 = _events.clone();

        let onmouseup = Closure::wrap(Box::new(move |ev: MouseEvent| {
            use events::MouseButton;
            let event = WindowEvent::MouseInput {
                device_id: DeviceId,
                state: ElementState::Released,
                button: MouseButton::from_mouse_button(ev.button()),
                modifiers: ModifiersState::default(),
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        }) as Box<FnMut(MouseEvent)>);

        window.set_onmouseup(Some(&onmouseup.as_ref().unchecked_ref()));

        let evs1 = _events.clone();

        let onmousemove = Closure::wrap(Box::new(move |ev: MouseEvent| {
            let event = WindowEvent::CursorMoved {
                device_id: DeviceId,
                position: (ev.client_x() as _, ev.client_y() as _),
                modifiers: ModifiersState::default(),
            };
            evs1.borrow_mut().push(Event::WindowEvent {
                window_id: WindowId,
                event,
            });
        }) as Box<FnMut(MouseEvent)>);

        window.set_onmousemove(Some(&onmousemove.as_ref().unchecked_ref()));

        let evs1 = _events.clone();

        closures.push(Box::new(onkeydown));
        closures.push(Box::new(onkeyup));
        closures.push(Box::new(onmousedown));
        closures.push(Box::new(onmouseup));
        closures.push(Box::new(onmousemove));

        let app = Rc::new(RefCell::new(App {
            canvas,
            //   window: canvas,
            events: Vec::new(),
            _events,
            render_loop: None,
        }));

        let mut app1 = app.clone();

        //   app.closures.push(Box::new(onmousemove));

        // Render loop handling
        let render_loop: Rc<RefCell<RenderLoop>> = Rc::new(RefCell::new(RenderLoop::new(
            window.clone(),
            app.clone(),
            None,
        )));

        app.borrow_mut().render_loop = Some(render_loop.clone());

        render_loop.borrow_mut().closure = Some({
            let render_loop = render_loop.clone();
            Closure::wrap(Box::new(move |time: f64| {
                render_loop.borrow_mut().render_loop(time);
            }))
        });

        //     render_loop.borrow_mut().play()?;

        Ok((
            app,
            RenderLoopHandle {
                render_loop,
                closures,
            },
        ))

        //   app
        /*
        Rc::new(RefCell::new(App {
            //   window: canvas,
            events: Vec::new(),
            _events: Rc::new(RefCell::new(Vec::new())),
            callback: None,
            render_loop: None,
        }))*/    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    pub fn gl(&self) -> WebGl2RenderingContext {
        self.canvas()
            .get_context("webgl2")
            .expect("get_context failed")
            .expect("get_context not found")
            .dyn_into::<WebGl2RenderingContext>()
            .expect("Not WebGl2RenderingContext")
    }
}

impl KeyboardInput {
    pub fn from_keyboard_event(ev: &KeyboardEvent, state: ElementState) -> KeyboardInput {
        KeyboardInput {
            scancode: 0,
            state,
            virtual_keycode: VirtualKeyCode::from_key(ev.key()),
            modifiers: ModifiersState {
                shift: ev.shift_key(),
                ctrl: ev.ctrl_key(),
                alt: ev.alt_key(),
                logo: false,
            },
        }
    }
}
