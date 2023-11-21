use api::TaskGroupInfo;
use shadow_clone::shadow_clone;
use yew::{prelude::*, suspense::use_future};
use yew_autoprops::autoprops_component;
use yew_bootstrap::{component::Spinner, icons::BI};
use yew_hooks::use_local_storage;
use yew_router::components::Link;

use crate::Route;

#[function_component(HomeTaskList)]
pub fn home_task_list() -> Html {
    let fallback = html! {
        <h1>{"Загружаем задания..."}<Spinner/></h1>
    };
    html!(
        <Suspense {fallback}>
            <HomeTaskListInner />
        </Suspense>
    )
}

#[function_component(HomeTaskListInner)]
fn profile_nav_inner() -> HtmlResult {
    let resp = use_future(|| async move {
        reqwest::get("https://fsm-api.rudn-lab.ru/tasks")
            .await?
            .error_for_status()?
            .json::<Vec<TaskGroupInfo>>()
            .await
    })?;

    let result_html = match *resp {
        Ok(ref res) => {
            let task_grps = res
                .iter()
                .map(|v| html!(<TaskGroupDisplay grp={v.clone()} />))
                .collect::<Html>();
            html! {
                <>
                    <h1>{"Количество групп задач: "}{res.len()}</h1>
                    <div class="row row-cols-3">
                        {task_grps}
                    </div>
                </>
            }
        }
        Err(ref failure) => {
            html!(<div class="alert alert-danger">{"Ошибка при загрузке задач. Перезагрузите страницу. Причина: "}{failure}</div>)
        }
    };

    Ok(result_html)
}

#[autoprops_component(TaskGroupDisplay)]
fn task_group_display(grp: &TaskGroupInfo) -> Html {
    let items = grp.tasks.iter().map(|v| {
        html!(
            <li class="list-group-item">
                <Link<Route> classes="card-link" to={Route::Task { group_slug: grp.slug.clone().into(), task_slug: v.slug.clone().into() }}>
                    {v.name.clone()}
                    <TaskStatusDisplay group_slug={grp.slug.clone()} task_slug={v.slug.clone()} />
                </Link<Route>>
            </li>
        )
    });
    html! {
        <div class="col">
            <div class="card">
                <div class="card-body">
                    <h5 class="card-title">{&grp.name}</h5>
                    <p class="card-text">
                    {&grp.legend}
                    <Link<Route> to={Route::Leaderboard { group_slug: grp.slug.clone().into() }}>{" (посмотреть таблицу результатов)"}</Link<Route>>

                    </p>
                    <ul class="list-group list-group-flush">
                        {items.collect::<Html>()}
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct TaskStatusDisplayProps {
    pub group_slug: AttrValue,
    pub task_slug: AttrValue,
}

#[function_component(TaskStatusDisplay)]
fn task_status_display(props: &TaskStatusDisplayProps) -> Html {
    let TaskStatusDisplayProps {
        group_slug,
        task_slug,
    } = props;

    let fallback = html!(<Spinner small={true} />);

    html!(
        <Suspense {fallback}>
            <TaskStatusDisplayInner {group_slug} {task_slug} />
        </Suspense>

    )
}

#[function_component(TaskStatusDisplayInner)]
fn task_status_display_inner(props: &TaskStatusDisplayProps) -> HtmlResult {
    let TaskStatusDisplayProps {
        group_slug,
        task_slug,
    } = props;

    let token = use_local_storage::<String>("token".to_string());

    let resp = {
        shadow_clone!(token, group_slug, task_slug);
        use_future(|| async move {
            let token_value = (&*token).clone();
            if token_value.is_none() {
                gloo::utils::document()
                    .location()
                    .unwrap()
                    .reload()
                    .unwrap();
            }
            let token_value = token_value.unwrap_or_default();
            reqwest::get(format!(
                "https://fsm-api.rudn-lab.ru/tasks/{group_slug}/{task_slug}/{token_value}/success"
            ))
            .await?
            .error_for_status()?
            .json::<bool>()
            .await
        })?
    };

    Ok(match &*resp {
        Ok(v) => match v {
            true => html!(<span class="fs-5 text-success">{BI::CHECK_CIRCLE_FILL}</span>),
            false => html!(<span class="fs-5 text-warning">{BI::QUESTION_DIAMOND_FILL}</span>),
        },
        Err(_why) => {
            html!(<span class="fs-5 text-danger">{BI::EXCLAMATION_TRIANGLE_FILL}</span>)
        }
    })
}
