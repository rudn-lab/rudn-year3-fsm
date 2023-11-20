use fsm::fsm::StateMachine;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::use_interval;

#[wasm_bindgen]
extern "C" {
    fn prepare_canvas(c: Element);
    fn fetch_json() -> String;
    fn load_from_json(v: String);
}

#[derive(Properties, PartialEq, Clone)]
pub struct CanvasProps {
    #[prop_or_default]
    pub onchange: Callback<StateMachine>,

    #[prop_or_default]
    pub init: StateMachine,
}

#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {
    let CanvasProps { onchange, init } = props;
    let canvas_ref = use_node_ref();
    let force_update = use_force_update();
    let state = use_state_eq(StateMachine::default);
    use_interval(move || force_update.force_update(), 100);

    {
        let canvas_ref = canvas_ref.clone();
        use_effect_with(canvas_ref, |canvas_ref| {
            prepare_canvas(canvas_ref.cast().unwrap());
        });
    }

    use_effect_with(init.clone(), |machine| {
        load_from_json(serde_json::to_string(machine).unwrap());
    });

    let text = fetch_json();
    let sm: StateMachine = serde_json::from_str(&text).expect("JS code produced invalid JSON?");
    if &sm != &*state {
        state.set(sm.clone());
        onchange.emit(sm);
    }

    html!(
        <>
            <canvas ref={canvas_ref} width="800" height="600" style="max-width: 800px; background: black; border-radius: 20px; margin: 10px auto; border: 1px white solid;">
            </canvas>
        </>
    )
}
