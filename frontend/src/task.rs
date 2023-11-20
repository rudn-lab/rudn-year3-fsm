use api::{TaskInfo, UserTaskSubmissions};
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
use yew_hooks::use_local_storage;

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
        <h1>{"Loading task info..."}<Spinner/></h1>
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

    let resp = use_future(|| async move {
        reqwest::get(format!(
            "https://fsm-api.rudn-lab.ru/tasks/{group_slug}/{task_slug}/{}",
            token.as_ref().unwrap_or(&"".to_string())
        ))
        .await?
        .error_for_status()?
        .json::<(TaskInfo, UserTaskSubmissions)>()
        .await
    })?;

    let current_fsm = use_state(StateMachine::default);
    let init_fsm = use_state(StateMachine::default);

    let local_test_outcome = use_state(|| html!());

    let set_fsm = {
        shadow_clone!(current_fsm);
        move |fsm: StateMachine| {
            current_fsm.set(fsm.clone());
        }
    };

    let examples = use_state(|| html!());

    let result_html = match *resp {
        Ok(ref res) => {
            let (task, submissions) = res.clone();

            let onselect = {
                shadow_clone!(init_fsm);
                move |fsm| {
                    init_fsm.set(fsm);
                }
            };

            let make_examples = {
                let script = task.script.clone();
                shadow_clone!(current_fsm, init_fsm, local_test_outcome, examples);
                move |ev: MouseEvent| {
                    log::info!("Starting local test generate!");
                    let fsm = (&*current_fsm).clone();
                    init_fsm.set(fsm.clone());

                    log::debug!("Instantiating tester");
                    let tester = FSMTester::new(fsm, script.clone());
                    let mut tester = match tester {
                        Ok(t) => t,
                        Err(why) => {
                            local_test_outcome.set(html!(<span class="text-danger">{"BUG IN TASK (please report this!): "}{why}</span>));
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
                    for _ in 0..3 {
                        let test = tester.make_test_case(rng.gen(), true);
                        let test = match test {
                            Ok(t) => t,
                            Err(why) => {
                                local_test_outcome.set(html!(<span class="text-danger">{"BUG IN TASK (please report this!): "}{why}</span>));
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
                                local_test_outcome.set(html!(<span class="text-danger">{"BUG IN TASK (please report this!): "}{why}</span>));
                                return;
                            }
                        };
                        match test.1 {
                            FSMOutput::Accept => tests_acc.push(test.0),
                            FSMOutput::Reject => tests_rej.push(test.0),
                        }
                    }
                    let rows = tests_acc
                        .iter()
                        .zip(tests_rej.iter())
                        .map(|(a, b)| {
                            html! {
                                <tr>
                                    <td>
                                        <div class="text-success overflow-scroll">{a}</div>
                                    </td>
                                    <td>
                                        <div class="text-danger overflow-scroll">{b}</div>
                                    </td>
                                </tr>
                            }
                        })
                        .collect::<Html>();

                    examples.set(html!(
                        <table class="table overflow-scroll">
                            <thead>
                                <tr>
                                    <th scope="col">{"These words are Accepted"}</th>
                                    <th scope="col">{"These words are Rejected"}</th>
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
                shadow_clone!(current_fsm, init_fsm, local_test_outcome, examples);
                move |ev: MouseEvent| {
                    log::info!("Starting local evaluation!");
                    ev.prevent_default();
                    let fsm = (&*current_fsm).clone();
                    init_fsm.set(fsm.clone());
                    log::debug!("Instantiating tester");
                    let tester = FSMTester::new(fsm, script.clone());
                    let mut tester = match tester {
                        Ok(t) => t,
                        Err(why) => {
                            local_test_outcome.set(html!(<span class="text-danger">{"BUG IN TASK (please report this!): "}{why}</span>));
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
                            local_test_outcome.set(html!(<span class="text-danger">{"BUG IN TASK (please report this!): "}{why}</span>));
                            return;
                        }
                        Ok(res) => {
                            log::debug!("Tester result: {res:?}");
                            match res {
                                fsm::tester::FSMTestingOutput::Ok(t) => local_test_outcome.set(html!(<span class="text-success">{"OK: all "}{t}{" tests passed"}</span>)),
                                fsm::tester::FSMTestingOutput::WrongAnswer {
                                    successes,
                                    total_tests,
                                    first_failure_seed,
                                    first_failure_expected_result,
                                } => {
                                    local_test_outcome.set(html!(<span class="text-warning">{"WRONG: only "}{successes}{"/"}{total_tests}{" passed"}</span>));
                                    let word_to_test = match tester.make_test_case(first_failure_seed, first_failure_expected_result.into()) {
                                        Ok(t) => t,
                                        Err(why) => {
                                            local_test_outcome.set(html!(<span class="text-danger">{"BUG IN TASK (please report this!): "}{why}</span>));
                                            return;
                                        }
                                    };
                                    let word = match word_to_test.1{
                                        FSMOutput::Accept => html!(<span class="text-success">{word_to_test.0}</span>),
                                        FSMOutput::Reject => html!(<span class="text-danger">{word_to_test.0}</span>),
                                    };
                                    examples.set(html!(
                                        <p>{"Your solution fails for word: "}{word}</p>
                                    ));
                            },
                                fsm::tester::FSMTestingOutput::FSMInvalid(why) => local_test_outcome.set(html!(<span class="text-warning">{"INVALID: "}{why}</span>)),
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
                            <button type="button" class="btn btn-outline-primary" onclick={run_local_test}>{"Test locally"}</button>
                            <button type="button" class="btn btn-outline-success">{"Send and test on server"}</button>
                        </div>

                        </div>
                        <div>
                            <Canvas onchange={set_fsm} init={(&*init_fsm).clone()} />
                        </div>
                        <div>
                            <div>
                                {(&*examples).clone()}
                                <button type="button" class="btn btn-outline-primary" onclick={make_examples}>{"Examples?"}</button>
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
            html!(<div class="alert alert-danger">{"Error while loading this task. Try reloading the page. Reason: "}{failure}</div>)
        }
    };

    Ok(result_html)
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    fn unix_time_to_locale_string(time: u64) -> String;
}

#[autoprops_component(SubmissionList)]
fn submissions_list(submissions: &UserTaskSubmissions, onselect: &Callback<StateMachine>) -> Html {
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

            let verdict = match &v.verdict {
                api::SubmissionVerdict::Ok(tests) => html!(
                    <span class="d-inline-block text-success" tabindex="0" data-bs-toggle="popover" data-bs-trigger="hover focus" data-bs-content={format!("OK: passed all {tests} tests")}>
                        {BI::CHECK_CIRCLE_FILL}
                    </span>
                ),
                api::SubmissionVerdict::WrongAnswer { total_tests, successes, .. } => html!(
                    <span class="d-inline-block text-warning" tabindex="0" data-bs-toggle="popover" data-bs-trigger="hover focus" data-bs-content={format!("WRONG: passed only {successes} out of {total_tests} tests")}>
                        {BI::EXCLAMATION_TRIANGLE_FILL}
                    </span>
                ),
                api::SubmissionVerdict::InvalidFSM(err) => {
                    let why = match err {
                        fsm::fsm::FSMError::InfiniteLoop => "There is a loop of empty links, which will never terminate",
                        fsm::fsm::FSMError::NoEntryLinks => "There are no entry links into the state machine",
                        fsm::fsm::FSMError::DisjointedLink(_) => "There is a link that refers to nodes that don't exist",
                    };
                    html!(
                    <span class="d-inline-block text-danger" tabindex="0" data-bs-toggle="popover" data-bs-trigger="hover focus" data-bs-content={format!("INVALID: {why}")}>
                        {BI::SHIELD_FILL_X}
                    </span>
                )},
                api::SubmissionVerdict::TaskInternalError(why) => html!(
                    <span class="d-inline-block text-info" tabindex="0" data-bs-toggle="popover" data-bs-trigger="hover focus" data-bs-content={format!("Error in task: {why}. Please contact jury!")}>
                        {BI::BUG_FILL}
                    </span>
                ),
            };

            html!(
                <tr>
                    <th scope="row">{v.id}</th>
                    <td>{unix_time_to_locale_string(v.when_unix_time)}
                    {if Some(v) == submissions.latest_ok_submission.as_ref() {" (latest OK)"} else if Some(v) == submissions.latest_submission.as_ref() {" (latest)"} else {""}}
                    </td>
                    <td><button class="btn btn-link" onclick={load_this}>{v.solution.nodes.len()}{" nodes, "}{v.solution.links.len()}{" links"}</button></td>
                    <td>{verdict}</td>
                </tr>
            )
        })
        .collect::<Html>();
    html!(
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">{"ID"}</th>
                    <th scope="col">{"Date"}</th>
                    <th scope="col">{"Load version"}</th>
                    <th scope="col">{"Verdict"}</th>
                </tr>
            </thead>
            <tbody>
                {submission_list}
            </tbody>
        </table>
    )
}
