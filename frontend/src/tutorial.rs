use fsm::fsm::StateMachine;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use shadow_clone::shadow_clone;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_autoprops::autoprops_component;

use crate::{canvas_player::CanvasPlayer, task::randfloat};

#[function_component]
fn SampleFSMPlayer() -> Html {
    let word = use_state(|| String::from("110101"));
    let fsm: StateMachine = serde_json::from_str(
        r#"
{"nodes":[{"x":62,"y":70,"text":"","isAcceptState":true},{"x":172,"y":70,"text":"","isAcceptState":false},{"x":286,"y":70,"text":"","isAcceptState":false},{"x":399,"y":70,"text":"","isAcceptState":false},{"x":506,"y":70,"text":"","isAcceptState":false},{"x":62,"y":181,"text":"","isAcceptState":false},{"x":62,"y":294,"text":"","isAcceptState":false},{"x":62,"y":405,"text":"","isAcceptState":false},{"x":62,"y":523,"text":"","isAcceptState":false},{"x":172,"y":523,"text":"","isAcceptState":false},{"x":286,"y":523,"text":"","isAcceptState":false},{"x":399,"y":523,"text":"","isAcceptState":false},{"x":506,"y":523,"text":"","isAcceptState":true},{"x":172,"y":405,"text":"","isAcceptState":false},{"x":172,"y":294,"text":"","isAcceptState":false},{"x":172,"y":181,"text":"","isAcceptState":true},{"x":286,"y":181,"text":"","isAcceptState":false},{"x":286,"y":294,"text":"","isAcceptState":true},{"x":286,"y":405,"text":"","isAcceptState":false},{"x":399,"y":405,"text":"","isAcceptState":true},{"x":399,"y":294,"text":"","isAcceptState":false},{"x":399,"y":181,"text":"","isAcceptState":false},{"x":506,"y":181,"text":"","isAcceptState":false},{"x":506,"y":294,"text":"","isAcceptState":false},{"x":506,"y":405,"text":"","isAcceptState":false},{"x":619,"y":70,"text":"","isAcceptState":false},{"x":736,"y":70,"text":"","isAcceptState":false},{"x":619,"y":181,"text":"","isAcceptState":false},{"x":619,"y":294,"text":"","isAcceptState":false},{"x":619,"y":405,"text":"","isAcceptState":true},{"x":619,"y":523,"text":"","isAcceptState":false},{"x":736,"y":523,"text":"","isAcceptState":false},{"x":736,"y":405,"text":"","isAcceptState":false},{"x":736,"y":294,"text":"","isAcceptState":true},{"x":736,"y":181,"text":"","isAcceptState":false}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-39,"deltaY":-49},{"type":"Link","nodeA":0,"nodeB":1,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":1,"nodeB":2,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":4,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":0,"nodeB":5,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":5,"nodeB":6,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":7,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":8,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":8,"nodeB":9,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":9,"nodeB":10,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":10,"nodeB":11,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":11,"nodeB":12,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":5,"nodeB":15,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":15,"nodeB":16,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":16,"nodeB":21,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":21,"nodeB":22,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":22,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":21,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":16,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":1,"nodeB":15,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":15,"nodeB":14,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":16,"nodeB":17,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":21,"nodeB":20,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":22,"nodeB":23,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":14,"nodeB":13,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":17,"nodeB":18,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":20,"nodeB":19,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":23,"nodeB":24,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":13,"nodeB":9,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":18,"nodeB":10,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":19,"nodeB":11,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":24,"nodeB":12,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":13,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":13,"nodeB":18,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":18,"nodeB":19,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":19,"nodeB":24,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":20,"nodeB":23,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":17,"nodeB":20,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":14,"nodeB":17,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":14,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":25,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":25,"nodeB":26,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":22,"nodeB":27,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":25,"nodeB":27,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":26,"nodeB":34,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":34,"nodeB":33,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":33,"nodeB":32,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":32,"nodeB":31,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":12,"nodeB":30,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":30,"nodeB":31,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":29,"nodeB":30,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":29,"nodeB":32,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":24,"nodeB":29,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":23,"nodeB":28,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":28,"nodeB":33,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":27,"nodeB":34,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":27,"nodeB":28,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":28,"nodeB":29,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0}]}
        "#,
    ).unwrap();

    let on_terminate = {
        shadow_clone!(word);
        move |_outcome| {
            let mut seed = [0; 32];
            for v in seed.iter_mut() {
                *v = (randfloat() * 256.0) as u8;
            }
            let mut rng = ChaCha8Rng::from_seed(seed);

            let mut new_word = String::new();
            for _ in 0..(rng.gen_range(5..20)) {
                new_word.push(if rng.gen_ratio(1, 2) { '0' } else { '1' })
            }
            word.set(new_word);
        }
    };

    html!(
        <CanvasPlayer word={(&*word).clone()}
        fsm={fsm.clone()} editable={false} speed_changeable={false}
        auto_restart={true} show_status_indicator={false} show_transport_buttons={true}
        pause_on_restart={false}
        speed={980}
        {on_terminate}/>
    )
}

#[function_component]
fn LinkShowcaseFSMPlayer() -> Html {
    let word = use_state(|| String::from("1101012"));
    let fsm: StateMachine = serde_json::from_str(
        r#"
        {"nodes":[{"x":146,"y":274,"text":"start","isAcceptState":false},{"x":365,"y":274,"text":"self","isAcceptState":false},{"x":508,"y":274,"text":"","isAcceptState":false},{"x":692,"y":140,"text":"","isAcceptState":false},{"x":223,"y":473,"text":"node","isAcceptState":false},{"x":462,"y":473,"text":"OK","isAcceptState":true}],"links":[{"type":"StartLink","node":0,"text":"link","deltaX":0,"deltaY":-80},{"type":"SelfLink","node":1,"text":"link","anchorAngle":-1.6804464989638066},{"type":"Link","nodeA":2,"nodeB":3,"text":"normal link","lineAngleAdjust":3.141592653589793,"parallelPart":0.30911757893924185,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":2,"text":"","lineAngleAdjust":3.141592653589793,"parallelPart":0.5570910213850073,"perpendicularPart":-85.79116946832669}]}
"#,
    )
    .unwrap();

    html!(
        <CanvasPlayer word={(&*word).clone()}
        fsm={fsm.clone()} editable={true} speed_changeable={false}
        auto_restart={false} show_status_indicator={false} show_transport_buttons={false}
        show_word_indicator={false}
        speed={0}/>
    )
}

#[function_component]
fn ZeroZeroOrZeroOneFSM() -> Html {
    let word = use_state(|| String::from("01"));
    let next_state = use_state(|| true);
    let fsm: StateMachine = serde_json::from_str(
        r#"
        {"nodes":[{"x":158,"y":276,"text":"","isAcceptState":false},{"x":369,"y":276,"text":"A","isAcceptState":false},{"x":515,"y":123,"text":"","isAcceptState":true},{"x":515,"y":429,"text":"","isAcceptState":false}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-93,"deltaY":0},{"type":"Link","nodeA":0,"nodeB":1,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":1,"nodeB":2,"text":"0","lineAngleAdjust":3.141592653589793,"parallelPart":0.6893683622135272,"perpendicularPart":0.0},{"type":"Link","nodeA":1,"nodeB":3,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0}]}
        "#,
    )
    .unwrap();

    let on_terminate = {
        shadow_clone!(word, next_state);
        move |_outcome| {
            let state = *next_state;

            let mut new_word = String::new();
            new_word.push('0');
            new_word.push(if state { '0' } else { '1' });
            word.set(new_word);
            next_state.set(!state);
        }
    };

    html!(
        <CanvasPlayer word={(&*word).clone()}
        fsm={fsm.clone()} editable={false} speed_changeable={true}
        auto_restart={true} show_status_indicator={true} show_transport_buttons={true}
        pause_on_restart={false}
        speed={860}
        {on_terminate}/>
    )
}

#[function_component]
fn AsThenBsFSM() -> Html {
    let word = use_state(|| String::from("AAABBB"));
    let is_running = use_state(|| true);
    let fsm: StateMachine = serde_json::from_str(
        r#"
        {"nodes":[{"x":226,"y":298,"text":"","isAcceptState":false},{"x":481,"y":298,"text":"","isAcceptState":true}],"links":[{"type":"SelfLink","node":0,"text":"A","anchorAngle":-1.5707963267948966},{"type":"StartLink","node":0,"text":"","deltaX":-123,"deltaY":0},{"type":"Link","nodeA":0,"nodeB":1,"text":"B","lineAngleAdjust":3.141592653589793,"parallelPart":0.5567765567765568,"perpendicularPart":-37.0},{"type":"SelfLink","node":1,"text":"B","anchorAngle":-1.5707963267948966}]}        "#,
    )
    .unwrap();

    let on_terminate = {
        shadow_clone!(is_running);
        move |_outcome| {
            is_running.set(false);
        }
    };

    let oninput = {
        shadow_clone!(word);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            word.set(target.value());
        }
    };

    let play_pulse = use_state(|| 0usize);
    let onclick = {
        shadow_clone!(is_running, play_pulse);
        move |ev: MouseEvent| {
            ev.prevent_default();
            is_running.set(true);
            play_pulse.set(*play_pulse + 1);
        }
    };

    html!(
        <>
            <CanvasPlayer word={(&*word).clone()}
            fsm={fsm.clone()} editable={false} speed_changeable={true}
            auto_restart={true} show_status_indicator={true} show_transport_buttons={true}
            pause_on_restart={true} play_on_change={*is_running}
            speed={860} play_pulse={*play_pulse}
            {on_terminate}/>
            <form class="input-group my-2" style="width: 800px; margin: 0 auto;">
                <span class="input-group-text">{"Введите слово для проверки: "}</span>
                <input class="form-control" disabled={*is_running} value={(&*word).clone()} {oninput} />
                <input class="btn btn-success" disabled={*is_running} value="Тест!" type="submit" {onclick} />
            </form>
        </>
    )
}

#[function_component]
fn AsBsFourFSM() -> Html {
    let word = use_state(|| String::from("AAABBB"));
    let is_running = use_state(|| true);
    let fsm: StateMachine = serde_json::from_str(
        r#"
{"nodes":[{"x":161,"y":470,"text":"","isAcceptState":false},{"x":629,"y":470,"text":"","isAcceptState":true},{"x":197,"y":369,"text":"","isAcceptState":false},{"x":240,"y":274,"text":"","isAcceptState":false},{"x":576,"y":369,"text":"","isAcceptState":false},{"x":515,"y":274,"text":"","isAcceptState":false},{"x":473,"y":193,"text":"","isAcceptState":false},{"x":296,"y":193,"text":"","isAcceptState":false},{"x":329,"y":117,"text":"","isAcceptState":false},{"x":426,"y":117,"text":"","isAcceptState":false},{"x":60,"y":470,"text":"","isAcceptState":false}],"links":[{"type":"Link","nodeA":0,"nodeB":2,"text":"A","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"A","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":7,"text":"A","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":8,"text":"A","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":9,"nodeB":6,"text":"B","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":5,"text":"B","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":5,"nodeB":4,"text":"B","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":1,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":0,"nodeB":4,"text":"B","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":5,"text":"B","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":6,"text":"B","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":9,"text":"B","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"StartLink","node":10,"text":"","deltaX":0,"deltaY":-110},{"type":"Link","nodeA":10,"nodeB":0,"text":"A","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0}]}
        "#,
    )
    .unwrap();

    let on_terminate = {
        shadow_clone!(is_running);
        move |_outcome| {
            is_running.set(false);
        }
    };

    let oninput = {
        shadow_clone!(word);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            word.set(target.value());
        }
    };

    let play_pulse = use_state(|| 0usize);
    let onclick = {
        shadow_clone!(is_running, play_pulse);
        move |ev: MouseEvent| {
            ev.prevent_default();
            is_running.set(true);
            play_pulse.set(*play_pulse + 1);
        }
    };

    html!(
        <>
            <CanvasPlayer word={(&*word).clone()}
            fsm={fsm.clone()} editable={false} speed_changeable={true}
            auto_restart={true} show_status_indicator={true} show_transport_buttons={true}
            pause_on_restart={true} play_on_change={*is_running}
            speed={860} play_pulse={*play_pulse}
            {on_terminate}/>
            <form class="input-group my-2" style="width: 800px; margin: 0 auto;">
                <span class="input-group-text">{"Введите слово для проверки: "}</span>
                <input class="form-control" disabled={*is_running} value={(&*word).clone()} {oninput} />
                <input class="btn btn-success" disabled={*is_running} value="Тест!" type="submit" {onclick} />
            </form>
        </>
    )
}

#[function_component]
fn EmailValidatorFSM() -> Html {
    let word = use_state(|| String::from("dabadab@feefeb.space"));
    let is_running = use_state(|| true);
    let fsm: StateMachine = serde_json::from_str(
        r#"
{"nodes":[{"x":74,"y":91,"text":"","isAcceptState":false},{"x":208,"y":91,"text":"user","isAcceptState":false},{"x":416,"y":91,"text":"","isAcceptState":false},{"x":539,"y":91,"text":"site","isAcceptState":false},{"x":135,"y":248,"text":"","isAcceptState":false},{"x":539,"y":248,"text":"dot","isAcceptState":false},{"x":135,"y":407,"text":"","isAcceptState":false},{"x":244,"y":314,"text":"","isAcceptState":true},{"x":305,"y":407,"text":"","isAcceptState":false},{"x":230,"y":491,"text":"","isAcceptState":true},{"x":87,"y":524,"text":"","isAcceptState":true},{"x":446,"y":331,"text":"","isAcceptState":true},{"x":435,"y":491,"text":"","isAcceptState":true}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-59,"deltaY":0},{"type":"Link","nodeA":0,"nodeB":1,"text":"a","lineAngleAdjust":3.141592653589793,"parallelPart":0.7647058823529411,"perpendicularPart":-56.0},{"type":"Link","nodeA":0,"nodeB":1,"text":"b","lineAngleAdjust":3.141592653589793,"parallelPart":0.5588235294117647,"perpendicularPart":-37.0},{"type":"Link","nodeA":0,"nodeB":1,"text":"c","lineAngleAdjust":3.141592653589793,"parallelPart":0.5588235294117647,"perpendicularPart":-14.0},{"type":"Link","nodeA":0,"nodeB":1,"text":"d","lineAngleAdjust":0.0,"parallelPart":0.5147058823529411,"perpendicularPart":19.0},{"type":"Link","nodeA":0,"nodeB":1,"text":"e","lineAngleAdjust":0.0,"parallelPart":0.6764705882352942,"perpendicularPart":40.0},{"type":"Link","nodeA":0,"nodeB":1,"text":"f","lineAngleAdjust":0.0,"parallelPart":0.6617647058823529,"perpendicularPart":67.0},{"type":"SelfLink","node":1,"text":"a","anchorAngle":1.740163983875466},{"type":"SelfLink","node":1,"text":"b","anchorAngle":1.3352513460740334},{"type":"SelfLink","node":1,"text":"c","anchorAngle":1.0360703319417248},{"type":"SelfLink","node":1,"text":"d","anchorAngle":-1.7539071440573808},{"type":"SelfLink","node":1,"text":"e","anchorAngle":-1.1839206090638683},{"type":"SelfLink","node":1,"text":"f","anchorAngle":-0.65788860518221},{"type":"Link","nodeA":1,"nodeB":2,"text":"@","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"a","lineAngleAdjust":3.141592653589793,"parallelPart":0.6754385964912281,"perpendicularPart":-58.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"b","lineAngleAdjust":3.141592653589793,"parallelPart":0.5877192982456141,"perpendicularPart":-34.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"c","lineAngleAdjust":3.141592653589793,"parallelPart":0.49122807017543857,"perpendicularPart":-14.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"d","lineAngleAdjust":0.0,"parallelPart":0.7017543859649122,"perpendicularPart":18.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"e","lineAngleAdjust":0.0,"parallelPart":0.8421052631578947,"perpendicularPart":35.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"f","lineAngleAdjust":0.0,"parallelPart":0.9385964912280702,"perpendicularPart":54.0},{"type":"Link","nodeA":3,"nodeB":5,"text":".","lineAngleAdjust":3.141592653589793,"parallelPart":0.7261146496815286,"perpendicularPart":-128.0},{"type":"Link","nodeA":5,"nodeB":4,"text":"","lineAngleAdjust":0.0,"parallelPart":0.836603886696308,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":6,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"SelfLink","node":3,"text":"b","anchorAngle":-1.2419654938113762},{"type":"SelfLink","node":3,"text":"a","anchorAngle":-1.797069688822532},{"type":"SelfLink","node":3,"text":"c","anchorAngle":-0.5763752205911837},{"type":"SelfLink","node":3,"text":"d","anchorAngle":0.0},{"type":"SelfLink","node":3,"text":"e","anchorAngle":0.6078019961139605},{"type":"SelfLink","node":3,"text":"f","anchorAngle":0.9025069079643124},{"type":"Link","nodeA":6,"nodeB":7,"text":"ru","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":8,"text":"s","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":9,"text":"com","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":10,"text":"net","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":8,"nodeB":11,"text":"u","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":8,"nodeB":12,"text":"pace","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0}]}
        "#,
    )
    .unwrap();

    let on_terminate = {
        shadow_clone!(is_running);
        move |_outcome| {
            is_running.set(false);
        }
    };

    let oninput = {
        shadow_clone!(word);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            word.set(target.value());
        }
    };

    let play_pulse = use_state(|| 0usize);
    let onclick = {
        shadow_clone!(is_running, play_pulse);
        move |ev: MouseEvent| {
            ev.prevent_default();
            is_running.set(true);
            play_pulse.set(*play_pulse + 1);
        }
    };

    html!(
        <>
            <CanvasPlayer word={(&*word).clone()}
            fsm={fsm.clone()} editable={false} speed_changeable={true}
            auto_restart={true} show_status_indicator={true} show_transport_buttons={true}
            pause_on_restart={true} play_on_change={*is_running}
            speed={860} play_pulse={*play_pulse}
            {on_terminate}/>
            <form class="input-group my-2" style="width: 800px; margin: 0 auto;">
                <span class="input-group-text">{"Введите слово для проверки: "}</span>
                <input class="form-control" disabled={*is_running} value={(&*word).clone()} {oninput} />
                <input class="btn btn-success" disabled={*is_running} value="Тест!" type="submit" {onclick} />
            </form>
        </>
    )
}

#[function_component]
fn DetermMazeFSM() -> Html {
    let word = use_state(|| String::from("0100010"));
    let fsm: StateMachine = serde_json::from_str(
        r#"
        {"nodes":[{"x":135,"y":133,"text":"A","isAcceptState":false},{"x":208,"y":271,"text":"B","isAcceptState":false},{"x":366,"y":152,"text":"C","isAcceptState":true},{"x":514,"y":271,"text":"D","isAcceptState":false},{"x":387,"y":394,"text":"","isAcceptState":true}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-86,"deltaY":-91},{"type":"Link","nodeA":0,"nodeB":1,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.4367127559184343,"perpendicularPart":70.00449575565976},{"type":"Link","nodeA":1,"nodeB":2,"text":"1","lineAngleAdjust":3.141592653589793,"parallelPart":0.3789904153354632,"perpendicularPart":-81.24856996529694},{"type":"Link","nodeA":2,"nodeB":3,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":4,"text":"0010","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0}]}
        "#,
    )
    .unwrap();

    html!(
        <CanvasPlayer word={(&*word).clone()}
        fsm={fsm.clone()} editable={false} speed_changeable={true}
        auto_restart={true} show_status_indicator={true} show_transport_buttons={true}
        pause_on_restart={false} show_steps_indicator={true}
        speed={800}/>
    )
}

#[function_component]
fn NonDetermDemoFSM() -> Html {
    let word = use_state(|| String::from("110110"));
    let fsm: StateMachine = serde_json::from_str(
        r#"
        {"nodes":[{"x":585,"y":134,"text":"A","isAcceptState":false},{"x":585,"y":260,"text":"B","isAcceptState":false},{"x":585,"y":377,"text":"C","isAcceptState":true},{"x":173,"y":134,"text":"A","isAcceptState":false},{"x":173,"y":253,"text":"A,B","isAcceptState":false},{"x":173,"y":377,"text":"A,C","isAcceptState":true}],"links":[{"type":"SelfLink","node":0,"text":"0","anchorAngle":-0.5633162614919681},{"type":"SelfLink","node":0,"text":"1","anchorAngle":0.3217505543966422},{"type":"Link","nodeA":0,"nodeB":1,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":1,"nodeB":2,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"SelfLink","node":3,"text":"0","anchorAngle":0.0},{"type":"Link","nodeA":3,"nodeB":4,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5714285714285714,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":5,"text":"0","lineAngleAdjust":3.141592653589793,"parallelPart":0.6901408450704225,"perpendicularPart":-14.0},{"type":"StartLink","node":3,"text":"","deltaX":72,"deltaY":-80},{"type":"SelfLink","node":4,"text":"1","anchorAngle":0.0},{"type":"Link","nodeA":5,"nodeB":4,"text":"1","lineAngleAdjust":3.141592653589793,"parallelPart":0.5704225352112676,"perpendicularPart":-19.0},{"type":"Link","nodeA":5,"nodeB":3,"text":"0","lineAngleAdjust":3.141592653589793,"parallelPart":0.32510288065843623,"perpendicularPart":-58.0},{"type":"StartLink","node":0,"text":"","deltaX":-96,"deltaY":-92}]}
        "#,
    ).unwrap();

    let on_terminate = {
        shadow_clone!(word);
        move |_outcome| {
            let mut seed = [0; 32];
            for v in seed.iter_mut() {
                *v = (randfloat() * 256.0) as u8;
            }
            let mut rng = ChaCha8Rng::from_seed(seed);

            let mut new_word = String::new();
            let do_accept = rng.gen_bool(0.5);
            for i in 0..(rng.gen_range(5..(if do_accept { 7 } else { 10 }))) {
                new_word.push(if rng.gen_ratio(1, 2) { '0' } else { '1' })
            }
            if do_accept {
                new_word.extend("110".chars());
            }
            word.set(new_word);
        }
    };

    html!(
        <CanvasPlayer word={(&*word).clone()}
        fsm={fsm.clone()} editable={false} speed_changeable={true}
        auto_restart={true} show_status_indicator={true} show_transport_buttons={true}
        pause_on_restart={false}
        speed={800}
        {on_terminate}/>
    )
}

#[function_component]
fn NonDetermMazeFSM() -> Html {
    let word = use_state(|| String::from("1101001"));
    let fsm: StateMachine = serde_json::from_str(
        r#"
        {"nodes":[{"x":399,"y":129,"text":"A","isAcceptState":false},{"x":178,"y":225,"text":"B","isAcceptState":false},{"x":539,"y":255,"text":"C","isAcceptState":false},{"x":592,"y":355,"text":"E","isAcceptState":false},{"x":178,"y":355,"text":"D","isAcceptState":false},{"x":178,"y":462,"text":"F","isAcceptState":true},{"x":592,"y":469,"text":"H","isAcceptState":true},{"x":385,"y":469,"text":"G","isAcceptState":false}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":0,"deltaY":-80},{"type":"Link","nodeA":0,"nodeB":1,"text":"11010","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":0,"nodeB":2,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":1,"nodeB":4,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":4,"text":"1","lineAngleAdjust":3.141592653589793,"parallelPart":0.37696424626392344,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":5,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":6,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":7,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":6,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":2,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0}]}
        "#,
    )
    .unwrap();

    html!(
        <CanvasPlayer word={(&*word).clone()}
        fsm={fsm.clone()} editable={false} speed_changeable={true}
        auto_restart={true} show_status_indicator={true} show_transport_buttons={true}
        pause_on_restart={false} show_steps_indicator={true}
        speed={800}/>
    )
}

#[autoprops_component]
fn WordDisplay(word: AttrValue) -> Html {
    if word.is_empty() {
        html!(<span class="badge text-bg-primary">{"ε"}</span>)
    } else {
        html!(<code>{word}</code>)
    }
}

#[function_component(Tutorial)]
pub fn tutorial() -> Html {
    let is_english = use_state(|| false);
    let toggle_en = {
        shadow_clone!(is_english);
        move |ev: MouseEvent| {
            ev.prevent_default();
            is_english.set(!*is_english);
        }
    };
    html!(
        <>
            <button class="btn btn-secondary" style="margin: 0 0;" onclick={toggle_en}>
                if *is_english {
                    {"Можно это также прочитать на русском языке, нажав сюда"}
                } else {
                    {"You can also read this in English, click here"}
                }
            </button>
            if *is_english {
                <TutorialEn />
            } else {
                <TutorialRu />
            }
        </>
    )
}

#[function_component(TutorialRu)]
pub fn tutorial_ru() -> Html {
    html!(
        <>
                <h1>{"Введение в конечные автоматы"}</h1>
                <div class="alert alert-info">
                    {"Есть комментарии, как улучшить это? "}<a href="https://github.com/rudn-lab/rudn-year3-fsm/issues" class="alert-link">{"Создайте issue в репозитории проекта!"}</a>
                </div>
                <hr />
                <h2>{"1. Формальные языки"}</h2>
                <p>
                    <em>{"Строка"}</em>{" — это конечная последовательность символов: "}<WordDisplay word="10010101"/>{" — строка, состоящая из символов 0 и 1."}
                </p>
                <p>
                    {"Строка может быть пустой, то есть состоять из нуля символов. Такая строка обозначается символом эпсилон: "}<WordDisplay word="" />
                    {" (в литературе иногда также используется символ лямбда λ; разницу между ними я не совсем понял, но здесь мы используем эпсилон для пустого слова.)"}
                </p>
                <p>
                    <em><a href="https://ru.wikipedia.org/wiki/%D0%A4%D0%BE%D1%80%D0%BC%D0%B0%D0%BB%D1%8C%D0%BD%D1%8B%D0%B9_%D1%8F%D0%B7%D1%8B%D0%BA"><b>{"Формальный язык"}</b></a>
                    {" — это (возможно бесконечное) множество строк. "}</em>
                    {"Строки, которые являются частью языка, называются "}<em>{"словами. "}</em>
                    {"Для каждой строки нужно иметь возможность сказать, является ли она словом; тогда язык определен."}
                </p>
                <p>
                    {"Формальный язык задается определенным правилом. "}
                    {"Самое простой вид правила — перечисление всех слов языка. "}
                    {"Самый сложный — компьютерная программа, которая для каждой строки говорит, является ли она словом языка."}
                </p>
                <p>
                    {"Согласно "}
                    <a href="https://ru.wikipedia.org/wiki/%D0%A2%D0%B5%D0%BE%D1%80%D0%B8%D1%8F_%D0%B0%D0%B2%D1%82%D0%BE%D0%BC%D0%B0%D1%82%D0%BE%D0%B2#%D0%9F%D0%BE_%D1%81%D0%BB%D0%BE%D0%B6%D0%BD%D0%BE%D1%81%D1%82%D0%B8_%D1%80%D0%B0%D0%B7%D0%B1%D0%B8%D1%80%D0%B0%D0%B5%D0%BC%D1%8B%D1%85_%D1%8F%D0%B7%D1%8B%D0%BA%D0%BE%D0%B2">
                        {"классификации формальных языков Н. Хомского,"}
                    </a>
                    {" существуют четыре категории формальных языков по сложности. "}
                    <em>{"Регулярные языки"}</em>{" — самые простые из них. "}
                    {"Их могут распознавать "}<em>{"конечные автоматы. "}</em>
                    {"Когда мы говорим, что конечный автомат распознает язык, это значит, что он всегда дает правильный ответ — слово является ли частью языка или нет."}
                </p>
                <hr />


                <h2>{"2. Конечные автоматы"}</h2>
                <p>{"Конечный автомат состоит из кружочков "}<em>{"(состояний)"}</em>{" и направленных стрелочек между ними "}<em>{"(переходов)."}</em></p>
                <SampleFSMPlayer />

                <p>{"Переходы могут приходить из пустоты "}<em>{"(начальные переходы)"}</em>{" или соединять два состояния, или одно состояние само с собой."}</p>
                <p>{"Состояния могут быть обведены один или два раза. Кружочек, который обведен два раза, называется "}<em>{"принимающим состоянием."}</em></p>
                <p>{"На стрелочках есть буквы, которые требуются, чтобы перейти по ней. Если для перехода по стрелочке не требуется букв, то вместо этого там написана буква эпислон; она пропадает, если начать писать текст."}</p>
                <p>{"Попробуйте создать эти объекты в поле снизу. Учтите, что можно вводить текст только, когда мышь находится над полем, и оно имеет красную обводку."}</p>
                <LinkShowcaseFSMPlayer />

                <p>{"Когда автомат обрабатывает строку, он хранит внутри себя указатель на состояние. "}
                {"Изначально указатель указывает на те состояния, к которым есть входящие переходы — стрелочки из пустоты."}</p>
                <p>{"На каждом шаге автомат смотрит на все стрелочки из текущего состояния. Если на стрелочке написана следующая буква в строке, то по этой стрелочке осуществляется переход, а курсор передвигается."}</p>
                <p><b>{"Можно сказать, что мы ходим по лабиринту из стрелочек и кружочков. Проверяемая строка — маршрут по лабиринту. "}
                {"В каждом кружочке смотрим на доступные выходы. Если есть выход, который соответствует строке, то идем по нему. Кружочки с двойной обводкой — выходы из лабиринта. "}
                </b></p>

                <DetermMazeFSM />

                <p>{"В этом примере у нас есть простой лабиринт, который имеет только один путь. По шагам:"}</p>
                <ol>
                    <li>{"Сначала, мы входим в лабиринт и оказываемся в кружочке A."}</li>
                    <li>{"Мы смотрим на наш маршрут; он говорит идти по "}<WordDisplay word="0" />{"."}</li>
                    <li>{"Мы видим выход с надписью "}<WordDisplay word="0" />{" и идем по нему. Мы зачеркиваем "}<WordDisplay word="0" />{" с начала строки: мы уже сделали этот переход в лабиринте. Мы оказываемся в кружочке B."}</li>
                    <li>{"Наш маршрут дальше говорит нам пойти в выход с надписью "}<WordDisplay word="1" />{". Мы находим такой выход из этого кружочка;"}</li>
                    <li>{"и после этого идем по этому пути, который ведет в кружочек C. Мы зачеркиваем цифру "}<WordDisplay word="1" />{" из маршрута."}</li>
                    <li>{"Кружочек C уже является выходом из лабиринта, но маршрут еще не закончился, поэтому мы идем дальше. Маршрут говорит идти по стрелочке "}<WordDisplay word="0" />{"."}</li>
                    <li>{"Мы идем по этой стрелочке, зачеркивая "}<WordDisplay word="0" />{", и оказываемся в кружочке D."}</li>
                    <li>{"Здесь мы находим выход, на котором написано "}<WordDisplay word="0010" />{". У нас в маршруте как раз написано, что дальше надо будет идти в выход "}<WordDisplay word="0" />{", затем "}<WordDisplay word="0" />{", затем "}<WordDisplay word="1" />{", и наконец "}<WordDisplay word="0" />{"."}</li>
                    <li>{"Несколько букв подряд на выходе — это как будто пройти по всем из них одновременно, поэтому мы идем в этот выход, и зачеркиваем все четыре буквы "}<WordDisplay word="0010" />{" из маршрута."}</li>
                    <li>{"У нас теперь закончился маршрут; в нем нет незачеркнутых букв. И мы также оказались в кружочке, где есть выход из лабиринта. Если такое произошло, это значит, что маршрут был хороший — мы принимаем это слово."}</li>
                </ol>

                <p>{"Вы можете внимательно просмотреть эти шаги, поставив автомат на паузу с помощью зеленой кнопки, "}
                {"затем запустить его с начала красной кнопкой, "}
                {"и исполнять его по отдельным шагам с помощью синей кнопки."}
                </p>



                <p>{"Если из кружочка есть несколько выходов, то мы идем только по тем стрелочкам, которые соответствуют маршруту, и игнорируем остальные."}</p>
                <p>{"В поле ниже, мы сначала идем по стрелочке 0, и оказываемся в состоянии A."}
                {"Когда строка — "}<WordDisplay word="00" />{", то автомат делает переход по верхней стрелочке, а когда "}<WordDisplay word="01" />{", то по нижней. "}
                {"Только в верхнем кружочке есть выход, поэтому автомат принимает только строку "}<WordDisplay word="00" />{"."}</p>
                <ZeroZeroOrZeroOneFSM />

                <p>{"Когда мы зачеркнули все буквы в нашем маршруте, то мы должны стоять на кружочке с выходом. Если это так, то мы успешно вышли из лабиринта, значит маршрут хороший, и автомат принимает такое слово."}</p>

                <ul>
                    <li>{"Если же мы дошли до конца маршрута, но оказались в обычном кружочке, то мы заблудились; маршрут не хороший, и мы отвергаем такое слово."}</li>
                    <li>{"Если маршрут говорит пойти по стрелочке, которой не существует (например, если в маршруте написано идти в выход "}<WordDisplay word="2" />{", а в этом кружочке есть только выходы "}<WordDisplay word="0" />{" и "}<WordDisplay word="1" />{"), значит мы тоже заблудились; в такой ситуации мы также отвергаем слово."}
                    </li>
                </ul>

                <hr />
                <h2>{"3. Применения автоматов"}</h2>

                <p>{"Конечные автоматы могут быть использованы для распознания любого языка, для которого требуется конечное количество памяти."}</p>
                <p>
                {"Например, легко можно распознать язык таких слов, которые состоят из какого-то количества букв "}
                <WordDisplay word="A" />
                {", за которыми идет какое-то количество букв "}<WordDisplay word="B" />
                {" — для этого нужно лишь хранить одно значение (мы сейчас ждем букву "}<WordDisplay word="A" />{" или "}<WordDisplay word="B" />{"), "}
                {"и конечный автомат для этого языка требует всего двух состояний. "}
                {"Попробуйте ввести разные последовательности "}<em>{"латинских букв"}</em>{" и посмотрите, как автомат их обрабатывает."}
                </p>

                <AsThenBsFSM />

                <p>{"С другой стороны, нельзя построить конечный автомат, который будет принимать последовательность букв "}<WordDisplay word="A" />{", "}
                {"а затем последовательность букв "}<WordDisplay word="B" /><em>{" такой же длины. "}</em>
                {"Это потому, что теперь нужно хранить количество букв "}<WordDisplay word="A" />{", которые мы видели. "}
                {"Это количество может быть очень большим; в теории, таким большим, что в нашем компьютере не хватит места, чтобы хранить это число. "}
                {"Из-за этого такой язык невозможно распознавать конечным автоматом."}
                </p>

                <p>{"Как демонстрация этого, вот автомат, который распознает такие последовательности, но с длиной не больше 8. "}
                {"Чтобы он распознавал любые такие слова, нужно, чтобы он таким же образом был продлен до бесконечности — "}
                {"но тогда он перестанет быть "}<em>{"конечным"}</em>{" автоматом."}
                </p>

                <AsBsFourFSM />

                <p>{"По этой же причине невозможно сделать конечный автомат, который распознает правильные скобочные последовательности: там также требуется хранить количество скобок, которые нужно закрыть, что запрещено."}</p>

                <p>{"Конечные автоматы задают регулярные языки, которые также задаются "}<em>{"регулярными выражениями"}</em>
                {" — они часто полезны в программировании. Например, они используются в валидации текстовых полей; например, email-адресов."}</p>
                <p>{"К сожалению, эта среда работы с конечными автоматами не позволяет сделать стрелочку, которая принимает \"любую одну букву\". "}
                {"Поэтому мы ограничим набор допустимых букв: пусть имя пользователя и домен могут состоять из букв от "}<WordDisplay word="a" />
                {" до "}<WordDisplay word="f" />{", и домен может заканчиваться только на .com, .ru, .su, .net или .space."}</p>
                <p>{"Регулярное выражение для такого правила валидации будет выглядеть вот так: "}<code>{"[a-f]+@[a-f]+\\.(com|ru|net|(s(u|pace)))"}</code>{"; "}
                {"это полностью соответствует следующему конечному автомату. "}
                {"(Подсказка: используйте пустые эпсилон-переходы, чтобы лучше организовывать автомат на рисунке.)"}
                </p>

                <EmailValidatorFSM />

                <hr />
                <h2>{"4. Недетерминированные автоматы"}</h2>
                <p>{"Пока что мы говорили в основном про "}<em>{"детерминированные конечные автоматы"}</em>{". "}
                {"В таких автоматах всегда есть только один путь: никогда нет ситуации, когда из одного кружочка есть две стрелочки с одинаковым символом. "}
                {"(Некоторые определения также исключают существование пустых переходов, но я считаю, что это необязательное условие; "}
                {"уточните в материалах своего курса, как классифицируются эти автоматы.)"}
                </p>
                <p>{"Но иногда бывает удобнее описать язык с помощью "}<em>{"недетерминированного конечного автомата"}</em>{" — "}
                {"здесь, разрешается иметь несколько стрелочек из одного состояния, имеющих один и тот же символ. "}
                {"Автомат будет ходить по всем вариантам параллельно; в этой визуализации это показывается разными цветами. "}
                </p>

                <p><b>{"Ходя по лабиринту НКА, если мы встречаем развилку, то мы клонируем себя. Каждый клон идет по своей ветке лабиринта. "}
                {"Если хотя бы один клон в итоге дошел до выхода, то маршрут хороший и мы его принимаем; "}
                {"если все клоны заблудились, то маршрут плохой и мы его отвергаем. "}</b></p>

                <NonDetermMazeFSM />

                <ol>
                    <li>{"Сначала мы заходим в лабиринт и оказываемся в состоянии A."}</li>
                    <li>{"Здесь есть два интересных выхода: один с надписью "}<WordDisplay word="11010" />{", а другой — с надписью "}<WordDisplay word="1" />{". Наш маршрут соответствует и той, и другой стрелочке, поэтому мы клонируем себя и идем по обоим путям."}</li>
                    <li>{"Красный клон идет по стрелочке в кружочек B, зачеркивая 5 букв со своего маршрута, а оранжевый клон идет по стрелочке в кружочек C, зачеркнув всего одну букву."}</li>
                    <li><ul>
                        <li>{"У красного клона дальше в маршруте написана цифра "}<WordDisplay word="0" />{", и здесь есть только один такой выход, поэтому он идет туда;"}</li>
                        <li>{"у оранжевого клона же есть два выхода, на обоих из которых написано "}<WordDisplay word="1" />{", поэтому он клонирует себя еще раз, прежде чем продолжать."}</li>
                    </ul></li>
                    <li><ul>
                        <li>{"Теперь, красный клон идет по стрелочке от B до D;"}</li>
                        <li>{"оранжевый — по стрелочке от C до Е;"}</li>
                        <li>{"а новый желтый клон — по стрелочке от C до D."}</li>
                    </ul></li>
                    <li><ul>
                        <li>{"Сейчас, красный и желтый клон встретились в кружочке D. Кружочек D отображается желтым, потому что желтый клон более новый чем красный. Однако, два клона в разных частях своего маршрута: красный клон сейчас смотрит на цифру "}<WordDisplay word="1" />{", а желтый — на "}<WordDisplay word="0" />{"."}</li>
                        <li>{"Оранжевый клон, в это время, зашел в тупик: у него маршрут говорит идти по стрелочке с надписью "}<WordDisplay word="0" />{", но в кружочке Е единственный путь имеет надпись "}<WordDisplay word="1" />{"."}</li>
                    </ul></li>
                    <li><ul>
                        <li>{"Красный клон пошел по стрелочке от D до F;"}</li>
                        <li>{"оранжевый клон, который стоял в кружочке Е, заблудился и умер;"}</li>
                        <li>{"а бывший желтый клон превратился в оранжевого, потому что он теперь второй по порядку, а не третий. Он пошел по стрелочке от D до G."}</li>
                    </ul></li>
                    <li><ul>
                        <li>{"Красный клон дошел до кружочка F, а также до конца своего маршрута. Кружочек F содержит выход, поэтому этот клон успешно дошел до выхода."}</li>
                        <li>{"Из-за этого, нам уже не важно, что произойдет дальше с оранжевым клоном. Симуляция будет продолжаться (на следующем шаге он превратится в красного, потому что красный ушел, и в итоге он заблудится и умрет в кружочке C), но мы уже знаем, что кто-то смог дойти до выхода, и значит маршрут хороший и принимается."}</li>
                    </ul></li>

                </ol>



                <p>{"НКА часто позволяют описать один и тот же язык легче, чем ДКА."}</p>
                <p>{"Например, ниже приведены два автомата, которые распознают один и тот же язык: "}
                {"они принимают слова, которые начинаются с символов 0 и 1, "}
                {"но должны заканчиваться на "}<WordDisplay word="10" />{". "}
                {"Автомат справа прямо описывает эту идею: в состоянии A мы принимаем и 0, и 1, но если мы приняли 1, то мы параллельно начинаем проверять ветку из B и C. "}
                {"Автомат слева — ДКА, который принимает тот же язык; он заметно сложнее, в частности потому что состояние A разделилось на все три кружочка. "}
                </p>

                <NonDetermDemoFSM />

                <p>{"НКА более удобные для создания, но они не сильнее ДКА: обе категории автоматов могут обрабатывать все регулярные языки. "}
                {"Во-первых, любой ДКА это тривиальный НКА: у тебя есть возможность делать клонов, чтобы ходить по лабиринту параллельно, но это просто не потребовалось. "}
                {"Помимо этого, любой НКА можно преобразовать в ДКА, используя определенную математическую конструкцию: "}<a href="https://en.wikipedia.org/wiki/Powerset_construction">{"Powerset construction."}</a>
                {" Однако, если взять НКА с N состояниями, такое преобразование может повысить количество состояний до 2^N, что может быть непозволительно большим."}
                </p>
                <p>{"Тестирующая система одинаково обрабатывает НКА и ДКА, поэтому следует использовать тот подход, который более удобен для каждой задачи."}</p>

        </>
    )
}

#[function_component(TutorialEn)]
pub fn tutorial_en() -> Html {
    html!(
        <>
                <h1>{"Intro to finite state machines"}</h1>
                <div class="alert alert-info">
                    {"Have ideas for improving this? "}<a href="https://github.com/rudn-lab/rudn-year3-fsm/issues" class="alert-link">{"Create an issue in the project's repo!"}</a>
                </div>
                <hr />
                <h2>{"1. Formal languages"}</h2>
                <p>
                    {"A "}<em>{"string"}</em>{" is a finite sequence of symbols: "}<WordDisplay word="10010101"/>{" is a string consisting of the symbols 0 and 1."}
                </p>
                <p>
                    {"A string may be empty, containing zero symbols. This string is designated with the epsilon symbol: "}<WordDisplay word="" />
                    {" (in the literature, a lambda symbol λ is also used for this purpose; I am not entirely clear on the difference between them, but here we shall use the epsilon to designate the empty word.)"}
                </p>
                <p>
                {"A "}<em><a href="https://en.wikipedia.org/wiki/Formal_language"><b>{"formal language"}</b></a>
                    {" is a (potentially infinite) set of strings. "}</em>
                    {"The strings that are part of a language are its "}<em>{"words. "}</em>
                    {"For every string, there needs to be a way to tell if it is a word of the language; then the language is considered fully defined."}
                </p>
                <p>
                    {"A formal language is defined by some kind of rule. "}
                    {"The simplest kind of rule is an enumeration of all words in the language. "}
                    {"The most complicated is a computer program that says 'yes' or 'no' for the input string."}
                </p>
                <p>
                    {"According to "}
                    <a href="https://en.wikipedia.org/wiki/Chomsky_hierarchy">
                        {"the Chomsky hierarchy of formal languages,"}
                    </a>
                    {" there are four complexity classes of formal grammars. "}
                    <em>{"Regular languages"}</em>{" are the simplest kind. "}
                    {"They are recognized by "}<em>{"finite state machines or finite automata. "}</em>
                    {"When we say that an automaton recognizes a language, it means that it always gives the correct answer about whether a word is part of a language or not."}
                </p>
                <hr />


                <h2>{"2. Finite state machines"}</h2>
                <p>{"A FSM consists of circles "}<em>{"(states)"}</em>{" and directed arrows connecting them "}<em>{"(transitions)."}</em></p>
                <SampleFSMPlayer />

                <p>{"Transitions can come from empty space "}<em>{"(initial transitions)"}</em>{" or connect two states together, or one state to itself."}</p>
                <p>{"States may have a circle inside the big circle. The doubly-stroked circles are the "}<em>{"accepting states."}</em></p>
                <p>{"The arrows have letters on them, which are the prerequisite for following the transition. If the transition does not require any letters, it is an epsilon-transition, designated by the letter epsilon next to it; it will disappear if you start typing."}</p>
                <p>{"Try creating these objects in the box below. Note that you can only enter text when the mouse is over the box, and it has a red outline."}</p>
                <LinkShowcaseFSMPlayer />

                <p>{"As a FSM processes a string, it keeps a cursor to one of its circles. "}
                {"Initially the pointer starts at the state, which has initial transitions from empty space."}</p>
                <p>{"At every step, the FSM looks at all arrows from the current state. If the arrow's text is the next letter in the string, a transition is performed, moving the cursor to the next state."}</p>
                <p><b>{"It's as if one is navigating a maze of arrows and circles, with the tested string serving as a guide. "}
                {"In every circle, look at the exits. If there is an exit corresponding to the string, we go across it. The accepting states are the exits from the maze. "}
                </b></p>

                <DetermMazeFSM />

                <p>{"In the above example we have a simple maze that has only a single path. Here is it step-by-step:"}</p>
                <ol>
                    <li>{"First, we enter the maze and are at node A."}</li>
                    <li>{"Looking at the guide, it tells us to move into "}<WordDisplay word="0" />{"."}</li>
                    <li>{"We see an arrow labeled "}<WordDisplay word="0" />{" and transition. We cross out "}<WordDisplay word="0" />{" from the start of the string: we have already done this transition in the maze. We are now in node B."}</li>
                    <li>{"The guide tells us to go into the exit labeled "}<WordDisplay word="1" />{". We find this exit from the current node;"}</li>
                    <li>{"then go along the arrow, which leads into node C. We cross out "}<WordDisplay word="1" />{" from the guide."}</li>
                    <li>{"Node C is already an accepting state, but the guide is not over yet, so we keep going. It tells us to take the "}<WordDisplay word="0" />{" path."}</li>
                    <li>{"We go into that arrow, crossing out "}<WordDisplay word="0" />{", and are now in node D."}</li>
                    <li>{"Here we find an arrow that has the label "}<WordDisplay word="0010" />{". According to our guide, we need to go into "}<WordDisplay word="0" />{", then "}<WordDisplay word="0" />{", then "}<WordDisplay word="1" />{", and finally "}<WordDisplay word="0" />{"."}</li>
                    <li>{"If the arrow has multiple letters on it, it's as if you go across all of them in sequence — so we take that arrow and cross out all the letters "}<WordDisplay word="0010" />{" из маршрута."}</li>
                    <li>{"We are now at the end of the guide; it has no more uncrossed letters. We are also in an accepting node. When this happens, it means the guide is good, and we therefore accept this word."}</li>
                </ol>

                <p>{"You can carefully view these steps by pausing the FSM above with the green button, "}
                {"then resetting it with the red button, "}
                {"and making single steps with the blue button."}
                </p>



                <p>{"If a single node has multiple exits, we only transition with the arrows that correspond to the guide, and ignore the rest."}</p>
                <p>{"In the box below, we first always go through the 0 arrow, finding ourselves in state A."}
                {"When the input string equals "}<WordDisplay word="00" />{", the FSM makes a transition through the top arrow, and when it's  "}<WordDisplay word="01" />{", we follow the bottom arrow. "}
                {"Only the top circle is an accepting state, so the automaton only accepts the word "}<WordDisplay word="00" />{"."}</p>
                <ZeroZeroOrZeroOneFSM />

                <p>{"When we will have crossed out all the letters in the guide, we need to be standing on an accepting circle. If we are, then we have successfully exited the maze, meaning the guide is good, so the automaton accepts this word."}</p>

                <ul>
                    <li>{"If we have gotten to the end of the guide, but we are in a normal circle, then we have gotten lost; the guide isn't good, and we reject this word."}</li>
                    <li>{"If the guide says to go across an arrow that doesn't exist (for example, if the guide says to go into the "}<WordDisplay word="2" />{" arrow, but the only arrows available are "}<WordDisplay word="0" />{" and "}<WordDisplay word="1" />{"), this also means we got lost; we reject the word in this case too."}
                    </li>
                </ul>

                <hr />
                <h2>{"3. Applications of FSMs"}</h2>

                <p>{"FSMs can be used to describe any language, for which only a finite amount of memory is needed."}</p>
                <p>
                {"For example, it is easy to recognize the language of words, which start with some number of "}
                <WordDisplay word="A" />
                {", and are followed by some number of "}<WordDisplay word="B" />
                {" — for this, you need to hold only one value (whether we're currently waiting for an "}<WordDisplay word="A" />{" or a "}<WordDisplay word="B" />{"), "}
                {" and the corresponding FSM needs only two states. "}
                {"Try inputting different sequences of "}<em>{"latin letters"}</em>{" and see how the FSM will process them."}
                </p>

                <AsThenBsFSM />

                <p>{"On the other hand, it is impossible to build a finite automaton that recognizes a sequence of "}<WordDisplay word="A" />{"s, "}
                {"followed by "}<em>{" the same number of "}</em><WordDisplay word="B" />{"s."}
                {"This is because it requires keeping track of the number of "}<WordDisplay word="A" />{"s that we've seen. "}
                {"This number can be very large; in theory, so large that our computer will not have enough memory to store it. "}
                {"Because of this, it is impossible to recognize this language with a finite state machine."}
                </p>

                <p>{"As a demo, here is a finite state machine that does recognize such sequences, but up to a length of 8. "}
                {"To recognize any such sequence, you'd need to extend it to infinity — "}
                {"which means it won't be a  "}<em>{"finite"}</em>{" state machine anymore."}
                </p>

                <AsBsFourFSM />

                <p>{"For the same reason, one cannot create a FSM that recognizes proper bracket sequences: this also requires storing the number of brackets that are yet to be closed, and this is not allowed."}</p>

                <p>{"FSMs define regular languages, which are also recognized by "}<em>{"regular expressions (regexes)"}</em>
                {" — these are very common in programming. For example, they're used for text field validation, such as for email addresses."}</p>
                <p>{"Unfortunately, this FSM designer cannot make an arrow that accepts \"any single letter\". "}
                {"So, we will need to limit the acceptable letters: let's imagine that the username and domain name can only contain letters between "}<WordDisplay word="a" />
                {" and "}<WordDisplay word="f" />{", and the domain can only end with .com, .ru, .su, .net or .space."}</p>
                <p>{"The regular expression for this rule will look like this: "}<code>{"[a-f]+@[a-f]+\\.(com|ru|net|(s(u|pace)))"}</code>{"; "}
                {"this precisely matches the FSM below. "}
                {"(Hint: you can use the empty epsilon-transitions to better fit your FSM in the box.)"}
                </p>

                <EmailValidatorFSM />

                <hr />
                <h2>{"4. Non-deterministic state machines"}</h2>
                <p>{"We have so far mainly spoken about "}<em>{"determenistic automata"}</em>{". "}
                {"These only ever have a single way to go: there is never a situation where there are two arrows with the same symbol on them. "}
                {"(Some definitions also demand an absence of empty epsilon-transitions, which I don't think is a necessary condition; "}
                {"check your course materials for the precise classification.)"}
                </p>
                <p>{"But sometimes it is more convenient to describe a language in terms of "}<em>{"non-deterministic automata"}</em>{" — "}
                {"here, it is allowed to have multiple arrows from the same circle with the same symbol on them. "}
                {"The automaton will process all of the options in parallel; this visualization uses color to indicate this. "}
                </p>

                <p><b>{"As you walk the NFSM maze, when meeting a fork in the road like this, we will clone ourselves. Each clone continues along its own branch of the maze. "}
                {"If at least one clone reaches the exit of the maze, the guide is good and we accept it; "}
                {"if all the clones get lost, the guide is bad and we reject it. "}</b></p>

                <ol>
                    <li>{"First we enter the maze and are in state A."}</li>
                    <li>{"Two arrows interest us: one is labeled "}<WordDisplay word="11010" />{", and the other is labeled "}<WordDisplay word="1" />{". Our guide matches both arrows,  so we clone ourselves and go both ways."}</li>
                    <li>{"The red clone goes through the arrow to node B, crossing out 5 letters from its guide, while the orange clone goes into the arrow leading to node C, crossing out only one letter."}</li>
                    <li><ul>
                        <li>{"The red clone's guide says next to go into "}<WordDisplay word="0" />{", and this circle has only one such arrow so it goes there;"}</li>
                        <li>{"meanwhile, the orange clone has two exits that are both labeled "}<WordDisplay word="1" />{", so it clones itself again before continuing."}</li>
                    </ul></li>
                    <li><ul>
                        <li>{"Now the red clone goes through the arrow from B to D;"}</li>
                        <li>{"the orange one follows the arrow from C to Е;"}</li>
                        <li>{"and the new yellow clone takes the arrow from C to D."}</li>
                    </ul></li>
                    <li><ul>
                        <li>{"Now, the red and yellow clones have met in node D. Node D is shown in yellow, because the yellow clone is newer than the red. But the two clones are at different points in the guide: the red clone is looking at the number "}<WordDisplay word="1" />{", and the yellow one is looking at "}<WordDisplay word="0" />{"."}</li>
                        <li>{"The orange clone, meanwhile, has gotten lost: its guide says to go on the arrow marked "}<WordDisplay word="0" />{", but node E has only a single exit marked "}<WordDisplay word="1" />{"."}</li>
                    </ul></li>
                    <li><ul>
                        <li>{"The red clone goes on the arrow from D to F;"}</li>
                        <li>{"the orange clone, formerly in node E, has now gotten lost and died;"}</li>
                        <li>{"and the former yellow clone is now orange, because it's now number 3 instead of 2. It went on the arrow from D to G."}</li>
                    </ul></li>
                    <li><ul>
                        <li>{"The red clone reached node F, and also the end of its guide. Node F contains an exit from the maze, so this clone has successfully exited the maze."}</li>
                        <li>{"Because of this, we no longer care what happens with the orange clone. The simulation will continue running (on the next step, it will turn red because the red clone is gone, and eventually it gets lost and dies in node C), but we already know that since one of the clones has reached the exit, this means the guide is good and this Word is Accepted."}</li>
                    </ul></li>

                </ol>

                <NonDetermMazeFSM />


                <p>{"NFAs are often easier to describe a language with than DFAs."}</p>
                <p>{"For example, the two automata below recognize the same language: "}
                {"they accept the words that start with 0s and 1s, "}
                {"but must end in "}<WordDisplay word="10" />{". "}
                {"The FSM on the right directly represents this idea: in state A, we are looking for both 0s and 1s, but if it was a 1, then we will also check the B-C branch in parallel. "}
                {"The FSM on the left is a deterministic FSM that accepts the same language; it is notably more complicated, in part because the A state is now spread over all three nodes. "}
                </p>

                <NonDetermDemoFSM />

                <p>{"NFAs are easier to build, but they are no stronger than DFAs: both kinds of state machines can process all regular languages, and no more. "}
                {"This is because, first, every DFA is a trivial NFA: given the ability to create clones to navigate the maze in parallel, you would just never do so. "}
                {"Apart from this, every NFA can be transformed into a DFA, using a particular algorithm known as the "}<a href="https://en.wikipedia.org/wiki/Powerset_construction">{"Powerset construction"}</a>
                {". However, an NFA with N circles can, in the worst case, be expanded into a DFA with 2^N circles, which may be intractably large."}
                </p>
                <p>{"The testing system processes both NFAs and DFAs the same way, so you should use the approach that is easiest for every task."}</p>

        </>
    )
}
