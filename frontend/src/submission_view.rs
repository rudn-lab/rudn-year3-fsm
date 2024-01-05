use crate::{canvas_player::CanvasPlayer, user_page::TaskLink, Route};
use api::OthersSubmissionInfo;
use fsm::fsm::StateMachine;
use gloo::storage::Storage;
use shadow_clone::shadow_clone;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*, suspense::use_future_with};
use yew_autoprops::autoprops_component;
use yew_bootstrap::component::Spinner;
use yew_router::components::Link;

#[derive(PartialEq, Clone, Properties)]
pub struct ViewSubmissionProps {
    pub id: i64,
}

#[function_component(Submission)]
pub fn view_submission(props: &ViewSubmissionProps) -> Html {
    let ViewSubmissionProps { id } = props;
    let fallback = html! {
        <h1>{"Загружаем посылку "}{id}{"..."}<Spinner/></h1>
    };
    html!(
        <Suspense {fallback}>
            <ViewSubmissionInner {id} />
        </Suspense>
    )
}

#[function_component(ViewSubmissionInner)]
pub fn view_submission_inner(props: &ViewSubmissionProps) -> HtmlResult {
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
        Err(ref failure) => {
            html!(<div class="alert alert-danger">{"Ошибка при загрузке посылки "}{props.id}{". Перезагрузите страницу. Причина: "}{failure}</div>)
        }
        Ok(ref info) => match info {
            None => {
                html!(<div class="alert alert-warning">{"Посылка с номером "}{props.id}{" не существует. "}</div>)
            }
            Some(info) => {
                let deets = match &info.details {
                    api::OthersSubmissionDetails::GuestAccess => {
                        html!(<div class="alert alert-info">
                        {"Вы не можете посмотреть содержимое посылки без аккаунта; "}
                        <Link<Route> classes="alert-link" to={Route::Profile}>{"зарегестрируйтесь?"}</Link<Route>>
                        </div>)
                    }
                    api::OthersSubmissionDetails::SolveThisFirst => {
                        html!(<div class="alert alert-info">
                        {"Вы не можете посмотреть содержимое посылки, если у вас нет верного решения этой же задачи; "}
                        <Link<Route> classes="alert-link" to={Route::TaskById{task_id: info.task_id}}>{"попробуйте решить ее?"}</Link<Route>>
                        </div>)
                    }
                    api::OthersSubmissionDetails::Ok(fsm) => {
                        html!(<ViewFSM state_machine={fsm.clone()} />)
                    }
                };

                let verdict_line = match &info.verdict {
                    api::SubmissionVerdict::Ok(how_many) => {
                        html!(<span class="text-success">{"Все "}{how_many}{" тестов проходят"}</span>)
                    }
                    api::SubmissionVerdict::WrongAnswer {
                        total_tests,
                        successes,
                        ..
                    } => {
                        html!(<span class="text-warning">{"НЕВЕРНО: только "}{successes}{"/"}{total_tests}{" тестов проходят"}</span>)
                    }
                    api::SubmissionVerdict::InvalidFSM(why) => match why {
                        fsm::fsm::FSMError::InfiniteLoop => {
                            html!(<span class="text-danger">{"Автомат нельзя тестировать, потому что он содержит бесконечный цикл"}</span>)
                        }
                        fsm::fsm::FSMError::NoEntryLinks => {
                            html!(<span class="text-danger">{"Автомат нельзя тестировать, потому что он не содержит входных стрелочек"}</span>)
                        }
                        fsm::fsm::FSMError::DisjointedLink(_) => {
                            html!(<span class="text-danger">{"Автомат нельзя тестировать, потому что есть несвязность между кружочками и стрелочками"}</span>)
                        }
                    },
                    api::SubmissionVerdict::TaskInternalError(why) => {
                        html!(<span class="text-danger">{"Внутренняя ошибка задания: "}{why}</span>)
                    }
                };

                html!(<>
                    <h1>{"Посылка "}{props.id}</h1>
                    <p>{"Отправил пользователь: "}
                        <Link<Route> classes="" to={Route::User{user_id: format!("{}", info.submitting_user.id).into()}}>{&info.submitting_user.name}{" ("}{&info.submitting_user.rudn_id}{")"}</Link<Route>>
                    </p>
                    <p>
                        {"В ответ на задание: "}
                        <TaskLink task_id={info.task_id} />
                    </p>
                    <p>{"Вердикт: "}{verdict_line}</p>
                    {deets}
                </>)
            }
        },
    })
}

#[autoprops_component(ViewFSM)]
fn view_fsm(state_machine: &StateMachine) -> Html {
    let current_fsm: UseStateHandle<StateMachine> = use_state_eq(Default::default);
    use_effect_with(state_machine.clone(), {
        shadow_clone!(current_fsm);
        move |fsm| {
            current_fsm.set(fsm.clone());
        }
    });

    let word = use_state(|| String::from(""));
    let is_running = use_state(|| true);

    let on_terminate = {
        shadow_clone!(is_running);
        move |_outcome| {
            is_running.set(false);
        }
    };

    let oninput = {
        shadow_clone!(word);
        move |ev: InputEvent| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            word.set(target.value());
        }
    };

    let play_pulse = use_state(|| 0usize);
    let onclick = {
        shadow_clone!(is_running, play_pulse);
        move |ev: MouseEvent| {
            ev.prevent_default();
            is_running.set(true);
            play_pulse.set(*play_pulse + 1);
        }
    };

    let on_fsm_apply = {
        shadow_clone!(current_fsm);
        move |new_fsm: StateMachine| {
            current_fsm.set(new_fsm);
        }
    };

    let do_reset_fsm = {
        shadow_clone!(current_fsm, state_machine);
        move |_ev: MouseEvent| {
            current_fsm.set(state_machine.clone());
        }
    };

    let maybe_reset_button = if &*current_fsm == state_machine {
        html!(
            <div class="d-grid gap-2 my-2" style="width: 800px; margin: 0 auto;">
                <button class="btn btn-outline-success disabled" type="button" disabled={true}>{"Вы можете временно отредактировать это"}</button>
            </div>
        )
    } else {
        html!(
            <div class="d-grid gap-2 my-2" style="width: 800px; margin: 0 auto;">
                <button class="btn btn-outline-danger" type="button" onclick={do_reset_fsm}>{"Удалить изменения"}</button>
            </div>
        )
    };

    html!(
        <>
            <CanvasPlayer word={(&*word).clone()}
            fsm={(*current_fsm).clone()} editable={true} speed_changeable={true}
            auto_restart={true} show_status_indicator={true} show_transport_buttons={true}
            pause_on_restart={true} play_on_change={*is_running}
            speed={860} play_pulse={*play_pulse}
            {on_fsm_apply}
            {on_terminate}/>
            {maybe_reset_button}
            <form class="input-group my-2" style="width: 800px; margin: 0 auto;">
                <span class="input-group-text">{"Введите слово для проверки: "}</span>
                <input class="form-control" disabled={*is_running} value={(&*word).clone()} {oninput} />
                <input class="btn btn-success" disabled={*is_running} value="Тест!" type="submit" {onclick} />
            </form>
        </>
    )
}
