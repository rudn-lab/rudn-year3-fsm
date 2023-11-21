use api::RegisterRequest;
use api::UserInfo;
use api::UserInfoResult;
use gloo::storage::Storage;
use shadow_clone::shadow_clone;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_bootstrap::component::form::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;
use yew_hooks::use_async;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;

use crate::Route;

#[function_component(Profile)]
pub fn profile() -> Html {
    let profile_token = gloo::storage::LocalStorage::get("token");
    let profile_token: Option<String> = match profile_token {
        Ok(key) => key,
        Err(_) => None,
    };

    if let Some(token) = profile_token {
        let fallback = html! {
            <h1>{"Загружаем информацию профиля..."}<Spinner /></h1>
        };
        html!(
            <Suspense {fallback}>
                <ProfileInner token={token} />
            </Suspense>

        )
    } else {
        html!(<Register />)
    }
}

#[function_component(ProfileInner)]
fn profile_inner(props: &ProfileNavInnerProps) -> HtmlResult {
    let navigator = use_navigator().unwrap();
    let ProfileNavInnerProps { token } = props;
    let token = token.clone();

    let resp = use_future(|| async move {
        reqwest::get(format!("https://fsm-api.rudn-lab.ru/user-info/{token}"))
            .await?
            .json::<UserInfoResult>()
            .await
    })?;

    let result_html = match *resp {
        Ok(ref res) => match res {
            UserInfoResult::Ok(UserInfo {
                name,
                rudn_id,
                token,
            }) => html! {
                <>
                    <h1>{name}</h1>
                    <h2>{"Студенческий билет RUDN: "}{rudn_id}</h2>

                    <p>{"Токен для других устройств: "}<code>{token}</code></p>
                </>
            },
            UserInfoResult::NoSuchToken => {
                navigator.push(&Route::Profile);
                gloo::storage::LocalStorage::delete("token");
                gloo::utils::document()
                    .location()
                    .unwrap()
                    .reload()
                    .unwrap();
                html!({ "Пользователь не существует" })
            }
        },
        Err(ref failure) => html!(<>{"Ошибка при загрузке профиля: "}{failure.to_string()}</>),
    };

    Ok(result_html)
}

#[function_component(ProfileNav)]
pub fn profile_nav() -> Html {
    let profile_key: Result<Option<String>, gloo::storage::errors::StorageError> =
        gloo::storage::LocalStorage::get("token");
    let profile_key: Option<String> = match profile_key {
        Ok(key) => key,
        Err(_) => None,
    };

    if let Some(key) = profile_key {
        let fallback = html! {
            <Link<Route> classes="nav-link" to={Route::Profile}>{"Загружаем пользователя..."}</Link<Route>>
        };
        html!(
            <div class="nav-item">
                <Suspense {fallback}>
                    <ProfileNavInner token={key} />
                </Suspense>
            </div>
        )
    } else {
        html!(
            <div class="nav-item">
                <Link<Route> classes="nav-link" to={Route::Profile}>{"Зарегестрируйся или войди сначала"}</Link<Route>>
            </div>
        )
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ProfileNavInnerProps {
    pub token: AttrValue,
}

#[function_component(ProfileNavInner)]
fn profile_nav_inner(props: &ProfileNavInnerProps) -> HtmlResult {
    let navigator = use_navigator().unwrap();
    let ProfileNavInnerProps { token } = props;
    let token = token.clone();

    let resp = use_future(|| async move {
        reqwest::get(format!("https://fsm-api.rudn-lab.ru/user-info/{token}"))
            .await?
            .json::<UserInfoResult>()
            .await
    })?;

    let result_html = match *resp {
        Ok(ref res) => match res {
            UserInfoResult::Ok(UserInfo { name, rudn_id, .. }) => {
                format!("Привет, {name} ({rudn_id})")
            }
            UserInfoResult::NoSuchToken => {
                navigator.push(&Route::Profile);
                gloo::storage::LocalStorage::delete("token");

                gloo::utils::document()
                    .location()
                    .unwrap()
                    .reload()
                    .unwrap();

                "Пользователь не существует".to_string()
            }
        },
        Err(ref failure) => failure.to_string(),
    };

    Ok(html!(<Link<Route> classes="nav-link" to={Route::Profile}>{result_html}</Link<Route>>))
}

#[function_component(Register)]
fn register() -> Html {
    html!(
        <div>
            <div class="alert alert-warning attention">
                {"Нужно зарегестрироваться или войти в систему, чтобы решать задачи"}
            </div>
            <Row>
                <Column>
                    <NewRegister />
                </Column>
                <Column>
                    <ExistingRegister />
                </Column>
            </Row>
        </div>
    )
}

#[function_component(NewRegister)]
fn new_register() -> Html {
    let navigator = use_navigator().unwrap();

    let name_state = use_state(|| String::new());
    let rudnid_state = use_state(|| String::new());
    let oninput_name = {
        shadow_clone!(name_state);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            name_state.set(target.value());
        }
    };
    let oninput_rudnid = {
        shadow_clone!(rudnid_state);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            rudnid_state.set(target.value());
        }
    };

    let token_result: yew_hooks::prelude::UseAsyncHandle<UserInfo, String> = use_async({
        shadow_clone!(rudnid_state, name_state);
        async move {
            let name = (*name_state).clone();
            let rudn_id = (*rudnid_state).clone();
            Ok(reqwest::Client::new()
                .post(format!("https://fsm-api.rudn-lab.ru/user-info"))
                .json(&RegisterRequest { name, rudn_id })
                .send()
                .await
                .map_err(|v| v.to_string())?
                .json::<UserInfo>()
                .await
                .map_err(|v| v.to_string())?)
        }
    });

    let start = {
        shadow_clone!(token_result);
        move |ev: MouseEvent| {
            ev.prevent_default();
            token_result.run();
        }
    };

    let validation = match &token_result.data {
        Some(_) => FormControlValidation::Valid(None),
        None => match &token_result.error {
            Some(why) => {
                FormControlValidation::Invalid(format!("Ошибка при регистрации: {why}").into())
            }
            None => FormControlValidation::None,
        },
    };

    if let Some(data) = &token_result.data {
        let UserInfo { token, .. } = data;
        gloo::storage::LocalStorage::set("token", token.clone()).unwrap();
        navigator.push(&Route::Home);
        gloo::utils::document()
            .location()
            .unwrap()
            .reload()
            .unwrap();
    }

    html!(
        <>
            <h1>{"Создать новый аккаунт"}</h1>
            <form>
                <FormControl id="name" ctype={FormControlType::Text} class="mb-3" label="Имя студента" value={(*name_state).clone()} oninput={oninput_name} validation={validation.clone()}/>
                <FormControl id="rudnid" ctype={FormControlType::Number{min: None, max: None}} class="mb-3" label="Номер студенческого билета RUDN" value={(*rudnid_state).clone()} oninput={oninput_rudnid} {validation}/>

                <button type="submit" class="btn btn-primary" disabled={token_result.loading} onclick={start}>
                    if token_result.loading {
                        <Spinner small={true}  />
                    }
                    {"Зарегистрироваться"}
                </button>
            </form>
        </>
    )
}

#[function_component(ExistingRegister)]
fn existing_register() -> Html {
    let navigator = use_navigator().unwrap();
    let token_state = use_state(|| String::new());
    let oninput_token = {
        shadow_clone!(token_state);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            token_state.set(target.value());
        }
    };
    let token_result: yew_hooks::prelude::UseAsyncHandle<UserInfoResult, String> = use_async({
        shadow_clone!(token_state);
        async move {
            let token = (*token_state).clone();
            Ok(
                reqwest::get(format!("https://fsm-api.rudn-lab.ru/user-info/{token}"))
                    .await
                    .map_err(|v| v.to_string())?
                    .json::<UserInfoResult>()
                    .await
                    .map_err(|v| v.to_string())?,
            )
        }
    });

    let validation = match &token_result.data {
        Some(data) => match data {
            UserInfoResult::Ok(_) => FormControlValidation::Valid(None),
            UserInfoResult::NoSuchToken => {
                FormControlValidation::Invalid("Этот токен не найден".into())
            }
        },
        None => match &token_result.error {
            Some(why) => {
                FormControlValidation::Invalid(format!("Ошибка при проверке токена: {why}").into())
            }
            None => FormControlValidation::None,
        },
    };

    let start = {
        shadow_clone!(token_result);
        move |_ev| {
            token_result.run();
        }
    };

    if let Some(data) = &token_result.data {
        if let UserInfoResult::Ok(UserInfo { token, .. }) = data {
            gloo::storage::LocalStorage::set("token", token.clone()).unwrap();
            navigator.push(&Route::Home);
            gloo::utils::document()
                .location()
                .unwrap()
                .reload()
                .unwrap();
        }
    }

    html!(
        <>
            <h1>{"Использовать существующий токен"}</h1>

            <FormControl id="token" ctype={FormControlType::Text} class="mb-3" label="Токен аккаунта" oninput={oninput_token} value={(*token_state).clone()} {validation}/>


            <Button style={Color::Primary} disabled={&token_result.loading} onclick={start}>
                if token_result.loading {
                    <Spinner small={true}  />
                }
                {"Войти с токеном"}
            </Button>
        </>
    )
}
