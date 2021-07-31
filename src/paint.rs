use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Called by our JS entry point to run the example
// #[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let document = web_sys::window().unwrap().document().unwrap();
    // let canvas = document.get_element_by_id("blackboard").unwrap();
    // let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()
    // .map_err(|_| ())
    // .unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(640);
    canvas.set_height(640);
    canvas.style().set_property("border", "solid")?;

    document.body().unwrap().append_child(&canvas)?;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);

            // context.set_fill_style(&"rgb(150,50,0)".into());
            // context.fill_rect(0.0, 0.0, 100.0, 100.0);
            // context.set_fill_style(&"rgb(0, 150, 150)".into());
            // context.fill_rect(10.0, 10.0, 80.0, 80.0);
            // context.set_fill_style(&"rgb(150, 0, 150)".into());
            // context.fill_rect(20.0, 20.0, 60.0, 60.0);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if (pressed.get()) {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
