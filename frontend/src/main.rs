mod canvas;
mod canvas_player;
mod editor;
mod leaderboard;
mod profile;
mod scripter;
mod task;
mod task_list;
mod tutorial;

use canvas::Canvas;
use gloo::storage::Storage;
use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::icons::*;
use yew_router::prelude::Link;
use yew_router::prelude::*;

use crate::editor::Editor;
use crate::leaderboard::Leaderboard;
use crate::profile::Profile;
use crate::profile::ProfileNav;
use crate::scripter::Scripter;
use crate::task::TaskPage;
use crate::task_list::HomeTaskList;
use crate::tutorial::Tutorial;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/profile")]
    Profile,

    #[at("/editor")]
    Editor,

    #[at("/scripter")]
    Scripter,

    #[at("/tutorial")]
    Tutorial,

    #[at("/leaderboard/:group_slug")]
    Leaderboard { group_slug: AttrValue },

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
            Route::Scripter => html!(<Scripter />),
            Route::Leaderboard { group_slug } => html!(<Leaderboard {group_slug} />),
            Route::Task {
                group_slug,
                task_slug,
            } => html!(<TaskPage {group_slug} {task_slug}/>),
            Route::Tutorial => html!(<Tutorial />),
            Route::NotFound => html!("404"),
        }
    }

    html! {
        <BrowserRouter>
            {BIFiles::cdn()}
            <nav class="navbar bg-body-tertiary">
                <div class="container-fluid">
                    <Link<Route> classes="navbar-brand" to={Route::Home}>{"RUDN FSM"}</Link<Route>>

                    <div class="nav-item">
                        <Link<Route> classes="nav-link" to={Route::Tutorial}>{"Инструкция: Введение в автоматы"}</Link<Route>>
                    </div>

                    <div class="nav-item">
                        <Link<Route> classes="nav-link" to={Route::Editor}>{"Редактор автоматов"}</Link<Route>>
                    </div>
                    <div class="nav-item">
                        <Link<Route> classes="nav-link" to={Route::Scripter}>{"Отладка заданий"}</Link<Route>>
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

// #[function_component]
// fn App() -> Html {
//     html! {
//         <>
//         {BIFiles::cdn()}
//         <Tutorial />
//         </>
//     }
// }

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
