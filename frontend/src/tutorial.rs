use fsm::fsm::StateMachine;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use shadow_clone::shadow_clone;
use yew::prelude::*;
use yew_autoprops::autoprops_component;
use yew_bootstrap::component::{Container, Lead};

use crate::{canvas::Canvas, canvas_player::CanvasPlayer, task::randfloat};

// {"nodes":[{"x":131,"y":98,"text":"","isAcceptState":false},{"x":189,"y":172,"text":"","isAcceptState":false},{"x":374,"y":98,"text":"","isAcceptState":false},{"x":627,"y":98,"text":"","isAcceptState":false},{"x":690,"y":172,"text":"","isAcceptState":false},{"x":437,"y":172,"text":"","isAcceptState":false},{"x":131,"y":293,"text":"","isAcceptState":false},{"x":131,"y":458,"text":"","isAcceptState":false},{"x":189,"y":528,"text":"","isAcceptState":false},{"x":189,"y":357,"text":"","isAcceptState":false},{"x":374,"y":293,"text":"","isAcceptState":false},{"x":627,"y":293,"text":"","isAcceptState":false},{"x":437,"y":357,"text":"","isAcceptState":false},{"x":690,"y":357,"text":"","isAcceptState":false},{"x":374,"y":458,"text":"","isAcceptState":false},{"x":437,"y":528,"text":"","isAcceptState":false},{"x":627,"y":458,"text":"","isAcceptState":false},{"x":690,"y":528,"text":"","isAcceptState":false}],"links":[{"type":"Link","nodeA":0,"nodeB":2,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":0,"nodeB":1,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5470588235294116,"perpendicularPart":36.50237703782816},{"type":"Link","nodeA":1,"nodeB":0,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.595022624434389,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":5,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.40158814187400743,"perpendicularPart":26.660385928787488},{"type":"Link","nodeA":5,"nodeB":2,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":4,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.46077289571201696,"perpendicularPart":27.61731988918009},{"type":"Link","nodeA":4,"nodeB":3,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":9,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5160857908847185,"perpendicularPart":28.250127523496843},{"type":"Link","nodeA":9,"nodeB":6,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5576407506702413,"perpendicularPart":0.0},{"type":"Link","nodeA":10,"nodeB":12,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.47451952882827025,"perpendicularPart":23.706829462714104},{"type":"Link","nodeA":12,"nodeB":10,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":11,"nodeB":13,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.611764705882353,"perpendicularPart":22.343837155321587},{"type":"Link","nodeA":13,"nodeB":11,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":8,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.531703775411423,"perpendicularPart":23.16664867524449},{"type":"Link","nodeA":8,"nodeB":7,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":14,"nodeB":15,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.6748224151539068,"perpendicularPart":23.562424436035172},{"type":"Link","nodeA":15,"nodeB":14,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":16,"nodeB":17,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5895816890292028,"perpendicularPart":32.481954191001165},{"type":"Link","nodeA":17,"nodeB":16,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":10,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":10,"nodeB":11,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":14,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":14,"nodeB":16,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":17,"nodeB":15,"text":"2","lineAngleAdjust":3.141592653589793,"parallelPart":0.5612648221343873,"perpendicularPart":-29.0},{"type":"Link","nodeA":15,"nodeB":8,"text":"2","lineAngleAdjust":3.141592653589793,"parallelPart":0.2903225806451613,"perpendicularPart":-25.0},{"type":"Link","nodeA":13,"nodeB":12,"text":"2","lineAngleAdjust":3.141592653589793,"parallelPart":0.5019762845849802,"perpendicularPart":-29.0},{"type":"Link","nodeA":12,"nodeB":9,"text":"2","lineAngleAdjust":3.141592653589793,"parallelPart":0.5040322580645161,"perpendicularPart":-31.0},{"type":"Link","nodeA":4,"nodeB":5,"text":"2","lineAngleAdjust":3.141592653589793,"parallelPart":0.6837944664031621,"perpendicularPart":-23.0},{"type":"Link","nodeA":5,"nodeB":1,"text":"2","lineAngleAdjust":3.141592653589793,"parallelPart":0.5040322580645161,"perpendicularPart":-23.0},{"type":"Link","nodeA":0,"nodeB":6,"text":"3","lineAngleAdjust":0.0,"parallelPart":0.7487179487179487,"perpendicularPart":28.0},{"type":"Link","nodeA":10,"nodeB":14,"text":"3","lineAngleAdjust":0.0,"parallelPart":0.6424242424242425,"perpendicularPart":21.0},{"type":"Link","nodeA":16,"nodeB":3,"text":"3","lineAngleAdjust":0.0,"parallelPart":0.14444444444444443,"perpendicularPart":86.0}]}

fn get_sample_fsm() -> StateMachine {
    serde_json::from_str(r#"{"nodes":[{"x":145,"y":207,"text":"","isAcceptState":true},{"x":145,"y":448,"text":"","isAcceptState":false},{"x":512,"y":448,"text":"","isAcceptState":true},{"x":512,"y":207,"text":"","isAcceptState":false}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-76,"deltaY":-105},{"type":"Link","nodeA":0,"nodeB":3,"text":"1","lineAngleAdjust":3.141592653589793,"parallelPart":0.5258855585831063,"perpendicularPart":-36.0},{"type":"Link","nodeA":3,"nodeB":0,"text":"1","lineAngleAdjust":3.141592653589793,"parallelPart":0.5858310626702997,"perpendicularPart":-29.0},{"type":"Link","nodeA":1,"nodeB":2,"text":"1","lineAngleAdjust":3.141592653589793,"parallelPart":0.7329700272479565,"perpendicularPart":-27.0},{"type":"Link","nodeA":2,"nodeB":1,"text":"1","lineAngleAdjust":3.141592653589793,"parallelPart":0.6158038147138964,"perpendicularPart":-42.0},{"type":"Link","nodeA":0,"nodeB":1,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.6763485477178424,"perpendicularPart":31.0},{"type":"Link","nodeA":1,"nodeB":0,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5062240663900415,"perpendicularPart":31.0},{"type":"Link","nodeA":3,"nodeB":2,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.7427385892116183,"perpendicularPart":25.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.6929460580912863,"perpendicularPart":23.0}]}"#).unwrap()
}

#[function_component]
fn SampleFSMPlayer() -> Html {
    let word = use_state(|| String::from("1101012"));
    let fsm: StateMachine = serde_json::from_str(
        r#"{"nodes":[{"x":62,"y":70,"text":"","isAcceptState":false},{"x":172,"y":70,"text":"","isAcceptState":false},{"x":286,"y":70,"text":"","isAcceptState":false},{"x":399,"y":70,"text":"","isAcceptState":false},{"x":506,"y":70,"text":"","isAcceptState":false},{"x":727,"y":70,"text":"","isAcceptState":false},{"x":62,"y":181,"text":"","isAcceptState":false},{"x":62,"y":294,"text":"","isAcceptState":false},{"x":62,"y":405,"text":"","isAcceptState":false},{"x":62,"y":523,"text":"","isAcceptState":false},{"x":172,"y":523,"text":"","isAcceptState":false},{"x":286,"y":523,"text":"","isAcceptState":false},{"x":399,"y":523,"text":"","isAcceptState":false},{"x":506,"y":523,"text":"","isAcceptState":false},{"x":727,"y":523,"text":"","isAcceptState":false},{"x":172,"y":405,"text":"","isAcceptState":false},{"x":172,"y":294,"text":"","isAcceptState":false},{"x":172,"y":181,"text":"","isAcceptState":false},{"x":286,"y":181,"text":"","isAcceptState":false},{"x":286,"y":294,"text":"","isAcceptState":false},{"x":286,"y":405,"text":"","isAcceptState":false},{"x":399,"y":405,"text":"","isAcceptState":false},{"x":399,"y":294,"text":"","isAcceptState":false},{"x":399,"y":181,"text":"","isAcceptState":false},{"x":506,"y":181,"text":"","isAcceptState":false},{"x":506,"y":294,"text":"","isAcceptState":false},{"x":506,"y":405,"text":"","isAcceptState":false},{"x":727,"y":181,"text":"","isAcceptState":false},{"x":727,"y":294,"text":"","isAcceptState":false},{"x":727,"y":405,"text":"","isAcceptState":false},{"x":62,"y":597,"text":"","isAcceptState":false},{"x":172,"y":597,"text":"","isAcceptState":false},{"x":286,"y":597,"text":"","isAcceptState":false},{"x":399,"y":597,"text":"","isAcceptState":false},{"x":506,"y":597,"text":"","isAcceptState":false}],"links":[{"type":"StartLink","node":0,"text":"","deltaX":-39,"deltaY":-49},{"type":"Link","nodeA":0,"nodeB":1,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":1,"nodeB":2,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":3,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":4,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":0,"nodeB":6,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":7,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":8,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":8,"nodeB":9,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":9,"nodeB":10,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":10,"nodeB":11,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":11,"nodeB":12,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":12,"nodeB":13,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":6,"nodeB":17,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":17,"nodeB":18,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":18,"nodeB":23,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":23,"nodeB":24,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":24,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":3,"nodeB":23,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":2,"nodeB":18,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":1,"nodeB":17,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":17,"nodeB":16,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":18,"nodeB":19,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":23,"nodeB":22,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":24,"nodeB":25,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":16,"nodeB":15,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":19,"nodeB":20,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":22,"nodeB":21,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":25,"nodeB":26,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":15,"nodeB":10,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":20,"nodeB":11,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":21,"nodeB":12,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":26,"nodeB":13,"text":"1","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":8,"nodeB":15,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":15,"nodeB":20,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":20,"nodeB":21,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":21,"nodeB":26,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":22,"nodeB":25,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":19,"nodeB":22,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":16,"nodeB":19,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":7,"nodeB":16,"text":"0","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":13,"nodeB":14,"text":"2","lineAngleAdjust":3.141592653589793,"parallelPart":0.5203619909502263,"perpendicularPart":0.0},{"type":"Link","nodeA":26,"nodeB":29,"text":"2","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":25,"nodeB":28,"text":"2","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":24,"nodeB":27,"text":"2","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":4,"nodeB":5,"text":"2","lineAngleAdjust":0.0,"parallelPart":0.37104072398190047,"perpendicularPart":0.0},{"type":"Link","nodeA":14,"nodeB":29,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":29,"nodeB":28,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":28,"nodeB":27,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":27,"nodeB":5,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":5,"nodeB":0,"text":"","lineAngleAdjust":0.0,"parallelPart":0.20601503759398496,"perpendicularPart":39.0},{"type":"Link","nodeA":30,"nodeB":31,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":9,"nodeB":30,"text":"2","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":10,"nodeB":31,"text":"2","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":11,"nodeB":32,"text":"2","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":12,"nodeB":33,"text":"2","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":13,"nodeB":34,"text":"2","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":31,"nodeB":32,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":32,"nodeB":33,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":33,"nodeB":34,"text":"","lineAngleAdjust":0.0,"parallelPart":0.5,"perpendicularPart":0.0},{"type":"Link","nodeA":34,"nodeB":14,"text":"","lineAngleAdjust":0.0,"parallelPart":0.7163134930132371,"perpendicularPart":17.935289622444646}]}"#,
    ).unwrap();

    let on_terminate = {
        shadow_clone!(word);
        move |_outcome| {
            log::info!("sample terminated");
            let mut seed = [0; 32];
            for v in seed.iter_mut() {
                *v = (randfloat() * 256.0) as u8;
            }
            let mut rng = ChaCha8Rng::from_seed(seed);

            let mut new_word = String::new();
            for i in 0..(rng.gen_range(5..20)) {
                if rng.gen_ratio(1, 8) {
                    new_word.push('2');
                } else {
                    new_word.push(if rng.gen_ratio(1, 2) { '0' } else { '1' })
                }
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
    html!(
        <>
            <nav class="navbar bg-body-tertiary">
                <div class="container-fluid">
                    <a href="/" classes="navbar-brand">{"RUDN FSM tutorial"}</a>
                    // <div class="nav-item">
                    //     <Link<Route> classes="nav-link" to={Route::Editor}>{"Редактор автоматов"}</Link<Route>>
                    // </div>
                    // <div class="nav-item">
                    //     <Link<Route> classes="nav-link" to={Route::Scripter}>{"Отладка заданий"}</Link<Route>>
                    // </div>
                    // <ProfileNav />
                </div>
            </nav>
            <Container>
                <h1>{"Введение в конечные автоматы"}</h1>
                <hr />
                <h2>{"1. Формальные языки"}</h2>
                <p>
                    <em>{"Строка"}</em>{" — это конечная последовательность символов: "}<WordDisplay word="10010101"/>{" — строка, состоящая из символов 0 и 1."}
                </p>
                <p>
                    {"Строка может быть пустой, то есть состоять из нуля символов. Такая строка обозначается символом эпсилон: "}<WordDisplay word="" />
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
                    {"Их могут распознавать "}<em>{"конечные автоматы."}</em>
                </p>
                <hr />


                <h2>{"2. Конечные автоматы"}</h2>
                <p>{"Конечный автомат состоит из кружочков "}<em>{"(состояний)"}</em>{" и направленных стрелочек между ними "}<em>{"(переходов)."}</em></p>
                <SampleFSMPlayer />

                <p>{"Переходы могут приходить из пустоты "}<em>{"(начальные переходы)"}</em>{" или соединять два состояния, или одно состояние само с собой."}</p>
                <p>{"Состояния могут быть обведены один или два раза. Кружочек, который обведен два раза, называется "}<em>{"принимающим состоянием."}</em></p>
                <p>{"Попробуйте создать эти объекты в поле снизу. Учтите, что можно вводить текст только, когда мышь находится над полем, и оно имеет красную обводку."}</p>
                <LinkShowcaseFSMPlayer />

                <p>{"Когда автомат обрабатывает строку, он хранит внутри себя указатель на состояние. "}
                {"Изначально указатель указывает на те состояния, к которым есть входящие переходы — стрелочки из пустоты."}</p>
                <p>{"На каждом шаге автомат смотрит на все стрелочки из текущего состояния. Если на стрелочке написана следующая буква в строке, то по этой стрелочке осуществляется переход."}</p>
                <p>{"В поле ниже, из состояния A есть две стрелочки: с надписью 0 или 1. Когда строка — "}<WordDisplay word="00" />{", то автомат делает переход по верхней стрелочке, а когда "}<WordDisplay word="01" />{", то по нижней."}</p>
                <ZeroZeroOrZeroOneFSM />

                <p>
                    {"Автомат может двигаться только в одном направлении по строке: каждый переход удаляет какое-то количество символов с начала. "}
                    {"Существуют также пустые переходы, которые называются лямбда-переходы: они берут ноль символов."}
                </p>

                <p>{"Если у автомата закончилась строка, и он оказался в принимающем кружочке, то автомат принимает переданное ему слово."}</p>

                <ul>
                    <li>{"Если у автомата закончилась строка, и он оказался в обычном кружочке, то он отвергает строку;"}</li>
                    <li>{"Если автомат, посередине строки, оказался в таком состоянии, что у него нет пути дальше, то он также отвергает строку."}</li>
                </ul>


            </Container>
        </>
    )
}
