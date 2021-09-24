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

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}


// Parse QML
// Canvas renderer
// Framebuffer renderer
