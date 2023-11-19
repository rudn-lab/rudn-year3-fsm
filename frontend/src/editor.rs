use yew::prelude::*;
use yew_bootstrap::component::{Column, Row};

use crate::canvas::Canvas;

#[function_component(Editor)]
pub fn editor() -> Html {
    html! {
        <>
            <h1>{"FSM Editor"}</h1>
            <p>{"You can edit, save, load and test state machines here."}</p>
            <Row>
                <Column>
                    <Canvas />
                </Column>
                <Column>
                    <p>{"Controls..."}</p>
                </Column>
            </Row>

        </>
    }
}
