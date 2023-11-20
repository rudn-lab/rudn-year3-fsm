use api::{TaskInfo, UserTaskSubmissions};
use fsm::{fsm::StateMachine, tester::FSMTester};
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
        move |fsm| {
            current_fsm.set(fsm);
        }
    };

    let result_html = match *resp {
        Ok(ref res) => {
            let (task, submissions) = res.clone();
            let onselect = {
                shadow_clone!(init_fsm);
                move |fsm| {
                    init_fsm.set(fsm);
                }
            };
            let run_local_test = {
                let script = task.script.clone();
                shadow_clone!(current_fsm, init_fsm, local_test_outcome);
                move |ev: MouseEvent| {
                    ev.prevent_default();
                    let fsm = (&*current_fsm).clone();
                    init_fsm.set(fsm.clone());
                    let tester = FSMTester::new(fsm, script.clone());
                    let mut tester = match tester {
                        Ok(t) => t,
                        Err(why) => {
                            local_test_outcome.set(html!(<span class="text-danger">{"BUG IN TASK (please report this!): "}{why}</span>));
                            return;
                        }
                    };
                    let mut seed = [0u8; 8];
                    for v in seed.iter_mut() {
                        *v = (randfloat() * 256.0) as u8;
                    }
                    let seed = i64::from_be_bytes(seed);
                    match tester.run_testing(seed) {
                        Err(why) => {
                            local_test_outcome.set(html!(<span class="text-danger">{"BUG IN TASK (please report this!): "}{why}</span>));
                            return;
                        }
                        Ok(res) => match res {
                            fsm::tester::FSMTestingOutput::Ok(t) => local_test_outcome.set(html!(<span class="text-success">{"OK: all "}{t}{" tests passed"}</span>)),
                            fsm::tester::FSMTestingOutput::WrongAnswer {
                                successes,
                                total_tests,
                                ..
                            } => local_test_outcome.set(html!(<span class="text-warning">{"WRONG: only "}{successes}{"/"}{total_tests}{" passed"}</span>)),
                            fsm::tester::FSMTestingOutput::FSMInvalid(why) => local_test_outcome.set(html!(<span class="text-warning">{"INVALID: "}{why}</span>)),
                        },
                    }
                }
            };
            html! {
                <>
                <h1>{task.name}</h1>
                <p>{task.legend}</p>
                <Row>
                    <Column>
                        <Canvas onchange={set_fsm} init={(&*init_fsm).clone()} />
                    </Column>
                    <Column>
                        <div class="btn-group" role="group">
                            <button type="button" class="btn btn-outline-primary" onclick={run_local_test}>{"Test locally"}</button>
                            <button type="button" class="btn btn-outline-success">{"Send and test on server"}</button>
                        </div>
                        <SubmissionList {submissions} {onselect} />
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
