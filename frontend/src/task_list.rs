use api::TaskGroupInfo;
use yew::{prelude::*, suspense::use_future};
use yew_autoprops::autoprops_component;
use yew_bootstrap::component::Spinner;
use yew_router::components::Link;

use crate::Route;

#[function_component(HomeTaskList)]
pub fn home_task_list() -> Html {
    let fallback = html! {
        <h1>{"Loading tasks..."}<Spinner/></h1>
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
                    <h1>{res.len()} {" task groups"}</h1>
                    <div class="row row-cols-3">
                        {task_grps}
                    </div>
                </>
            }
        }
        Err(ref failure) => {
            html!(<div class="alert alert-danger">{"Error while loading tasks. Try reloading the page. Reason: "}{failure}</div>)
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
                </Link<Route>>
            </li>
        )
    });
    html! {
        <div class="col">
            <div class="card">
                <div class="card-body">
                    <h5 class="card-title">{&grp.name}</h5>
                    <ul class="list-group list-group-flush">
                        {items.collect::<Html>()}
                    </ul>
                </div>
            </div>
        </div>
    }
}
