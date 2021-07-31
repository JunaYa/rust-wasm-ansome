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

    document.body().unwrap().append_child(&canvas);

    let mut context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    canvas.set_width(640);
    canvas.set_height(640);

    context.set_fill_style(&"rgb(150,50,0)".into());
    context.fill_rect(0.0, 0.0, 200.0, 200.0);
    context.set_fill_style(&"rgb(0, 150, 150)".into());
    context.fill_rect(10.0, 10.0, 180.0, 180.0);
    context.set_fill_style(&"rgb(150, 0, 150)".into());
    context.fill_rect(20.0, 20.0, 160.0, 160.0);

    Ok(())
}
