use std::collections::HashSet;

use api::TaskGroupLeaderboard;
use shadow_clone::shadow_clone;
use yew::{prelude::*, suspense::use_future};
use yew_bootstrap::component::Spinner;
use yew_hooks::use_interval;

use crate::task::{prepare_popovers, unix_time_to_locale_string, VerdictDisplay};

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

    prepare_popovers();

    let LeaderboardProps { group_slug } = props.clone();

    let resp = {
        shadow_clone!(group_slug);
        use_future(|| async move {
            reqwest::get(format!(
                "https://fsm-api.rudn-lab.ru/tasks/{group_slug}/leaderboard",
            ))
            .await?
            .error_for_status()?
            .json::<TaskGroupLeaderboard>()
            .await
        })?
    };

    let res_html = match *resp {
        Err(ref failure) => {
            html!(<div class="alert alert-danger">{"Ошибка при загрузке таблицы результатов. Перезагрузите страницу. Причина: "}{failure}</div>)
        }
        Ok(ref leaderboard) => {
            let mut user_set = HashSet::new();
            for task in leaderboard.tasks.iter() {
                for submission in task.latest_submissions.iter() {
                    user_set.insert(submission.0.clone());
                }
            }
            let mut user_set: Vec<_> = user_set.into_iter().collect();
            user_set.sort_by(|a, b| a.name.cmp(&b.name));

            let task_heads: Html = leaderboard
                .tasks
                .iter()
                .map(|t| {
                    html!(
                        <th scope="col">
                            {&t.name}
                        </th>
                    )
                })
                .collect();
            let task_stats: Html = user_set.iter().map(|user| {

                let task_items: Html = leaderboard.tasks.iter().map(|task| {
                    if let Some(my_submission) = task.latest_submissions.iter().find(|v| &v.0 == user) {
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

                html!(<tr><th scope="row">{&user.name}{" ("}{&user.rudn_id}{")"}</th>{task_items}</tr>)
            }).collect();

            html!(
                <>
                    <h1>{&leaderboard.name}</h1>
                    <p>{&leaderboard.legend}</p>
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
