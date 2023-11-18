use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::use_interval;

#[wasm_bindgen]
extern "C" {
    fn prepare_canvas(c: Element);
    fn fetch_json() -> String;
}

#[function_component(Canvas)]
pub fn canvas() -> Html {
    let canvas_ref = use_node_ref();
    let force_update = use_force_update();
    use_interval(move || force_update.force_update(), 100);

    {
        let canvas_ref = canvas_ref.clone();
        use_effect_with(canvas_ref, |canvas_ref| {
            prepare_canvas(canvas_ref.cast().unwrap());
        });
    }

    let text = fetch_json();

    html!(
        <>
            <canvas ref={canvas_ref} width="800" height="600" style="max-width: 800px; background: black; border-radius: 20px; margin: 10px auto; border: 1px white solid;">
            </canvas>
            <p>{text}</p>
        </>
    )
}
