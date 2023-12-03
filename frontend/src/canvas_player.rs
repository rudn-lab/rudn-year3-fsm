use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    time::Duration,
};

use fsm::fsm::{FSMError, FSMOutput, StateMachine, StateMachineEvaluator};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_bootstrap::{
    component::{Column, Row},
    icons::BI,
};

use crate::canvas::Canvas;

pub struct CanvasPlayer {
    fsm: StateMachine,
    eval: Result<Result<StateMachineEvaluator, FSMError>, String>,
    word: String,
    link_highlights: Rc<HashMap<usize, JsValue>>,
    node_highlights: Rc<HashMap<usize, JsValue>>,
    node_crosses: Rc<HashSet<usize>>,

    auto_mode: bool,
    interval: Option<gloo::timers::callback::Interval>,
    interval_time: Duration,
    interval_slider: u32,

    current_step: u64,
    max_step: (u64, bool),

    status: Option<FSMOutput>,
}

#[derive(Properties, PartialEq)]
pub struct CanvasPlayerProps {
    #[prop_or(true)]
    pub speed_changeable: bool,

    #[prop_or(false)]
    pub auto_restart: bool,

    #[prop_or(true)]
    pub pause_on_restart: bool,

    #[prop_or_default]
    pub editable: bool,

    #[prop_or(true)]
    pub auto_play: bool,

    #[prop_or(true)]
    pub show_word_indicator: bool,

    #[prop_or(true)]
    pub show_status_indicator: bool,

    #[prop_or(false)]
    pub show_steps_indicator: bool,

    #[prop_or(true)]
    pub show_transport_buttons: bool,

    #[prop_or(false)]
    pub play_on_change: bool,

    #[prop_or(860)]
    pub speed: u32,

    pub word: AttrValue,
    pub fsm: StateMachine,

    #[prop_or_default]
    pub on_terminate: Callback<FSMOutput>,
}

#[derive(Debug)]
pub enum CanvasPlayerMsg {
    Interval,
    SpeedSliderChange(u32),
    NewFSMApplied(StateMachine),
    ResetFSM,
    AdvanceFSM,
    ToggleAuto,
}

// <3
const COLORS: &[&'static str] = &[
    "#E40303", "#FF8C00", "#FFED00", "#008026", "#24408E", "#732982",
];

impl CanvasPlayer {
    fn step(&mut self, ctx: &Context<Self>) {
        let mut needs_reset = false;
        let mut complexity = 0;
        if self.current_step == 0 {
            self.status = None;
        }

        if let Ok(Ok(eval)) = &mut self.eval {
            log::debug!("step!");
            eval.step();
            complexity = eval.link_cursors().len() + eval.node_cursors().len();
            let mut color_idx = 0;

            let mut new_nodehl = HashMap::new();
            let mut new_linkhl = HashMap::new();
            let mut new_node_crosses: HashSet<usize> = (*self.node_crosses).clone();
            for (node_idx, remaining_string) in eval.node_cursors() {
                new_nodehl.insert(*node_idx, JsValue::from_str(COLORS[color_idx]));
                color_idx += 1;
                color_idx = color_idx % COLORS.len();

                if remaining_string.is_empty() && self.fsm.nodes[*node_idx].accept_state {
                    new_node_crosses.insert(*node_idx);
                    self.status = Some(FSMOutput::Accept);
                }
            }
            color_idx = 0;
            for (link_idx, _before_remaining_string, _after_remaining_string) in eval.link_cursors()
            {
                new_linkhl.insert(*link_idx, JsValue::from_str(COLORS[color_idx]));
                color_idx += 1;
                color_idx = color_idx % COLORS.len();
            }

            // If only nodes with no exit are highlighted, then we reject.
            if eval.link_cursors().is_empty() && self.status.is_none() {
                let mut do_reject = true;
                for (node_idx, remaining_string) in eval.node_cursors() {
                    for link in self
                        .fsm
                        .links
                        .iter()
                        .filter(|l| l.get_nodes().0 == Some(*node_idx))
                    {
                        if remaining_string.starts_with(link.get_text()) {
                            do_reject = false;
                        }
                    }
                }

                if do_reject {
                    self.status = Some(FSMOutput::Reject);
                }
            }

            needs_reset = eval.link_cursors().is_empty() && eval.node_cursors().is_empty();

            self.node_highlights = Rc::new(new_nodehl);
            self.link_highlights = Rc::new(new_linkhl);
            self.node_crosses = Rc::new(new_node_crosses);

            if !self.max_step.1 {
                self.max_step.0 = self.current_step + 1;
            }
            if needs_reset {
                self.max_step.1 = true;
            } else {
                self.current_step += 1;
            }
        }

        if needs_reset && self.status.is_none() {
            self.status = Some(FSMOutput::Reject);
        }
        if needs_reset {
            ctx.props().on_terminate.emit(self.status.unwrap());
        }

        if needs_reset && ctx.props().auto_restart {
            self.reset(false);
        }
        if needs_reset && ctx.props().pause_on_restart {
            self.auto_mode = false;
        }

        let max_complexity = 512;
        if complexity > max_complexity {
            self.eval = Err(format!(
                "Слишком сложный автомат: {complexity} (>{max_complexity}) курсоров одновременно"
            ))
        }
    }

    fn reset(&mut self, changed: bool) {
        self.eval = Ok(StateMachineEvaluator::new(
            self.fsm.clone(),
            self.word.clone(),
        ));
        self.link_highlights = Default::default();
        self.node_highlights = Default::default();
        self.node_crosses = Default::default();
        self.current_step = 0;
        if changed {
            self.max_step = (0, false);
        }
    }

    fn word_indicator(&self) -> Html {
        match &self.eval {
            Ok(Ok(eval)) => {
                let word_len = self.word.chars().count();
                if word_len == 0 {
                    return html!(<span style="display: inline-block; flex: 1;">
                        <span class="badge text-bg-primary fs-3 font-monospace">{"ε"}</span>
                        </span>
                    );
                }

                let mut before_char_cursors = vec![];
                let mut final_char_cursors = vec![];
                let mut color_idx;

                for char_idx in self.word.chars().enumerate().map(|v| v.0) {
                    let mut this_char_cursors = vec![];

                    color_idx = 0;
                    for (node_id, remaining_word) in eval.node_cursors() {
                        let remaining_word_len = remaining_word.chars().count();
                        let remaining_word_start_idx = word_len - remaining_word_len;
                        if remaining_word_start_idx == char_idx {
                            this_char_cursors.push((*node_id, COLORS[color_idx]));
                        }
                        color_idx += 1;
                        color_idx = color_idx % COLORS.len();
                    }

                    before_char_cursors.push(this_char_cursors);
                }

                color_idx = 0;
                for (node_id, remaining_word) in eval.node_cursors() {
                    if remaining_word.is_empty() {
                        final_char_cursors.push((*node_id, COLORS[color_idx]));
                    }
                    color_idx += 1;
                    color_idx = color_idx % COLORS.len();
                }

                let mut under_char_cursors = vec![vec![]; word_len + 1];
                color_idx = 0;
                for (link_id, before_word, after_word) in eval.link_cursors() {
                    let before_word_len = before_word.chars().count();
                    let after_word_len = after_word.chars().count();
                    let before_word_start = word_len - before_word_len;
                    let after_word_start = word_len - after_word_len;
                    for idx in before_word_start..after_word_start {
                        if let Some(v) = under_char_cursors.get_mut(idx) {
                            v.push((*link_id, COLORS[color_idx]));
                        }
                    }
                    color_idx += 1;
                    color_idx = color_idx % COLORS.len();
                }

                let empty = vec![];
                let word_display = self
                .word
                .chars()
                .enumerate()
                .map(|(idx, char)| {
                    let mut v = html!({char});
                    let mut height = 0.1;
                    for (_node_id, color) in before_char_cursors.get(idx).unwrap_or(&empty) {
                        v = html!(<span style={format!("border-left: 0.1em solid; padding-left: 0.1em; border-left-color: {color};")}>{v}</span>);
                    }
                    for (_link_id, color) in under_char_cursors.get(idx).unwrap_or(&empty) {
                        v = html!(<span style={format!("border-bottom: 0.1em solid; padding-bottom: {height}em; border-bottom-color: {color};")}>{v}</span>);
                        height += 0.1;
                    }
                    if idx == word_len - 1 {
                        for (_node, color) in &final_char_cursors {
                            v = html!(<span style={format!("border-right: 0.1em solid; padding-right: 0.1em; border-right-color: {color};")}>{v}</span>);
                        }
                    }
                    v
                })
                .collect::<Html>();

                html!(<span class="fs-3 font-monospace" style="display: inline-block; flex: 1;">{word_display}</span>)
            }
            Err(text) => {
                html!(<p>{"Не могу обработать автомат: "}{text}</p>)
            }
            Ok(Err(e)) => match e {
                FSMError::InfiniteLoop => html!(<p>{"Автомат содержит бесконечный цикл"}</p>),
                FSMError::NoEntryLinks => html!(<p>{"Автомат не имеет входных стрелочек"}</p>),
                FSMError::DisjointedLink(_) => html!(<p>{"Автомат имеет ошибку связности"}</p>),
            },
        }
    }

    fn steps_indicator(&self) -> Html {
        html!(
            <span>
            {"Шаг:"}
            {self.current_step}
            {"/"}
            {self.max_step.0}
            {if !self.max_step.1 {"???"} else {""}}
            </span>
        )
    }

    fn status_indicator(&self) -> Html {
        match self.status {
            Some(FSMOutput::Accept) => {
                html!(<span class="text-success fs-4" style="flex: 1;">{"ACCEPT"}</span>)
            }
            Some(FSMOutput::Reject) => {
                html!(<span class="text-danger fs-4" style="flex: 1;">{"REJECT"}</span>)
            }
            None => html!(<span class="text-warning fs-4" style="flex: 1;">{"UNKNOWN"}</span>),
        }
    }
}

fn duration_from_slider(slider: u32) -> Duration {
    // Lowest(0 units) = 5000ms
    // highest(1000 units) = 100ms
    let inverse = 1000 - slider;
    let time = 100 + (5 * inverse);
    Duration::from_millis(time as u64)
}

impl Component for CanvasPlayer {
    type Message = CanvasPlayerMsg;

    type Properties = CanvasPlayerProps;

    fn create(ctx: &Context<Self>) -> Self {
        // let fsm: StateMachine = get_sample_fsm();
        let fsm = ctx.props().fsm.clone();
        Self {
            fsm: fsm.clone(),
            eval: Ok(StateMachineEvaluator::new(
                fsm,
                ctx.props().word.to_string(),
            )),
            word: ctx.props().word.to_string(),
            node_highlights: Default::default(),
            node_crosses: Default::default(),
            link_highlights: Default::default(),
            auto_mode: ctx.props().auto_play,
            interval: None,
            interval_time: duration_from_slider(ctx.props().speed),
            interval_slider: ctx.props().speed,
            current_step: 0,
            max_step: (0, false),
            status: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        self.fsm = ctx.props().fsm.clone();
        self.word = ctx.props().word.to_string();
        self.eval = Ok(StateMachineEvaluator::new(
            self.fsm.clone(),
            self.word.clone(),
        ));
        self.reset(true);
        if ctx.props().word != old_props.word || ctx.props().fsm != old_props.fsm {
            self.status = None;
        }
        // self.auto_mode = ctx.props().auto_play && ctx.props().auto_restart;
        self.auto_mode = self.auto_mode || ctx.props().play_on_change;
        if ctx.props().speed != old_props.speed {
            ctx.link()
                .send_message(CanvasPlayerMsg::SpeedSliderChange(ctx.props().speed));
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style="width: 800px; margin: 0 auto;">
                <Canvas init={self.fsm.clone()}
                    node_highlights={self.node_highlights.clone()}
                    link_highlights={self.link_highlights.clone()}
                    node_crosses={self.node_crosses.clone()}
                    immutable={!ctx.props().editable}
                    onchange={
                    ctx.link().callback(|new_fsm| {
                        CanvasPlayerMsg::NewFSMApplied(new_fsm)
                    })
                } />

                <div style="display: flex;" class="mb-2">
                    if ctx.props().show_word_indicator {
                        {self.word_indicator()}
                    }

                    if ctx.props().show_status_indicator {
                        {self.status_indicator()}
                    }

                </div>

                if ctx.props().show_transport_buttons {
                    <button class="btn btn-outline-success ml-3" onclick={
                        ctx.link().callback(|_ev: MouseEvent| {
                            CanvasPlayerMsg::ToggleAuto
                        })
                    }>{
                        if self.auto_mode {BI::PAUSE_CIRCLE_FILL} else {BI::PLAY_CIRCLE_FILL}
                    }</button>


                    <div class="btn-group mx-2">
                        <button class="btn btn-outline-danger" onclick={
                            ctx.link().callback(|_ev: MouseEvent| {
                                CanvasPlayerMsg::ResetFSM
                            })
                        }>{BI::ARROW_REPEAT}</button>
                        <button class="btn btn-outline-primary" onclick={
                            ctx.link().callback(|_ev: MouseEvent| {
                                CanvasPlayerMsg::AdvanceFSM
                            })
                        }>{BI::FAST_FORWARD_FILL}</button>
                    </div>
                }

                if ctx.props().speed_changeable {
                    <input class="form-input mx-2" type="range" min="0" max="1000" value={self.interval_slider.to_string()} disabled={!self.auto_mode} oninput={
                        ctx.link().callback(|ev: InputEvent| {
                            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
                            let value = target.value();
                            let value = value.parse().unwrap();
                            CanvasPlayerMsg::SpeedSliderChange(value)
                        })
                    }/>
                    <span class="mx-2">{"Скорость: "}{format!("{:?}", self.interval_time)}{"/кадр"}</span>
                }

                if ctx.props().show_steps_indicator {
                    {self.steps_indicator()}
                }
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CanvasPlayerMsg::Interval => {
                if self.auto_mode {
                    self.step(ctx);
                }
            }
            CanvasPlayerMsg::SpeedSliderChange(value) => {
                self.interval_slider = value;
                self.interval_time = duration_from_slider(value);
                let time = self.interval_time.as_millis() as u32;
                let send_interval = ctx.link().callback(|_| CanvasPlayerMsg::Interval);
                self.interval = Some(gloo::timers::callback::Interval::new(time, move || {
                    send_interval.emit(());
                }));
            }
            CanvasPlayerMsg::NewFSMApplied(fsm) => {
                self.fsm = fsm;
                self.reset(true);
            }
            CanvasPlayerMsg::ResetFSM => {
                self.reset(false);
            }
            CanvasPlayerMsg::ToggleAuto => {
                self.auto_mode = !self.auto_mode;
            }

            CanvasPlayerMsg::AdvanceFSM => {
                self.step(ctx);
            }
        }
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let send_interval = ctx.link().callback(|_| CanvasPlayerMsg::Interval);
            let time = self.interval_time.as_millis() as u32;
            self.interval = Some(gloo::timers::callback::Interval::new(time, move || {
                send_interval.emit(());
            }));
        }
    }
}
