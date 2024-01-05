use api::{TaskGroupLeaderboard, UserAndSubmissionStats};
use shadow_clone::shadow_clone;
use yew::{prelude::*, suspense::use_future};
use yew_bootstrap::component::Spinner;
use yew_hooks::use_interval;
use yew_router::components::Link;

use crate::{
    task::{prepare_popovers, unix_time_to_locale_string, VerdictDisplay},
    Route,
};

#[derive(Properties, PartialEq, Clone)]
pub struct LeaderboardProps {
    pub group_slug: AttrValue,
}

#[function_component(Leaderboard)]
pub fn leaderboard(props: &LeaderboardProps) -> Html {
    let LeaderboardProps { group_slug } = props;
    let fallback = html! {
        <h1>{"Загружаем таблицу результатов..."}<Spinner/></h1>
    };
    html!(
        <Suspense {fallback}>
            <LeaderboardInner {group_slug} />
        </Suspense>
    )
}

#[function_component(LeaderboardInner)]
pub fn leaderboard_inner(props: &LeaderboardProps) -> HtmlResult {
    let force = use_force_update();
    use_interval(move || force.force_update(), 1000);

    #[derive(PartialEq, Clone, Copy)]
    enum SortingMode {
        AlphabeticalByName,
        AlphabeticalByRudnId,
        ByTotalSubmissions,
        ByOkSubmissions,
        ByTotalTasks,
        ByOkTasks,
        BySubmissionDate,
        ByNodeCount,
        ByLinkCount,
    }

    impl SortingMode {
        fn is_for_task(&self) -> bool {
            match self {
                Self::BySubmissionDate | Self::ByLinkCount | Self::ByNodeCount => true,
                _ => false,
            }
        }
    }

    let selected_mode = use_state(|| SortingMode::AlphabeticalByName);
    let selected_task = use_state(|| 0usize);

    prepare_popovers();

    let LeaderboardProps { group_slug } = props.clone();

    let resp = {
        shadow_clone!(group_slug);
        use_future(|| async move {
            let leaderboard = reqwest::get(format!(
                "https://fsm-api.rudn-lab.ru/tasks/{group_slug}/leaderboard",
            ))
            .await?
            .error_for_status()?
            .json::<TaskGroupLeaderboard>()
            .await?;
            let users = reqwest::get(format!("https://fsm-api.rudn-lab.ru/users",))
                .await?
                .error_for_status()?
                .json::<Vec<UserAndSubmissionStats>>()
                .await?;
            Ok::<_, reqwest::Error>((users, leaderboard))
        })?
    };

    let res_html = match *resp {
        Err(ref failure) => {
            html!(<div class="alert alert-danger">{"Ошибка при загрузке таблицы результатов. Перезагрузите страницу. Причина: "}{failure}</div>)
        }
        Ok((ref users, ref leaderboard)) => {
            let mut user_set = users.clone();

            match *selected_mode {
                SortingMode::AlphabeticalByName => {
                    user_set.sort_by(|a, b| a.user.name.cmp(&b.user.name))
                }
                SortingMode::AlphabeticalByRudnId => {
                    user_set.sort_by(|a, b| a.user.rudn_id.cmp(&b.user.rudn_id))
                }
                SortingMode::ByTotalSubmissions => {
                    user_set.sort_by(|a, b| b.total_submissions.cmp(&a.total_submissions));
                }
                SortingMode::ByOkSubmissions => {
                    user_set.sort_by(|a, b| b.ok_submissions.cmp(&a.ok_submissions));
                }
                SortingMode::ByTotalTasks => {
                    user_set.sort_by(|a, b| b.attempted_tasks.cmp(&a.attempted_tasks));
                }
                SortingMode::ByOkTasks => {
                    user_set.sort_by(|a, b| b.ok_tasks.cmp(&a.ok_tasks));
                }
                SortingMode::BySubmissionDate => {
                    let mut task_num = *selected_task;
                    if task_num >= leaderboard.tasks.len() {
                        task_num = 0;
                    }
                    user_set.sort_by(|a, b| {
                        let user_a_result = leaderboard.tasks[task_num]
                            .latest_submissions
                            .iter()
                            .find(|v| v.0 == a.user);
                        let user_b_result = leaderboard.tasks[task_num]
                            .latest_submissions
                            .iter()
                            .find(|v| v.0 == b.user);
                        match (user_a_result, user_b_result) {
                            (None, None) => std::cmp::Ordering::Equal,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (Some(a), Some(b)) => a.1.cmp(&b.1),
                        }
                    });
                }

                SortingMode::ByNodeCount | SortingMode::ByLinkCount => {
                    let mut task_num = *selected_task;
                    if task_num >= leaderboard.tasks.len() {
                        task_num = 0;
                    }
                    user_set.sort_by(|a, b| {
                        let user_a_result = leaderboard.tasks[task_num]
                            .latest_submissions
                            .iter()
                            .find(|v| v.0 == a.user);
                        let user_b_result = leaderboard.tasks[task_num]
                            .latest_submissions
                            .iter()
                            .find(|v| v.0 == b.user);
                        match (user_a_result, user_b_result) {
                            (None, None) => std::cmp::Ordering::Equal,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (Some(a), Some(b)) => {
                                if matches!(*selected_mode, SortingMode::ByNodeCount) {
                                    a.3.cmp(&b.3)
                                } else {
                                    a.4.cmp(&b.4)
                                }
                            }
                        }
                    });
                }
            }

            let task_heads: Html = leaderboard
                .tasks
                .iter()
                .map(|t| {
                    html!(
                        <th scope="col">
                            <p>{&t.name}</p>
                        </th>
                    )
                })
                .collect();
            let task_stats: Html = user_set.iter().map(|user| {

                let task_items: Html = leaderboard.tasks.iter().map(|task| {
                    if let Some(my_submission) = task.latest_submissions.iter().find(|v| v.0 == user.user) {
                        html!(<td><VerdictDisplay verdict={my_submission.5.clone()} /><br />
                            {unix_time_to_locale_string(my_submission.1 as f64)}<br />{" ("}
                            {my_submission.3} // nodes
                            {" кружочков, "}
                            {my_submission.4} // links
                            {" стрелочек)"}
                            </td>)
                    } else {
                        html!(<td class="fs-1">{"∅"}</td>)
                    }
                }).collect();

                html!(<tr>
                        <th scope="row">
                            <p>
                                <Link<Route> classes="" to={Route::User{user_id: format!("{}", user.user.id).into()}}>{&user.user.name}{" ("}{&user.user.rudn_id}{")"}</Link<Route>>
                            </p>

                            <p class="fw-normal">{"Посылок: "}{user.ok_submissions}{" ОК / "}{user.total_submissions}{" всего"}</p>
                            <p class="fw-normal">{"Задач: "}{user.attempted_tasks}{" ОК / "}{user.ok_tasks}{" приступил"}</p>
                        </th>
                        {task_items}
                    </tr>)
            }).collect();

            let sorting_options = {
                let on_outer_select = {
                    shadow_clone!(selected_mode);
                    Callback::from(move |ev: InputEvent| {
                        let el: web_sys::HtmlSelectElement = ev.target_dyn_into().unwrap();
                        match el.value().as_str() {
                            "alpha-name" => {
                                selected_mode.set(SortingMode::AlphabeticalByName);
                            }
                            "alpha-rudnid" => {
                                selected_mode.set(SortingMode::AlphabeticalByRudnId);
                            }
                            "total-submissions" => {
                                selected_mode.set(SortingMode::ByTotalSubmissions);
                            }
                            "ok-submissions" => {
                                selected_mode.set(SortingMode::ByOkSubmissions);
                            }
                            "total-tasks" => {
                                selected_mode.set(SortingMode::ByTotalTasks);
                            }
                            "ok-tasks" => {
                                selected_mode.set(SortingMode::ByOkTasks);
                            }
                            "submission-date" => {
                                selected_mode.set(SortingMode::BySubmissionDate);
                            }
                            "node-count" => {
                                selected_mode.set(SortingMode::ByNodeCount);
                            }
                            "link-count" => {
                                selected_mode.set(SortingMode::ByLinkCount);
                            }

                            what => {
                                log::error!("Unexpected value for outer select: {what:?}");
                            }
                        }
                    })
                };
                let outer_select = html!(
                    <select oninput={on_outer_select} class="form-select">
                        <option value="alpha-name" selected={matches!(*selected_mode, SortingMode::AlphabeticalByName)}>{"По алфавиту по имени"}</option>
                        <option value="alpha-rudnid" selected={matches!(*selected_mode, SortingMode::AlphabeticalByRudnId)}>{"По алфавиту по номеру билета"}</option>
                        <option value="total-submissions" selected={matches!(*selected_mode, SortingMode::ByTotalSubmissions)}>{"По общему количеству посылок от пользователя"}</option>
                        <option value="ok-submissions" selected={matches!(*selected_mode, SortingMode::ByOkSubmissions)}>{"По количеству успешных посылок от пользователя"}</option>
                        <option value="total-tasks" selected={matches!(*selected_mode, SortingMode::ByTotalTasks)}>{"По общему количеству задач, к которым пользователь приступил"}</option>
                        <option value="ok-tasks" selected={matches!(*selected_mode, SortingMode::ByOkTasks)}>{"По количеству успешно решенных задач от пользователя"}</option>
                        <option value="submission-date" selected={matches!(*selected_mode, SortingMode::BySubmissionDate)}>{"По дате посылки..."}</option>
                        <option value="node-count" selected={matches!(*selected_mode, SortingMode::ByNodeCount)}>{"По количеству кружочков..."}</option>
                        <option value="link-count" selected={matches!(*selected_mode, SortingMode::ByLinkCount)}>{"По количеству стрелочек..."}</option>
                    </select>
                );
                let mut inner_select_values = vec![];
                for (id, task) in leaderboard.tasks.iter().enumerate() {
                    inner_select_values.push(
                        html!(
                            <option value={format!("{id}")} selected={*selected_task == id}>{id+1}{". "}{&task.name}</option>
                        )
                    );
                }
                let inner_select = if selected_mode.is_for_task() {
                    let on_inner_select = {
                        shadow_clone!(selected_task);
                        Callback::from(move |ev: InputEvent| {
                            let el: web_sys::HtmlSelectElement = ev.target_dyn_into().unwrap();
                            let num: usize = el.value().parse().unwrap_or(0);
                            selected_task.set(num);
                        })
                    };
                    html!(
                        <>
                            <i>{" для задачи: "}</i>
                            <select oninput={on_inner_select} class="form-select">
                                {for inner_select_values}
                            </select>
                        </>
                    )
                } else {
                    html!()
                };

                html!(
                    <>
                    {outer_select}
                    {inner_select}
                    </>
                )
            };

            html!(
                <>
                    <h1>{&leaderboard.name}</h1>
                    <p>{&leaderboard.legend}</p>
                    <p><i>{"Сортировать: "}</i>{sorting_options}</p>
                    <table class="table">
                        <thead>
                            <tr>
                                <th></th>
                                {task_heads}
                            </tr>
                        </thead>
                        <tbody>
                            {task_stats}
                        </tbody>
                    </table>
                </>
            )
        }
    };

    Ok(res_html)
}
