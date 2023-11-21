use api::{SubmissionVerdict, TaskInfo, UserTaskSubmission, UserTaskSubmissions};
use fsm::{
    fsm::{FSMOutput, StateMachine},
    tester::FSMTester,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use shadow_clone::shadow_clone;
use yew::{prelude::*, suspense::use_future};
use yew_autoprops::autoprops_component;
use yew_bootstrap::{
    component::{Column, Row, Spinner},
    icons::BI,
};
use yew_hooks::{use_async, use_interval, use_local_storage};

use crate::canvas::Canvas;

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
    fn randfloat() -> f64;
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
                html!(<button type="button" class="btn btn-outline-success" disabled={true}>{"Отправить и тестировать на сервере"}<Spinner small={true} /></button>)
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
                    <button type="button" class="btn btn-outline-success" onclick={send_to_server}>{"Отправить и тестировать на сервере"}</button>
                    </>)
            } else {
                html!(<button type="button" class="btn btn-outline-success" onclick={send_to_server}>{"Отправить и тестировать на сервере"}</button>)
            };

            let onselect = {
                shadow_clone!(fsm_to_load);
                move |fsm| {
                    fsm_to_load.set(Some(fsm));
                }
            };

            let make_examples = {
                let script = task.script.clone();
                shadow_clone!(current_fsm, fsm_to_load, local_test_outcome, examples);
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
                            html! {
                                <tr>
                                    <td>
                                        <WordDisplay word={a.clone()} response={FSMOutput::Accept} />
                                    </td>
                                    <td>
                                        <WordDisplay word={b.clone()} response={FSMOutput::Reject} />
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
                shadow_clone!(current_fsm, fsm_to_load, local_test_outcome, examples);
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
                                    examples.set(html!(
                                        <p>{"Ваше решение не работает для слова: "}<WordDisplay word={word_to_test.0} response={word_to_test.1} /></p>
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
                <Row>
                    <Column>
                        <div>
                        <div class="btn-group" role="group">
                            <button type="button" class="btn btn-outline-primary" onclick={run_local_test}>{"Тестировать локально"}</button>
                            {send_to_server_button}
                        </div>

                        </div>
                        <div>
                            <Canvas onchange={set_fsm} init={(&*fsm_to_load).clone()} />
                        </div>
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
                    </Column>
                </Row>
                </>
            }
        }
        Err(ref failure) => {
            html!(<div class="alert alert-danger">{"Ошибка при загрузке этой задачи. Перезагрузите страницу. Причина: "}{failure}</div>)
        }
    };

    Ok(result_html)
}

#[autoprops_component(WordDisplay)]
fn word_display(word: &AttrValue, response: &FSMOutput) -> Html {
    match response {
        FSMOutput::Accept => {
            if word.as_str().is_empty() {
                html!(<span class="badge text-bg-success">{"λ"}</span>)
            } else {
                html!(<span class="text-success">{word}</span>)
            }
        }
        FSMOutput::Reject => {
            if word.as_str().is_empty() {
                html!(<span class="badge text-bg-danger">{"λ"}</span>)
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
