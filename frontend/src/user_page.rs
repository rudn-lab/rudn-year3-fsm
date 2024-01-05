use api::{OthersSubmissionInfo, TaskInfo, UserAndSubmissions};
use gloo::storage::Storage;
use yew::{prelude::*, suspense::use_future_with};
use yew_bootstrap::{component::Spinner, icons::BI};
use yew_hooks::use_interval;
use yew_router::components::Link;

use crate::{
    task::{prepare_popovers, unix_time_to_locale_string, VerdictDisplay},
    Route,
};

#[derive(PartialEq, Clone, Properties)]
pub struct UserPageProps {
    pub user_id: AttrValue,
}

#[function_component(User)]
pub fn user_page(props: &UserPageProps) -> Html {
    let user_id = &props.user_id;
    let fallback =
        html!(<h1>{"Загружаю информацию о пользователе "}{user_id}{"..."}<Spinner/></h1>);
    html!(
        <Suspense {fallback}>
            <UserPageInner {user_id} />
        </Suspense>
    )
}

#[function_component(UserPageInner)]
pub fn user_page_inner(props: &UserPageProps) -> HtmlResult {
    let force = use_force_update();
    use_interval(move || force.force_update(), 1000);

    let resp = {
        use_future_with(props.user_id.clone(), |user_id| async move {
            let user_info = reqwest::get(format!("https://fsm-api.rudn-lab.ru/users/{user_id}",))
                .await?
                .error_for_status()?
                .json::<UserAndSubmissions>()
                .await?;
            Ok::<_, reqwest::Error>(user_info)
        })?
    };
    prepare_popovers();

    Ok(match *resp {
        Err(ref failure) => {
            html!(<div class="alert alert-danger">{"Ошибка при загрузке информации о пользователе. Перезагрузите страницу. Причина: "}{failure}</div>)
        }
        Ok(ref info) => match info {
            UserAndSubmissions::UserNotFound => {
                html!(<div class="alert alert-warning">{"Пользователь с ID "}{props.user_id.clone()}{" не существует."}</div>)
            }
            UserAndSubmissions::Present { user, submissions } => {
                let submission_table = {
                    let rows = submissions.iter().map(|v| {
                        html!(
                            <tr>
                                <td>{v.id}</td>
                                <td><TaskLink task_id={v.task_id} /></td>
                                <td>{unix_time_to_locale_string(v.when_unix_time as f64)}</td>
                                <td>{v.node_count}{" кружочков, "}{v.link_count}{" стрелочек"}</td>
                                <td><VerdictDisplay verdict={v.verdict.clone()} /></td>
                                <td><SubmissionLink id={v.id} /></td>
                            </tr>
                        )
                    });

                    html!(
                        <table class="table">
                            <thead>
                                <tr>
                                    <th scope="col">{"ID"}</th>
                                    <th scope="col">{"Задача"}</th>
                                    <th scope="col">{"Дата"}</th>
                                    <th scope="col">{"Статистика"}</th>
                                    <th scope="col">{"Вердикт"}</th>
                                    <th scope="col">{"Посмотреть посылку"}</th>

                                </tr>
                            </thead>
                            <tbody>
                                {for rows}
                            </tbody>
                        </table>
                    )
                };

                html!(
                    <>
                        <h1>{&user.name}{" ("}{&user.rudn_id}{")"}</h1>
                        <p>{"Всего посылок: "}{submissions.len()}</p>
                        {submission_table}
                    </>
                )
            }
        },
    })
}

#[derive(PartialEq, Clone, Properties)]
pub struct TaskLinkProps {
    pub task_id: i64,
}

#[function_component(TaskLink)]
pub fn task_link(props: &TaskLinkProps) -> Html {
    let fallback = html!(
        <Link<Route> classes="" to={Route::TaskById{task_id: props.task_id}}>{"Задание с ID="}{props.task_id}<Spinner small={true} /></Link<Route>>
    );
    let task_id = props.task_id;
    html!(
        <Suspense {fallback}>
            <TaskLinkInner {task_id} />
        </Suspense>
    )
}

#[function_component(TaskLinkInner)]
fn task_link_inner(props: &TaskLinkProps) -> HtmlResult {
    let resp = {
        use_future_with(props.task_id, |task_id| async move {
            let user_info =
                reqwest::get(format!("https://fsm-api.rudn-lab.ru/task-by-id/{task_id}",))
                    .await?
                    .error_for_status()?
                    .json::<TaskInfo>()
                    .await?;
            Ok::<_, reqwest::Error>(user_info)
        })?
    };
    Ok(match *resp {
        Err(ref _failure) => {
            html!(<Link<Route> classes="" to={Route::TaskById{task_id: props.task_id}}>{"Задание с ID="}{props.task_id} <span class="d-inline-block text-warning fs-5">{BI::EXCLAMATION_TRIANGLE_FILL}</span></Link<Route>>)
        }
        Ok(ref info) => html!(
            <Link<Route> classes="" to={Route::Task { group_slug: "_".into(), task_slug: info.slug.clone().into() }}>{&info.name}</Link<Route>>
        ),
    })
}

#[derive(PartialEq, Clone, Properties)]
pub struct SubmissionLinkProps {
    pub id: i64,
}

#[function_component(SubmissionLink)]
fn submission_link(props: &SubmissionLinkProps) -> Html {
    let fallback = html!(
        <Link<Route> classes="btn btn-outline-primary" to={Route::Submission{ sid: props.id }}>{BI::EYE_FILL}</Link<Route>>
    );
    let id = props.id;
    html!(
        <Suspense {fallback}>
            <SubmissionLinkInner {id} />
        </Suspense>
    )
}

#[function_component(SubmissionLinkInner)]
fn submission_link_inner(props: &SubmissionLinkProps) -> HtmlResult {
    let token: String = gloo::storage::LocalStorage::get::<Option<String>>("token")
        .unwrap_or(None)
        .unwrap_or("guest".to_string());
    let resp = {
        use_future_with(props.id, |id| async move {
            let submission_info = reqwest::get(format!(
                "https://fsm-api.rudn-lab.ru/submissions/{id}/{token}",
            ))
            .await?
            .error_for_status()?
            .json::<Option<OthersSubmissionInfo>>()
            .await?;
            Ok::<_, reqwest::Error>(submission_info)
        })?
    };
    Ok(match *resp {
        Err(ref _failure) => {
            html!(<Link<Route> classes="btn btn-outline-warning" to={Route::Submission{ sid: props.id }}>{BI::EYE_FILL}{BI::QUESTION_DIAMOND}</Link<Route>>)
        }
        Ok(ref info) => match info {
            None => {
                html!(<Link<Route> classes="btn btn-outline-danger" to={Route::Submission{ sid: props.id }}>{BI::X_OCTAGON_FILL}</Link<Route>>)
            }
            Some(info) => match info.details {
                api::OthersSubmissionDetails::GuestAccess
                | api::OthersSubmissionDetails::SolveThisFirst => {
                    html!(<Link<Route> classes="btn btn-outline-warning" to={Route::Submission{ sid: props.id }}>{BI::EYE_SLASH_FILL}</Link<Route>>)
                }
                api::OthersSubmissionDetails::Ok(_) => {
                    html!(<Link<Route> classes="btn btn-outline-success" to={Route::Submission{ sid: props.id }}>{BI::EYE_FILL}</Link<Route>>)
                }
            },
        },
    })
}
