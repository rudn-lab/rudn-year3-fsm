use fsm::fsm::StateMachine;
use gloo::events::EventListener;
use shadow_clone::shadow_clone;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::Element;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::canvas::self_link::SelfLink;

use self::any_link::Link;
use self::node::Node;
use self::normal_link::NormalLink;
use self::start_link::StartLink;
use self::temp_link::TemporaryLink;
use self::utils::SNAP_TO_PADDING;

mod any_link;
mod node;
mod normal_link;
mod self_link;
mod start_link;
mod temp_link;
mod utils;

#[wasm_bindgen]
extern "C" {
    fn prepare_canvas(c: Element);
    fn fetch_json() -> String;
    fn load_from_json(v: String);
}

#[derive(Properties, PartialEq, Clone)]
pub struct CanvasProps {
    #[prop_or_default]
    pub onchange: Callback<StateMachine>,

    #[prop_or_default]
    pub init: Option<StateMachine>,

    #[prop_or_default]
    pub immutable: bool,

    #[prop_or_default]
    pub node_highlights: Rc<HashMap<usize, JsValue>>,
    #[prop_or_default]
    pub link_highlights: Rc<HashMap<usize, JsValue>>,
    #[prop_or_default]
    pub node_crosses: Rc<HashSet<usize>>,
}

#[derive(PartialEq, Debug)]
enum Object {
    Node(usize),
    Link(usize),
}

struct SelectionContext {
    node_highlights: Rc<HashMap<usize, JsValue>>,
    node_crosses: Rc<HashSet<usize>>,
    link_highlights: Rc<HashMap<usize, JsValue>>,
    current_active_object: Option<Object>,
    caret_is_displayed: bool,
    canvas_is_focused: bool,
}

pub struct Canvas {
    canvas_ref: NodeRef,
    on_keydown: Option<EventListener>,
    on_keyup: Option<EventListener>,
    on_interval: Option<gloo::timers::callback::Interval>,
    nodes: Vec<Node>,
    links: Vec<Link>,
    temp_link: Option<TemporaryLink>,
    current_link: Option<Link>,
    selections: SelectionContext,

    shift: bool,
    original_click: (f64, f64),
    moving_object: bool,

    changes_since_last_interval: bool,
}

pub enum CanvasMessage {
    MouseDown { x: f64, y: f64 },
    DblClick { x: f64, y: f64 },
    MouseMove { x: f64, y: f64 },
    MouseUp { x: f64, y: f64 },
    KeyDown { keycode: u32, key_text: String },
    KeyUp { keycode: u32, key_text: String },
    Interval,
    MouseEnter,
    MouseLeave,
}

impl Canvas {
    fn render(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas_ref.cast().unwrap_throw();
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap_throw()
            .unwrap_throw()
            .dyn_into()
            .unwrap_throw();

        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        ctx.save();
        ctx.translate(0.5, 0.5).unwrap_throw();
        ctx.set_line_width(4.0);

        let white = JsValue::from_str("white");
        let red = JsValue::from_str("red");

        for (id, node) in self.nodes.iter().enumerate() {
            let me_is_selected;

            if self.selections.current_active_object == Some(Object::Node(id))
                && self.selections.canvas_is_focused
            {
                ctx.set_fill_style(&red);
                ctx.set_stroke_style(&red);
                me_is_selected = true;
            } else {
                ctx.set_fill_style(&white);
                ctx.set_stroke_style(&white);
                me_is_selected = false;
            }

            if let Some(color) = self.selections.node_highlights.get(&id) {
                ctx.set_fill_style(color);
                ctx.set_stroke_style(color);
            }

            node.draw(
                &ctx,
                me_is_selected,
                self.selections.node_crosses.contains(&id),
                &self.selections,
            );
        }
        let mut to_delete = HashSet::new();
        for (id, link) in self.links.iter().enumerate() {
            let me_is_selected;
            if self.selections.current_active_object == Some(Object::Link(id))
                && self.selections.canvas_is_focused
            {
                ctx.set_fill_style(&red);
                ctx.set_stroke_style(&red);
                me_is_selected = true;
            } else {
                ctx.set_fill_style(&white);
                ctx.set_stroke_style(&white);
                me_is_selected = false;
            }

            if let Some(color) = self.selections.link_highlights.get(&id) {
                ctx.set_fill_style(color);
                ctx.set_stroke_style(color);
            }

            if link
                .draw(&self.nodes, &ctx, me_is_selected, &self.selections)
                .is_err()
            {
                to_delete.insert(id);
            }
        }
        if !to_delete.is_empty() {
            self.changes_since_last_interval = true;
            let mut inner_idx = 0;
            self.links.retain(|_| {
                let delete = to_delete.contains(&inner_idx);
                inner_idx += 1;
                !delete
            });
        }
        if let Some(t) = &self.temp_link {
            ctx.set_fill_style(&white);
            ctx.set_stroke_style(&white);
            t.draw(&ctx);
        }
        if let Some(t) = &self.current_link {
            ctx.set_fill_style(&white);
            ctx.set_stroke_style(&white);
            let _ = t.draw(&self.nodes, &ctx, false, &self.selections);
        }

        ctx.restore();
    }

    fn find_selected_object(&mut self, pos: (f64, f64)) -> Option<Object> {
        for (id, node) in self.nodes.iter().enumerate() {
            if node.contains_point(pos) {
                return Some(Object::Node(id));
            }
        }
        let mut to_delete = HashSet::new();

        for (id, link) in self.links.iter().enumerate() {
            match link.contains_point(&self.nodes, pos) {
                Ok(true) => {
                    return Some(Object::Link(id));
                }
                Ok(_) => {}
                Err(_) => {
                    to_delete.insert(id);
                }
            }
        }
        if !to_delete.is_empty() {
            let mut inner_idx = 0;
            self.links.retain(|_| {
                let delete = to_delete.contains(&inner_idx);
                inner_idx += 1;
                !delete
            });
        }

        None
    }

    fn serialize(&self) -> StateMachine {
        let nodes = self.nodes.iter().map(|v| v.clone().into()).collect();
        let links = self.links.iter().map(|v| v.clone().into()).collect();

        StateMachine { nodes, links }
    }

    fn deserialize(&mut self, fsm: StateMachine) {
        self.current_link = None;
        self.temp_link = None;
        self.links.clear();
        self.nodes.clear();
        self.nodes.extend(fsm.nodes.into_iter().map(|v| v.into()));
        self.links.extend(fsm.links.into_iter().map(|v| v.into()));
    }

    fn snap_node(&mut self, node_id: usize) {
        let my_nodes = self.nodes.clone();
        let node = self.nodes.get_mut(node_id);
        let node = match node {
            Some(n) => n,
            None => return,
        };
        for (i, other_node) in my_nodes.into_iter().enumerate() {
            if node_id == i {
                continue;
            }

            if (node.x - other_node.x).abs() < SNAP_TO_PADDING {
                node.x = other_node.x;
            }
            if (node.y - other_node.y).abs() < SNAP_TO_PADDING {
                node.y = other_node.y;
            }
        }
    }
}

impl Component for Canvas {
    type Message = CanvasMessage;

    type Properties = CanvasProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut canvas = Self {
            canvas_ref: NodeRef::default(),
            on_keydown: None,
            on_keyup: None,
            on_interval: None,
            nodes: vec![],
            links: vec![],
            temp_link: None,
            current_link: None,
            selections: SelectionContext {
                node_highlights: ctx.props().node_highlights.clone(),
                link_highlights: ctx.props().link_highlights.clone(),
                node_crosses: ctx.props().node_crosses.clone(),
                current_active_object: None,
                caret_is_displayed: false,
                canvas_is_focused: false,
            },
            shift: false,
            moving_object: false,
            original_click: (0.0, 0.0),
            changes_since_last_interval: false,
        };
        if let Some(ref fsm) = ctx.props().init {
            canvas.deserialize(fsm.clone());
        }
        canvas
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if old_props.init != ctx.props().init {
            if let Some(ref v) = ctx.props().init {
                self.deserialize(v.clone());
                ctx.props().onchange.emit(self.serialize());
                return true;
            }
        }

        if ctx.props().immutable {
            self.moving_object = false;
            self.current_link = None;
            self.temp_link = None;
            self.selections.canvas_is_focused = false;
            self.selections.current_active_object = None;
        }

        self.selections.node_highlights = ctx.props().node_highlights.clone();
        self.selections.link_highlights = ctx.props().link_highlights.clone();
        self.selections.node_crosses = ctx.props().node_crosses.clone();

        self.render();
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut render = false;
        let mut change = false;
        match msg {
            CanvasMessage::MouseDown { x, y } => {
                if !ctx.props().immutable {
                    self.selections.current_active_object = self.find_selected_object((x, y));
                    render = true;

                    self.moving_object = false;
                    self.original_click = (x, y);
                    if let Some(ref obj) = self.selections.current_active_object {
                        if let Object::Node(id) = obj {
                            if self.shift {
                                self.current_link =
                                    Some(SelfLink::from_mouse(&self.nodes, *id, (x, y)).into());
                                self.temp_link = None;
                            }
                        }
                        if !matches!(obj, Object::Node(_)) || !self.shift {
                            self.moving_object = true;
                            match obj {
                                Object::Node(node) => {
                                    if let Some(node) = self.nodes.get_mut(*node) {
                                        node.set_mouse_start((x, y));
                                        change = true;
                                    }
                                }
                                Object::Link(link) => {
                                    if let Some(link) = self.links.get_mut(*link) {
                                        link.set_mouse_start(&self.nodes, (x, y));
                                        change = true;
                                    }
                                }
                            };
                        }
                    } else if self.shift {
                        self.temp_link = Some(TemporaryLink {
                            from: (x, y),
                            to: (x, y),
                        });
                        self.current_link = None;
                        render = true;
                    }
                }
            }
            CanvasMessage::DblClick { x, y } => {
                if !ctx.props().immutable {
                    self.selections.current_active_object = self.find_selected_object((x, y));
                    match self.selections.current_active_object {
                        None => {
                            self.nodes.push(Node::new(x, y));
                            change = true;
                            log::info!("Adding new node at {x} {y}");
                            self.selections.current_active_object =
                                Some(Object::Node(self.nodes.len() - 1));
                            render = true;
                        }
                        Some(Object::Node(id)) => {
                            if let Some(node) = self.nodes.get_mut(id) {
                                log::info!("Toggling state of node {id}");
                                node.is_accept_state = !node.is_accept_state;
                                change = true;
                            };
                            render = true;
                        }
                        _ => {}
                    }
                }
            }
            CanvasMessage::MouseMove { x, y } => {
                if !ctx.props().immutable {
                    if self.current_link.is_some() || self.temp_link.is_some() {
                        let target_node = self.find_selected_object((x, y));
                        let target_node = match target_node {
                            Some(Object::Node(id)) => self.nodes.get(id).map(|v| (id, v)),
                            _ => None,
                        };

                        if let Some(ref object) = self.selections.current_active_object {
                            // if selected object is not null
                            if let Some((id, _node)) = target_node {
                                // target node is not null
                                if self.selections.current_active_object == Some(Object::Node(id)) {
                                    // target node is selected object
                                    // self link
                                    self.current_link =
                                        Some(SelfLink::from_mouse(&self.nodes, id, (x, y)).into());
                                    self.temp_link = None;
                                    render = true;
                                } else if let Some(Object::Node(n)) =
                                    self.selections.current_active_object
                                {
                                    // target node is not current active object, but is a node
                                    self.current_link = Some(NormalLink::new(n, id).into());
                                    self.temp_link = None;
                                    render = true;
                                } else {
                                    // target node is null
                                    let node = {
                                        match object {
                                            Object::Node(id) => self.nodes.get(*id),
                                            _ => None,
                                        }
                                    };
                                    if let Some(node) = node {
                                        self.temp_link = Some(TemporaryLink {
                                            from: node.closest_point_on_circle((x, y)),
                                            to: (x, y),
                                        });
                                    } else {
                                        self.temp_link = Some(TemporaryLink {
                                            from: self.original_click,
                                            to: (x, y),
                                        });
                                    }
                                    self.current_link = None;
                                    render = true;
                                }
                            } else {
                                // target_node is null:
                                // started dragging at node, now over empty space
                                let node = {
                                    match object {
                                        Object::Node(id) => self.nodes.get(*id),
                                        _ => None,
                                    }
                                };
                                if let Some(node) = node {
                                    self.temp_link = Some(TemporaryLink {
                                        from: node.closest_point_on_circle((x, y)),
                                        to: (x, y),
                                    });
                                } else {
                                    self.temp_link = Some(TemporaryLink {
                                        from: self.original_click,
                                        to: (x, y),
                                    });
                                }
                                self.current_link = None;
                                render = true;
                            }
                        } else {
                            // if selected_object is null
                            if let Some((id, _node)) = target_node {
                                // target_node is not null
                                // current_link = start_link to this node
                                let mut l = StartLink::new(id);
                                let _ = l.set_anchor_point(&self.nodes, self.original_click);
                                self.current_link = Some(l.into());
                                self.temp_link = None;
                                render = true;
                            } else {
                                // current_link = temporary link to current position
                                self.current_link = None;
                                self.temp_link = Some(TemporaryLink {
                                    from: self.original_click,
                                    to: (x, y),
                                });
                                render = true;
                            }
                        }
                    }
                }

                if self.moving_object {
                    match self.selections.current_active_object {
                        Some(Object::Link(l)) => {
                            if let Some(l) = self.links.get_mut(l) {
                                let _ = l.set_anchor_point(&self.nodes, (x, y));
                                render = true;
                                change = true;
                            }
                        }
                        Some(Object::Node(n_id)) => {
                            let mut try_snap = false;
                            if let Some(n) = self.nodes.get_mut(n_id) {
                                n.set_anchor_point((x, y));
                                try_snap = true;
                                render = true;
                                change = true;
                            }
                            if try_snap {
                                self.snap_node(n_id);
                            }
                        }
                        None => {}
                    }
                }
            }
            CanvasMessage::MouseUp { .. } => {
                self.moving_object = false;
                if !ctx.props().immutable {
                    if let Some(l) = self.current_link.take() {
                        render = true;
                        self.links.push(l);
                        change = true;
                        self.selections.current_active_object =
                            Some(Object::Link(self.links.len() - 1));
                    }

                    if self.temp_link.take().is_some() {
                        render = true;
                    }
                }
            }
            CanvasMessage::KeyDown { keycode, key_text } => {
                if keycode == 16 {
                    self.shift = true;
                }
                if !ctx.props().immutable {
                    if self.selections.canvas_is_focused {
                        if keycode == 8 {
                            // backspace
                            render = true;
                            match self.selections.current_active_object {
                                Some(Object::Node(id)) => {
                                    if let Some(node) = self.nodes.get_mut(id) {
                                        node.text.pop();
                                        change = true;
                                    }
                                }
                                Some(Object::Link(id)) => {
                                    if let Some(link) = self.links.get_mut(id) {
                                        match link {
                                            Link::Normal(NormalLink { text, .. }) => text.pop(),
                                            Link::Start(StartLink { text, .. }) => text.pop(),
                                            Link::ToSelf(SelfLink { text, .. }) => text.pop(),
                                        };
                                        change = true;
                                    }
                                }
                                _ => {}
                            }
                        }

                        if key_text.chars().count() == 1 {
                            // letters
                            render = true;
                            match self.selections.current_active_object {
                                Some(Object::Node(id)) => {
                                    if let Some(node) = self.nodes.get_mut(id) {
                                        node.text.push_str(&key_text);
                                        change = true;
                                    }
                                }
                                Some(Object::Link(id)) => {
                                    if let Some(link) = self.links.get_mut(id) {
                                        match link {
                                            Link::Normal(NormalLink { text, .. }) => {
                                                text.push_str(&key_text)
                                            }
                                            Link::Start(StartLink { text, .. }) => {
                                                text.push_str(&key_text)
                                            }
                                            Link::ToSelf(SelfLink { text, .. }) => {
                                                text.push_str(&key_text)
                                            }
                                        };
                                        change = true;
                                    }
                                }
                                _ => {}
                            }
                        }

                        if keycode == 46 {
                            // delete
                            render = true;
                            match self.selections.current_active_object {
                                Some(Object::Link(id)) => {
                                    self.links.remove(id);
                                    change = true;
                                }
                                Some(Object::Node(id)) => {
                                    // Ordering of nodes is important for links:
                                    // those that link to items after this one
                                    // need to have indices shifted
                                    self.links.retain(|v| match v {
                                        Link::Normal(NormalLink { node_a, node_b, .. }) => {
                                            id != *node_a && id != *node_b
                                        }
                                        Link::Start(StartLink { node, .. }) => id != *node,
                                        Link::ToSelf(SelfLink { node, .. }) => id != *node,
                                    });

                                    self.nodes.remove(id);
                                    change = true;

                                    self.links.iter_mut().for_each(|v| match v {
                                        Link::Normal(NormalLink { node_a, node_b, .. }) => {
                                            if *node_a > id {
                                                *node_a -= 1;
                                            }
                                            if *node_b > id {
                                                *node_b -= 1;
                                            }
                                        }
                                        Link::Start(StartLink { node, .. }) => {
                                            if *node > id {
                                                *node -= 1;
                                            }
                                        }
                                        Link::ToSelf(SelfLink { node, .. }) => {
                                            if *node > id {
                                                *node -= 1;
                                            }
                                        }
                                    });
                                }
                                _ => {}
                            }
                            self.selections.current_active_object = None;
                        }
                    }
                }
            }
            CanvasMessage::KeyUp { keycode, .. } => {
                if keycode == 16 {
                    self.shift = false;
                }
            }
            CanvasMessage::Interval => {
                self.selections.caret_is_displayed = !self.selections.caret_is_displayed;
                render = true;
                if self.changes_since_last_interval {
                    ctx.props().onchange.emit(self.serialize());
                    self.changes_since_last_interval = false;
                }
            }
            CanvasMessage::MouseEnter => {
                if !ctx.props().immutable {
                    self.selections.canvas_is_focused = true;
                    render = true;
                }
            }
            CanvasMessage::MouseLeave => {
                self.selections.canvas_is_focused = false;
                render = true;
            }
        };

        if change {
            self.changes_since_last_interval = true;
        }
        if render {
            self.render();
            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = format!("max-width: 800px; background: black; border-radius: 20px; margin: 10px auto; border: 1px {} solid;",
            if self.selections.canvas_is_focused {"red"} else {"white"});
        html! {
            <canvas ref={self.canvas_ref.clone()}
            width="800" height="600" style={style}
            onmousedown={
                ctx.link().callback(|e: MouseEvent| {
                    let element: Element = e.target().unwrap().dyn_into().unwrap();
                    let rect = element.get_bounding_client_rect();
                    let x = e.client_x() as f64 - rect.left();
                    let y = e.client_y() as f64 - rect.top();
                    CanvasMessage::MouseDown{x,y}
                })
            }
            ondblclick={
                ctx.link().callback(|e: MouseEvent| {
                    let element: Element = e.target().unwrap().dyn_into().unwrap();
                    let rect = element.get_bounding_client_rect();
                    let x = e.client_x() as f64 - rect.left();
                    let y = e.client_y() as f64 - rect.top();
                    CanvasMessage::DblClick{x,y}
                })
            }
            onmousemove={
                ctx.link().callback(|e: MouseEvent| {
                    let element: Element = e.target().unwrap().dyn_into().unwrap();
                    let rect = element.get_bounding_client_rect();
                    let x = e.client_x() as f64 - rect.left();
                    let y = e.client_y() as f64 - rect.top();
                    CanvasMessage::MouseMove{x,y}
                })
            }
            onmouseup={
                ctx.link().callback(|e: MouseEvent| {
                    let element: Element = e.target().unwrap().dyn_into().unwrap();
                    let rect = element.get_bounding_client_rect();
                    let x = e.client_x() as f64 - rect.left();
                    let y = e.client_y() as f64 - rect.top();
                    CanvasMessage::MouseUp{x,y}
                })
            }
            onmouseenter={
                ctx.link().callback(|_e: MouseEvent| {
                    // let element: Element = e.target().unwrap().dyn_into().unwrap();
                    // let rect = element.get_bounding_client_rect();
                    // let x = e.client_x() as f64 - rect.left();
                    // let y = e.client_y() as f64 - rect.top();
                    CanvasMessage::MouseEnter
                })
            }
            onmouseleave={
                ctx.link().callback(|_e: MouseEvent| {
                    // let element: Element = e.target().unwrap().dyn_into().unwrap();
                    // let rect = element.get_bounding_client_rect();
                    // let x = e.client_x() as f64 - rect.left();
                    // let y = e.client_y() as f64 - rect.top();
                    CanvasMessage::MouseLeave
                })
            }
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let window = gloo::utils::window();
            let on_keydown = ctx.link().callback(|e: KeyboardEvent| {
                let code = e.key_code();
                let text = e.key();
                CanvasMessage::KeyDown {
                    keycode: code,
                    key_text: text,
                }
            });
            let on_keyup = ctx.link().callback(|e: KeyboardEvent| {
                let code = e.key_code();
                let text = e.key();
                CanvasMessage::KeyUp {
                    keycode: code,
                    key_text: text,
                }
            });

            let on_interval = ctx.link().callback(|_| CanvasMessage::Interval);

            self.on_keydown = Some(EventListener::new(&window, "keydown", move |e| {
                let event = e.dyn_ref::<KeyboardEvent>().unwrap_throw();
                on_keydown.emit(event.clone());
            }));
            self.on_keyup = Some(EventListener::new(&window, "keyup", move |e| {
                let event = e.dyn_ref::<KeyboardEvent>().unwrap_throw();
                on_keyup.emit(event.clone());
            }));

            self.on_interval = Some(gloo::timers::callback::Interval::new(250, move || {
                on_interval.emit(());
            }))
        }
    }
}
