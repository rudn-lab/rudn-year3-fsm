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
    let fsm_to_load = use_state(|| None);

    let fsm_output = use_state(|| None);
    let fsm_json_content = use_state(|| String::from("{}"));

    let save_fsm = {
        shadow_clone!(fsm, fsm_output, fsm_json_content);
        move |v| {
            fsm_json_content.set(serde_json::to_string(&v).unwrap());
            fsm.set(v);
            fsm_output.set(None);
        }
    };

    let oninput_json = {
        shadow_clone!(fsm_json_content);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            fsm_json_content.set(target.value());
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

    let do_load_json = {
        shadow_clone!(fsm_output, fsm_json_content, fsm_to_load);
        move |ev: MouseEvent| {
            ev.prevent_default();
            let fsm_text: String = (&*fsm_json_content).clone();
            match serde_json::from_str::<StateMachine>(&fsm_text) {
                Ok(new_fsm) => {
                    fsm_to_load.set(Some(new_fsm));
                    fsm_output.set(None);
                }
                Err(why) => {
                    gloo::dialogs::alert(&format!("Error in JSON: {why}"));
                }
            }
        }
    };

    if fsm_to_load.as_ref() == Some(&*fsm) {
        log::debug!("FSM successfully set, clearing fsm_to_load");
        fsm_to_load.set(None);
    } else {
        // log::debug!("FSM not propagated yet");
    }

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
                <p class="fs-3 text-warning">{"ERROR: "}{
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
            <div class="alert alert-warning attention">{"Эта страница используется для создания и отладки заданий и тестирующей системы, и не требуется для решения задач."}</div>
            <h1>{"FSM Editor"}</h1>
            <p>{"You can edit, save, load and test state machines here."}</p>
            <Row>
                <Column>
                    <Canvas onchange={save_fsm} init={(&*fsm_to_load).clone()} />
                </Column>
                <Column>
                    <form class="input-group mb-3">
                        <input class="form-control" type="text" placeholder="Word to test" oninput={word_input} />
                        <button type="submit" class="btn btn-outline-primary" onclick={do_check_word}>{"Check word"}</button>
                    </form>
                    {fsm_output_html}
                    <form class="input-group mb-3">
                        <input type="text" class="form-control" value={(&*fsm_json_content).clone()} oninput={oninput_json}/>
                        <button type="submit" class="btn btn-outline-danger" onclick={do_load_json}>{"Load"}</button>

                    </form>
                </Column>
            </Row>

        </>
    }
}
