mod canvas;
mod editor;
mod profile;
mod task;
mod task_list;

use gloo::storage::Storage;
use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::icons::*;
use yew_router::prelude::Link;
use yew_router::prelude::*;

use crate::editor::Editor;
use crate::profile::Profile;
use crate::profile::ProfileNav;
use crate::task::TaskPage;
use crate::task_list::HomeTaskList;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/profile")]
    Profile,

    #[at("/editor")]
    Editor,

    #[at("/task/:group_slug/:task_slug")]
    Task {
        group_slug: AttrValue,
        task_slug: AttrValue,
    },

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn Home() -> Html {
    let navigator = use_navigator().unwrap();

    let profile_key = gloo::storage::LocalStorage::get("token");
    let profile_key: Option<String> = match profile_key {
        Ok(key) => key,
        Err(_) => None,
    };
    if profile_key.is_none() {
        navigator.push(&Route::Profile);
    }

    html! {
        <HomeTaskList />
    }
}

#[function_component]
fn App() -> Html {
    fn switch(route: Route) -> Html {
        match route {
            Route::Home => html!(<Home/>),
            Route::Profile => html!(<Profile />),
            Route::Editor => html!(<Editor />),
            Route::Task {
                group_slug,
                task_slug,
            } => html!(<TaskPage {group_slug} {task_slug}/>),
            Route::NotFound => html!("route not found"),
        }
    }

    html! {
        <BrowserRouter>
            {BIFiles::cdn()}
            <nav class="navbar bg-body-tertiary">
                <div class="container-fluid">
                    <Link<Route> classes="navbar-brand" to={Route::Home}>{"RUDN FSM"}</Link<Route>>
                    <div class="nav-item">
                        <Link<Route> classes="nav-link" to={Route::Editor}>{"Editor"}</Link<Route>>
                    </div>
                    <ProfileNav />
                </div>
            </nav>
            <Container>
                <Switch<Route> render={switch} />
            </Container>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
