mod canvas;
mod profile;

use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::icons::*;
use yew_router::prelude::*;

use crate::canvas::Canvas;
use crate::profile::Profile;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/profile")]
    Profile,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn Home() -> Html {
    html! {
        <Canvas />
    }
}

#[function_component]
fn App() -> Html {
    fn switch(route: Route) -> Html {
        match route {
            Route::Home => html!(<Home/>),
            Route::Profile => html!(<Profile />),
            Route::NotFound => todo!(),
        }
    }

    let brand = BrandType::BrandSimple {
        text: AttrValue::from("FSM Editor"),
        url: Some(AttrValue::from("https://fsm.rudn-lab.ru")),
    };

    html! {
        <>
            {BIFiles::cdn()}
            <NavBar class="navbar-expand-lg" brand={brand}>
            </NavBar>
            <Container>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </Container>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
