use fsm::fsm::StateMachine;
use shadow_clone::shadow_clone;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_bootstrap::component::{Column, Row};

use crate::canvas::Canvas;

#[function_component(Editor)]
pub fn editor() -> Html {
    let fsm = use_state(StateMachine::default);

    let fsm_output = use_state(|| None);

    let save_fsm = {
        shadow_clone!(fsm, fsm_output);
        move |v| {
            fsm.set(v);
            fsm_output.set(None);
        }
    };

    let word_to_check_state = use_state(String::new);
    let word_input = {
        shadow_clone!(word_to_check_state, fsm_output);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            word_to_check_state.set(target.value());
            fsm_output.set(None);
        }
    };

    let do_check_word = {
        shadow_clone!(fsm, fsm_output, word_to_check_state);
        move |ev: MouseEvent| {
            ev.prevent_default();
            let fsm = &(*fsm);
            let word = &*word_to_check_state;
            fsm_output.set(Some(fsm.evaluate(word)));
        }
    };

    let fsm_output_html = match *fsm_output {
        None => html!(),
        Some(ref v) => match v {
            Ok(output) => html!(
                <p class="fs-3">{"Output: "}{
                    match output {
                        fsm::fsm::FSMOutput::Accept => html!(<span class="text-success">{"ACCEPT"}</span>),
                        fsm::fsm::FSMOutput::Reject => html!(<span class="text-danger">{"REJECT"}</span>),
                    }
                }
                </p>
            ),
            Err(err) => html!(
                <p class="fs-3">{"ERROR: "}{
                    match err{
                        fsm::fsm::FSMError::InfiniteLoop => html!("FSM contains infinite loop"),
                        fsm::fsm::FSMError::NoEntryLinks => html!("FSM contains no entry links"),
                        fsm::fsm::FSMError::DisjointedLink(_) => html!("FSM contains disconnected links"),
                    }
                }
                </p>
            ),
        },
    };

    html! {
        <>
            <h1>{"FSM Editor"}</h1>
            <p>{"You can edit, save, load and test state machines here."}</p>
            <Row>
                <Column>
                    <Canvas onchange={save_fsm} />
                </Column>
                <Column>
                    <form class="input-group mb-3">
                        <input class="form-control" type="text" placeholder="Word to test" oninput={word_input} />
                        <button type="submit" class="btn btn-outline-primary" onclick={do_check_word}>{"Check word"}</button>
                    </form>
                    {fsm_output_html}
                </Column>
            </Row>

        </>
    }
}
