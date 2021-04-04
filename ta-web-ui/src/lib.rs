#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
use components::text::*;

mod exstring;

use ta::{EnrichedText, Zone};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

struct Model {
    pub link: ComponentLink<Self>,
    selection_callback: Closure<dyn FnMut()>,
}

enum Msg {
    SelectionChange,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let selection_callback = link.callback(|_: ()| Msg::SelectionChange);
        let selection_callback = Box::new(move || selection_callback.emit(())) as Box<dyn FnMut()>;
        let selection_callback = Closure::wrap(selection_callback);

        Self {
            link,
            selection_callback,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let s: String = exstring::s.clone().into();

        let mut et = EnrichedText::new(s);
        let zone0: Zone<String> = vec![2..8, 50..120, 135..160].into();
        let zone1: Zone<String> = vec![7..13, 200..600].into();
        let zone2: Zone<String> = vec![(300..400)].into();

        et.add(zone0);
        et.add(zone1);
        et.add(zone2);

        let doc = yew::utils::document();
        doc.add_event_listener_with_callback(
            "selectionchange",
            self.selection_callback.as_ref().unchecked_ref(),
        )
        .unwrap();

        let select = doc.get_selection().unwrap().unwrap();
        let anchor_offset = select.anchor_offset() as usize;
        let focus_offset = select.focus_offset() as usize;
        let beg = anchor_offset.min(focus_offset);
        let end = anchor_offset.max(focus_offset);
        let selected_text: String = et
            .content
            .clone()
            .chars()
            .skip(beg)
            .take(end - beg)
            .collect();

        html! {
            <div>
                <div>{format!("anchor offset:{:?}", anchor_offset)}</div>
                <div>{format!("focus offset:{:?}", focus_offset)}</div>
                <div>{format!("anchor node is focus node? :{:?}", select.anchor_node() == select.focus_node())}</div>
                <div>{format!("range count:{:?}", select.range_count())}</div>
                <hr/>
                <TextComponent text={et}></TextComponent>
                <hr/>
                <div style="border: 1px solid black; background-color: #e7e7e7; margin: 50px; white-space: pre;">{selected_text}</div>
            </div>
        }
    }
}

extern crate console_error_panic_hook;
use std::panic;
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<Model>::new().mount_to_body();
}
