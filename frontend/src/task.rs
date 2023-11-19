use api::TaskInfo;
use yew::{prelude::*, suspense::use_future};
use yew_bootstrap::component::Spinner;

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

#[function_component(TaskPageInner)]
fn task_page_inner(props: &TaskPageProps) -> HtmlResult {
    let TaskPageProps {
        group_slug,
        task_slug,
    } = props.clone();

    let resp = use_future(|| async move {
        reqwest::get(format!(
            "https://fsm-api.rudn-lab.ru/tasks/{group_slug}/{task_slug}"
        ))
        .await?
        .error_for_status()?
        .json::<TaskInfo>()
        .await
    })?;

    let result_html = match *resp {
        Ok(ref res) => {
            let res = res.clone();
            html! {
                <>
                <h1>{res.name}</h1>
                <p>{res.legend}</p>
                </>
            }
        }
        Err(ref failure) => {
            html!(<div class="alert alert-danger">{"Error while loading this task. Try reloading the page. Reason: "}{failure}</div>)
        }
    };

    Ok(result_html)
}
