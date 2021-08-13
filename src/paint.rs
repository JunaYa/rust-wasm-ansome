use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::canvas;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exist ")
}

fn document() -> web_sys::Document {
    window().document().expect("should have document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have body")
}

fn request_frame_animation(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// Called by our JS entry point to run the example
// #[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let canvas = document()
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;

    body().append_child(&canvas)?;

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
            context.set_stroke_style(&"rgb(80,80,80)".into());
            context.set_line_width(8.0);
            // ctx.lineJoin = "bevel" || "round" || "miter";
            // 不生效
            context.set_line_join("bevel");
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.set_line_join("bevel");
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

#[wasm_bindgen]
#[derive(Debug)]
pub struct Paint {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    pressed: Rc<Cell<bool>>,
}

#[wasm_bindgen]
impl Paint {
    pub fn new() -> Paint {
        // self.pressed = Rc::new(Cell::new(false));
        let canvasVal = Self::init_canvas();
        let context = Self::init_context(canvasVal);

        Paint {
            canvas: canvasVal,
            context: context,
            pressed: Rc::new(Cell::new(false)),
        }
    }

    pub fn init_canvas() -> web_sys::HtmlCanvasElement {
        let canvas = document()
            .create_element("canvas")
            .expect("create canvas fail")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect(" create canvas fail");

        canvas.set_width(640);
        canvas.set_height(480);
        canvas
            .style()
            .set_property("border", "solid")
            .expect("set property fail");
        canvas
    }

    pub fn init_context(canvas: HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
        let context = 
            canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context
    }
}