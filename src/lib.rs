mod utils;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::Cell;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, pilcrow!");
}

// generated
struct State {
    name: String,
    image: web_sys::HtmlImageElement
}

struct MouseArea {
    onPress: fn()
}

// generated
struct ButtonWindow {
    state: String,
    states: [State; 2],
    mouseArea: MouseArea,
    painter: Painter
}

impl ButtonWindow {
    pub fn setState(&mut self, val: String) {
        self.state = val;
        // self.painter.paint(&self);
    }
}

// generated
fn init() -> ButtonWindow {
    // Make painter
    let document = web_sys::window().unwrap().document().unwrap();
    let normal = document.get_element_by_id("normal").unwrap();
    let normal: web_sys::HtmlImageElement = normal
        .dyn_into::<web_sys::HtmlImageElement>()
        .map_err(|_| ())
        .unwrap();

    let pressed = document.get_element_by_id("pressed").unwrap();
    let pressed: web_sys::HtmlImageElement = pressed
        .dyn_into::<web_sys::HtmlImageElement>()
        .map_err(|_| ())
        .unwrap();

    let painter = Painter::new(String::from("canvas"));

    // Make window
    let win = ButtonWindow {
        state: String::from("fdlsa"),
        states: [
            State {
                image: normal,
                name: String::from("fdsl")
            },
            State {
                image: pressed,
                name: String::from("fdsl")
            }
        ],
        mouseArea: MouseArea {
            onPress: || {}
        },
        painter
    };
    win
}

// Provided
fn decorated(win: ButtonWindow) -> ButtonWindow {
    // Set the things in mouseAreas that you want here
    win
}

// in library
struct Painter {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d
}

impl Painter {
    pub fn paint(&self, win: &ButtonWindow) {
        // Find current state
        let image = &win.states[0].image;
        // Paint it
        self.context.draw_image_with_html_image_element(image, 0.0, 0.0);
    }

    pub fn new(canvas_id: String) -> Painter {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&canvas_id[..]).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Painter {
            canvas,
            context
        }
    }
}

// library
#[wasm_bindgen]
pub fn run() {
    let win = decorated(init());
    win.painter.paint(&win);
    // win.setPainter(canvasPainter)
}


#[wasm_bindgen]
pub fn drawSmiley() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    {
        let pressed = pressed.clone();
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(true);
            let document = web_sys::window().unwrap().document().unwrap();
            let normal = document.get_element_by_id("pressed").unwrap();
            let normal: web_sys::HtmlImageElement = normal
                .dyn_into::<web_sys::HtmlImageElement>()
                .map_err(|_| ())
                .unwrap();
            context.draw_image_with_html_image_element(&normal, 0.0, 0.0);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    {
        let pressed = pressed.clone();
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            let document = web_sys::window().unwrap().document().unwrap();
            let normal = document.get_element_by_id("normal").unwrap();
            let normal: web_sys::HtmlImageElement = normal
                .dyn_into::<web_sys::HtmlImageElement>()
                .map_err(|_| ())
                .unwrap();
            context.draw_image_with_html_image_element(&normal, 0.0, 0.0);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    let normal = document.get_element_by_id("pressed").unwrap();
    let normal: web_sys::HtmlImageElement = normal
        .dyn_into::<web_sys::HtmlImageElement>()
        .map_err(|_| ())
        .unwrap();
    context.draw_image_with_html_image_element(&normal, 0.0, 0.0);
}


// Parse QML
// Canvas renderer
// Framebuffer renderer
