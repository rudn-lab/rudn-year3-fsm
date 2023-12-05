use fsm::{fsm::StateMachine, tester::FSMTester};
use shadow_clone::shadow_clone;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_bootstrap::component::{Column, Row};

const TEMPLATE: &'static str = r#"
// Generate a word. It should have the provided acceptance state:
// if ok==true, it should be accepted, and if false rejected
// Use rng.gen_range(x, y), which generates a random number from x to y inclusive.
fn gen_word(ok) {
    if rng.gen_range(0, 10) < 1 == 0 {
        return "";
    }
    let length = rng.gen_range(1, 25);
    let word = "";
    if ok {
        for i in 0..length {
            word += "1";
        }
    } else {
        for i in 0..length {
            word += rng.gen_range(0, 1);
        }
    }
    
    if !(check_word(word) == ok) {
        word += "0";
    }
    return word;
}

// Check a word:
// return true if the FSM should accept it, and false if it should reject it.
fn check_word(word) {
    if word.is_empty() {
        return false;
    }
    while !word.is_empty() {
        if word.pop() != "1" {
            return false;
        }
    }
    return true;
}
"#;

#[function_component(Scripter)]
pub fn scripter() -> Html {
    let script_text = use_state(|| String::from(TEMPLATE));
    let oninput = {
        shadow_clone!(script_text);
        move |ev: InputEvent| {
            ev.prevent_default();
            let target: HtmlTextAreaElement = ev.target().unwrap().dyn_into().unwrap();
            script_text.set(target.value());
            log::debug!("New textarea value: {} chars", target.value().len());
        }
    };
    let tester = use_state(|| None);
    let error = use_state(|| None);

    let compile = {
        shadow_clone!(script_text, tester, error);
        move |ev: MouseEvent| {
            ev.prevent_default();
            let text = (&*script_text).clone();
            let new_tester = FSMTester::new(StateMachine::default(), &text);
            match new_tester {
                Ok(t) => {
                    tester.set(Some(t));
                    error.set(None);
                }
                Err(why) => {
                    tester.set(None);
                    error.set(Some(format!("{why}")))
                }
            }
        }
    };

    let word_input_state = use_state(String::new);
    let number_input_state = use_state(String::new);
    let output = use_state(String::new);

    let oninput_word = {
        shadow_clone!(word_input_state);
        move |ev: InputEvent| {
            ev.prevent_default();
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            word_input_state.set(target.value());
        }
    };
    let oninput_number = {
        shadow_clone!(number_input_state);
        move |ev: InputEvent| {
            ev.prevent_default();
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            number_input_state.set(target.value());
        }
    };

    let test_word = {
        shadow_clone!(word_input_state, output, tester);
        move |ev: MouseEvent| {
            ev.prevent_default();
            let mut my_tester: FSMTester = tester.as_ref().unwrap().semiclone();
            let word = (&*word_input_state).clone();
            let res = my_tester.check_word(word.clone());
            output.set(format!("Question: check_word {word:?}\nAnswer: {res:?}"));
        }
    };

    let gen_test = {
        shadow_clone!(number_input_state, output, tester);
        move |ev: MouseEvent| {
            ev.prevent_default();
            let mut my_tester: FSMTester = tester.as_ref().unwrap().semiclone();
            let seed = (&*number_input_state).clone();
            let seed = match seed.parse::<i64>() {
                Ok(v) => v,
                Err(why) => {
                    output.set(format!("Cannot parse {seed:?} as i64: {why}"));
                    return;
                }
            };
            let res_t = my_tester.make_test_case(seed, true);
            let res_f = my_tester.make_test_case(seed, false);
            output.set(format!("Question: make_test_case ({seed}, true)\nAnswer: {res_t:?}\n\nQuestion: make_test_case ({seed}, false)\nAnswer: {res_f:?}"));
        }
    };

    let controls = match *tester {
        Some(ref _t) => {
            html!(
                <>
                    <form class="input-group">
                        <input class="form-control" type="text" placeholder="word to test" value={(&*word_input_state).clone()} oninput={oninput_word} />
                        <button class="btn btn-outline-primary" type="submit" onclick={test_word}>{"Test"}</button>
                    </form>
                    <form class="input-group">
                        <input class="form-control" type="number" placeholder="seed to generate" value={(&*number_input_state).clone()} oninput={oninput_number} />
                        <button class="btn btn-outline-primary" type="submit" onclick={gen_test}>{"Generate"}</button>
                    </form>
                    <div class="card">
                        <div class="card-body">
                            <pre>{(&*output).clone()}</pre>
                        </div>
                    </div>
                </>
            )
        }
        None => {
            html!(<p class="text-warning">{"Tester is not initialized, cannot use features"}</p>)
        }
    };

    html!(<>
            <div class="alert alert-warning attention">{"Эта страница используется для создания и отладки заданий и тестирующей системы, и не требуется для решения задач."}</div>
            <Row>
                <Column>
                    <textarea class="form-control" rows="24" {oninput} value={(&*script_text).clone()}>
                    </textarea>
                </Column>
                <Column>
                    <div>
                        <button type="button" class="btn btn-success" onclick={compile}>{"Compile"}</button>
                        <p>{(&*error).clone()}</p>

                        {controls}
                    </div>
                </Column>
            </Row>
        </>
    )
}
