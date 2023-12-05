use api::{SubmissionVerdict, TaskInfo, UserTaskSubmission, UserTaskSubmissions};
use fsm::{
    fsm::{FSMOutput, StateMachine},
    tester::FSMTester,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use shadow_clone::shadow_clone;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*, suspense::use_future};
use yew_autoprops::autoprops_component;
use yew_bootstrap::{
    component::{Column, Row, Spinner},
    icons::BI,
};
use yew_hooks::{use_async, use_interval, use_local_storage};

use crate::canvas_player::CanvasPlayer;

#[derive(Properties, PartialEq, Clone)]
pub struct TaskPageProps {
    pub group_slug: AttrValue,
    pub task_slug: AttrValue,
}

#[function_component(TaskPage)]
pub fn task_page(props: &TaskPageProps) -> Html {
    let TaskPageProps {
        group_slug,
        task_slug,
    } = props;
    let fallback = html! {
        <h1>{"Загружаем задание..."}<Spinner/></h1>
    };
    html!(
        <Suspense {fallback}>
            <TaskPageInner {group_slug} {task_slug} />
        </Suspense>
    )
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    pub fn randfloat() -> f64;
}

#[function_component(TaskPageInner)]
fn task_page_inner(props: &TaskPageProps) -> HtmlResult {
    let TaskPageProps {
        group_slug,
        task_slug,
    } = props.clone();

    let token = use_local_storage::<String>("token".to_string());

    let resp = {
        shadow_clone!(token, group_slug, task_slug);
        use_future(|| async move {
            reqwest::get(format!(
                "https://fsm-api.rudn-lab.ru/tasks/{group_slug}/{task_slug}/{}",
                token.as_ref().unwrap_or(&"".to_string())
            ))
            .await?
            .error_for_status()?
            .json::<(TaskInfo, UserTaskSubmissions)>()
            .await
        })?
    };

    let current_fsm = use_state(StateMachine::default);
    let fsm_to_load = use_state(|| None);
    let init_word = use_state(|| None);

    let local_test_outcome = use_state(|| html!());

    let set_fsm = {
        shadow_clone!(current_fsm);
        move |fsm: StateMachine| {
            // log::info!("Received current FSM: {fsm:?}");
            current_fsm.set(fsm.clone());
        }
    };

    let send_to_server_async: yew_hooks::prelude::UseAsyncHandle<UserTaskSubmission, String> = {
        shadow_clone!(current_fsm, fsm_to_load, token);
        use_async(async move {
            let fsm = (&*current_fsm).clone();
            fsm_to_load.set(Some(fsm.clone()));
            Ok(reqwest::Client::new()
                .post(format!(
                    "https://fsm-api.rudn-lab.ru/tasks/{group_slug}/{task_slug}/{}",
                    token.as_ref().unwrap_or(&"".to_string())
                ))
                .json(&fsm)
                .send()
                .await
                .map_err(|v| v.to_string())?
                .json::<UserTaskSubmission>()
                .await
                .map_err(|v| v.to_string())?)
        })
    };

    let send_to_server = {
        shadow_clone!(send_to_server_async);
        move |ev: MouseEvent| {
            ev.prevent_default();
            send_to_server_async.run()
        }
    };

    let examples = use_state(|| html!());

    if fsm_to_load.as_ref() == Some(&*current_fsm) {
        log::debug!("FSM successfully set, clearing fsm_to_set");
        fsm_to_load.set(None);
    } else {
        // log::debug!("FSM not propagated yet");
    }

    let result_html = match *resp {
        Ok(ref res) => {
            let (task, submissions) = res.clone();

            let send_to_server_button = if send_to_server_async.loading {
                html!(<button type="button" class="btn btn-outline-success" disabled={true}>{"Сохранить и сдать задание"}<Spinner small={true} /></button>)
            } else if let Some(data) = &send_to_server_async.data {
                log::info!("Submission result: {data:?}");
                gloo::utils::document()
                    .location()
                    .unwrap()
                    .reload()
                    .unwrap();
                html!("Ждем перезагрузки страницы...")
            } else if let Some(error) = &send_to_server_async.error {
                html!(<>
                    <p class="text-danger">{"Ошибка при отправке: "}{error}</p>
                    <button type="button" class="btn btn-outline-success" onclick={send_to_server}>{"Сохранить и сдать задание"}</button>
                    </>)
            } else {
                html!(<button type="button" class="btn btn-outline-success" onclick={send_to_server}>{"Сохранить и сдать задание"}</button>)
            };

            let onselect = {
                shadow_clone!(fsm_to_load);
                move |fsm| {
                    fsm_to_load.set(Some(fsm));
                }
            };

            let make_examples = {
                let script = task.script.clone();
                shadow_clone!(
                    current_fsm,
                    fsm_to_load,
                    local_test_outcome,
                    examples,
                    init_word
                );
                move |ev: MouseEvent| {
                    ev.prevent_default();
                    log::info!("Starting local test generate!");
                    let fsm = (&*current_fsm).clone();
                    fsm_to_load.set(Some(fsm.clone()));

                    log::debug!("Instantiating tester");
                    let tester = FSMTester::new(fsm, &script);
                    let mut tester = match tester {
                        Ok(t) => t,
                        Err(why) => {
                            local_test_outcome.set(html!(<span class="text-danger">{"ОШИБКА ЗАДАНИЯ (пожалуйста сообщите об этом!): "}{why}</span>));
                            return;
                        }
                    };
                    log::debug!("Instantiating seed");
                    let mut seed = [0u8; 32];
                    for v in seed.iter_mut() {
                        *v = (randfloat() * 256.0) as u8;
                    }
                    let mut rng = ChaCha8Rng::from_seed(seed);
                    let mut tests_acc = Vec::with_capacity(3);
                    let mut tests_rej = Vec::with_capacity(3);
                    while !(tests_acc.len() >= 3 && tests_rej.len() >= 3) {
                        let test = tester.make_test_case(rng.gen(), true);
                        let test = match test {
                            Ok(t) => t,
                            Err(why) => {
                                local_test_outcome.set(html!(<span class="text-danger">{"ОШИБКА ЗАДАНИЯ (пожалуйста сообщите об этом!): "}{why}</span>));
                                return;
                            }
                        };
                        match test.1 {
                            FSMOutput::Accept => tests_acc.push(test.0),
                            FSMOutput::Reject => tests_rej.push(test.0),
                        }

                        let test = tester.make_test_case(rng.gen(), false);
                        let test = match test {
                            Ok(t) => t,
                            Err(why) => {
                                local_test_outcome.set(html!(<span class="text-danger">{"ОШИБКА ЗАДАНИЯ (пожалуйста сообщите об этом!): "}{why}</span>));
                                return;
                            }
                        };
                        match test.1 {
                            FSMOutput::Accept => tests_acc.push(test.0),
                            FSMOutput::Reject => tests_rej.push(test.0),
                        }
                    }
                    log::debug!("Generated tests: ACC={tests_acc:?}, REJ={tests_rej:?}");
                    let rows = tests_acc
                        .iter()
                        .zip(tests_rej.iter())
                        .map(|(a, b)| {
                            let init_word_out = init_word.clone();
                            let out_a = a.clone();
                            let load_a = move |ev: MouseEvent| {
                                ev.prevent_default();
                                init_word_out.set(Some(out_a.clone()));
                            };
                            let init_word_out = init_word.clone();
                            let out_b = a.clone();
                            let load_b = move |ev: MouseEvent| {
                                ev.prevent_default();
                                init_word_out.set(Some(out_b.clone()));
                            };
                            html! {
                                <tr>
                                    <td>
                                        <WordDisplay word={a.clone()} response={FSMOutput::Accept} />
                                        <button class="btn btn-outline-primary btn-sm" onclick={load_a} >{BI::ARROW_UP_LEFT_SQUARE}</button>
                                    </td>
                                    <td>
                                        <WordDisplay word={b.clone()} response={FSMOutput::Reject} />
                                        <button class="btn btn-outline-primary btn-sm" onclick={load_b} >{BI::ARROW_UP_LEFT_SQUARE}</button>
                                    </td>
                                </tr>
                            }
                        })
                        .collect::<Html>();

                    examples.set(html!(
                        <table class="table overflow-scroll">
                            <thead>
                                <tr>
                                    <th scope="col">{"Эти слова принимаются"}</th>
                                    <th scope="col">{"Эти слова не принимаются"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {rows}
                            </tbody>
                        </table>
                    ))
                }
            };

            let run_local_test = {
                let script = task.script.clone();
                shadow_clone!(
                    current_fsm,
                    fsm_to_load,
                    local_test_outcome,
                    examples,
                    init_word
                );
                move |ev: MouseEvent| {
                    log::info!("Starting local evaluation!");
                    ev.prevent_default();
                    let fsm = (&*current_fsm).clone();
                    fsm_to_load.set(Some(fsm.clone()));
                    log::debug!("Instantiating tester");
                    let tester = FSMTester::new(fsm, &script);
                    let mut tester = match tester {
                        Ok(t) => t,
                        Err(why) => {
                            local_test_outcome.set(html!(<span class="text-danger">{"ОШИБКА ЗАДАНИЯ (пожалуйста сообщите об этом!): "}{why}</span>));
                            return;
                        }
                    };
                    log::debug!("Instantiating seed");
                    let mut seed = [0u8; 8];
                    for v in seed.iter_mut() {
                        *v = (randfloat() * 256.0) as u8;
                    }
                    let seed = i64::from_be_bytes(seed);
                    log::debug!("Running tester");
                    match tester.run_testing(seed) {
                        Err(why) => {
                            local_test_outcome.set(html!(<span class="text-danger">{"ОШИБКА ЗАДАНИЯ (пожалуйста сообщите об этом!): "}{why}</span>));
                            return;
                        }
                        Ok(res) => {
                            log::debug!("Tester result: {res:?}");
                            examples.set(html!());
                            match res {
                                fsm::tester::FSMTestingOutput::Ok(t) => local_test_outcome.set(html!(<span class="text-success">{"OK: все "}{t}{" тесты прошли"}</span>)),
                                fsm::tester::FSMTestingOutput::WrongAnswer {
                                    successes,
                                    total_tests,
                                    first_failure_seed,
                                    first_failure_expected_result,
                                } => {
                                    local_test_outcome.set(html!(<span class="text-warning">{"НЕВЕРНО: только "}{successes}{"/"}{total_tests}{" тестов прошло"}</span>));
                                    let word_to_test = match tester.make_test_case(first_failure_seed, first_failure_expected_result.into()) {
                                        Ok(t) => t,
                                        Err(why) => {
                                            local_test_outcome.set(html!(<span class="text-danger">{"ОШИБКА ЗАДАНИЯ (пожалуйста сообщите об этом!): "}{why}</span>));
                                            return;
                                        }
                                    };
                                    let response = match word_to_test.1 {
                                        FSMOutput::Accept => " (слово следует принимать, а ваш автомат отвергает)",
                                        FSMOutput::Reject => " (слово следует отвергать, а ваш автомат принимает)",
                                    };
                                    let failed_word = word_to_test.0.clone();
                                    let load_failed = {
                                        shadow_clone!(init_word);
                                        move |ev: MouseEvent| {
                                            ev.prevent_default();
                                            init_word.set(Some(failed_word.clone()))
                                        }
                                    };
                                    examples.set(
                                        html!(
                                        <p>{"Ваше решение не работает для слова: "}<WordDisplay word={word_to_test.0} response={word_to_test.1} />
                                        <button class="btn btn-sm btn-outline-primary mx-2" onclick={load_failed}>{BI::ARROW_UP_LEFT_SQUARE}</button>
                                        {response}
                                        </p>
                                        ));
                            },
                                fsm::tester::FSMTestingOutput::FSMInvalid(why) => local_test_outcome.set(html!(<span class="text-warning">{"НЕВЕРНЫЙ ФОРМАТ: "}{why}</span>)),
                            }
                        }
                    }
                }
            };
            html! {
                <>
                <h1>{task.name}</h1>
                <p>{task.legend}</p>
                        <div>
                        <div class="btn-group" role="group">
                            <button type="button" class="btn btn-outline-primary" onclick={run_local_test}>{"Тестировать локально"}</button>
                            {send_to_server_button}
                        </div>

                        </div>
                        <Row>
                            <Column>
                                <TestableFSM onchange={set_fsm}
                                init={(&*fsm_to_load).clone()}
                                init_word={(&*init_word).clone()}/>
                            </Column>
                            <Column>
                                <ul>
                                    <li>{"Создать кружочек: двойной клик по пустому пространству"}</li>
                                    <li>{"Создать стрелочку: нажать Shift, щелкнуть по начальному кружочку и передвинуть до целевого кружочка"}</li>
                                    <li>{"Создать начальную стрелочку: нажать Shift, щелкнуть по пустому месту и передвинуть до кружочка"}</li>
                                    <li>{"Передвинуть кружочек: щелкнуть и тянуть"}</li>
                                    <li>{"Изогнуть стрелочку: щелкнуть и тянуть"}</li>
                                    <li>{"Удалить что-то: щелкнуть, затем нажать Delete"}</li>
                                    <li>{"Сделать кружочек принимающим (или наоборот): дважды щелкнуть по нему"}</li>
                                    <li>{"Текст на стрелочках - условие для перехода между состояниями"}</li>
                                    <li>{"Текст можно писать только, когда мышь внутри поля (есть красная обводка)"}</li>
                                    <li>{"Если на стрелочке нет текста, на ней нарисована буква эпсилон; пиши текст, чтобы убрать"}</li>
                                </ul>
                            </Column>
                        </Row>
                        <div>
                            <div>
                                {(&*examples).clone()}
                                <button type="button" class="btn btn-outline-primary" onclick={make_examples}>{"Примеры?"}</button>
                            </div>
                            <div>
                                {(&*local_test_outcome).clone()}
                            </div>
                            <SubmissionList {submissions} {onselect} />
                        </div>
                </>
            }
        }
        Err(ref failure) => {
            html!(<div class="alert alert-danger">{"Ошибка при загрузке этой задачи. Перезагрузите страницу. Причина: "}{failure}</div>)
        }
    };

    Ok(result_html)
}

#[autoprops_component]
fn TestableFSM(
    onchange: Callback<StateMachine>,
    init: Option<StateMachine>,
    init_word: Option<AttrValue>,
) -> Html {
    let word = use_state_eq(|| String::from(""));
    let is_running = use_state_eq(|| false);
    let validity = use_state_eq(|| false);

    let play_pulse = use_state_eq(|| 0usize);

    let fsm = use_state_eq(StateMachine::default);

    if let Some(init) = init {
        if &*fsm != &init {
            fsm.set(init);
        }
    }

    if let Some(init_word) = init_word {
        if &*word != init_word {
            word.set(init_word.to_string());
        }
    }

    let on_is_running = {
        shadow_clone!(is_running);
        move |run| {
            is_running.set(run);
        }
    };

    let on_validity = {
        shadow_clone!(validity);
        move |v| {
            validity.set(v);
        }
    };

    let oninput = {
        shadow_clone!(word);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            word.set(target.value());
        }
    };

    let onclick = {
        shadow_clone!(is_running, play_pulse);
        move |ev: MouseEvent| {
            ev.prevent_default();
            is_running.set(true);
            play_pulse.set(*play_pulse + 1);
        }
    };

    let on_fsm_apply = {
        shadow_clone!(fsm);
        move |new_fsm: StateMachine| {
            fsm.set(new_fsm.clone());
            onchange.emit(new_fsm);
        }
    };

    html!(
        <>
            <CanvasPlayer word={(&*word).clone()}
            fsm={(&*fsm).clone()} editable={true} speed_changeable={true}
            auto_restart={true} show_status_indicator={true} show_transport_buttons={true}
            pause_on_restart={true} {on_fsm_apply}
            speed={860} auto_play={false} show_steps_indicator={true}
            play_pulse={*play_pulse} {on_validity}
            {on_is_running}/>
            <form class="input-group my-2" style="width: 800px; margin: 0 auto;">
                <span class="input-group-text">{"Введите слово для проверки: "}</span>
                <input class="form-control" disabled={*is_running || !*validity} value={(&*word).clone()} {oninput} />
                <input class="btn btn-success" disabled={*is_running || !*validity} value="Тест!" type="submit" {onclick} />
            </form>
        </>
    )
}

#[autoprops_component(WordDisplay)]
fn word_display(word: &AttrValue, response: &FSMOutput) -> Html {
    match response {
        FSMOutput::Accept => {
            if word.as_str().is_empty() {
                html!(<span class="badge text-bg-success">{"ε"}</span>)
            } else {
                html!(<span class="text-success">{word}</span>)
            }
        }
        FSMOutput::Reject => {
            if word.as_str().is_empty() {
                html!(<span class="badge text-bg-danger">{"ε"}</span>)
            } else {
                html!(<span class="text-danger">{word}</span>)
            }
        }
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    pub fn unix_time_to_locale_string(time: f64) -> String;
    pub fn prepare_popovers();
}

#[autoprops_component(VerdictDisplay)]
pub fn verdict_display(verdict: &SubmissionVerdict) -> Html {
    match verdict {
        api::SubmissionVerdict::Ok(tests) => html!(
            <span class="d-inline-block text-success fs-2" tabindex="0" data-bs-toggle="popover" data-bs-trigger="hover focus" data-bs-content={format!("OK: прошли все {tests} тестов")}>
                {BI::CHECK_CIRCLE_FILL}
            </span>
        ),
        api::SubmissionVerdict::WrongAnswer {
            total_tests,
            successes,
            ..
        } => html!(
            <span class="d-inline-block text-warning fs-2" tabindex="0" data-bs-toggle="popover" data-bs-trigger="hover focus" data-bs-content={format!("НЕВЕРНО: прошли только {successes} из {total_tests} тестов")}>
                {BI::EXCLAMATION_TRIANGLE_FILL}
            </span>
        ),
        api::SubmissionVerdict::InvalidFSM(err) => {
            let why = match err {
                fsm::fsm::FSMError::InfiniteLoop => {
                    "Есть цикл из пустых связей, который никогда не завершится"
                }
                fsm::fsm::FSMError::NoEntryLinks => "Нет вводных стрелочек в конечный автомат",
                fsm::fsm::FSMError::DisjointedLink(_) => {
                    "Есть стрелочка, которая связана с несуществующим кружочком"
                }
            };
            html!(
                <span class="d-inline-block text-danger fs-2" tabindex="0" data-bs-toggle="popover" data-bs-trigger="hover focus" data-bs-content={format!("НЕВЕРНЫЙ ФОРМАТ: {why}")}>
                    {BI::SHIELD_FILL_X}
                </span>
            )
        }
        api::SubmissionVerdict::TaskInternalError(why) => html!(
            <span class="d-inline-block text-info fs-2" tabindex="0" data-bs-toggle="popover" data-bs-trigger="hover focus" data-bs-content={format!("Error in task: {why}. Please contact jury!")}>
                {BI::BUG_FILL}
            </span>
        ),
    }
}

#[autoprops_component(SubmissionList)]
fn submissions_list(submissions: &UserTaskSubmissions, onselect: &Callback<StateMachine>) -> Html {
    let force = use_force_update();
    use_interval(move || force.force_update(), 1000);

    prepare_popovers();

    let submission_list = submissions
        .submissions
        .iter()
        .map(|v| {
            let machine = v.solution.clone();
            shadow_clone!(onselect);
            let load_this = move |ev: MouseEvent| {
                ev.prevent_default();
                onselect.emit(machine.clone());
            };


            html!(
                <tr>
                    <th scope="row">{v.id}</th>
                    <td>{unix_time_to_locale_string(v.when_unix_time as f64)}
                    {if Some(v) == submissions.latest_ok_submission.as_ref() {" (latest OK)"} else if Some(v) == submissions.latest_submission.as_ref() {" (latest)"} else {""}}
                    </td>
                    <td><button class="btn btn-link" onclick={load_this}>{v.solution.nodes.len()}{" кружочков, "}{v.solution.links.len()}{" стрелочек"}</button></td>
                    <td><VerdictDisplay verdict={v.verdict.clone()} /></td>
                </tr>
            )
        })
        .collect::<Html>();
    html!(
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">{"ID"}</th>
                    <th scope="col">{"Дата"}</th>
                    <th scope="col">{"Загрузить версию"}</th>
                    <th scope="col">{"Вердикт"}</th>
                </tr>
            </thead>
            <tbody>
                {submission_list}
            </tbody>
        </table>
    )
}
