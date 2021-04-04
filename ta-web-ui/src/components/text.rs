use boolinator::*;
use yew::prelude::*;

use ta::{EnrichedText, Interval};

use super::char::*;

pub struct TextComponent {
    pub props: Props,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub text: EnrichedText<String>,
    #[prop_or_default]
    pub ongoing_selection: Option<(usize, usize)>,
    #[prop_or_default]
    pub intervals: Vec<Interval>,
}

pub enum Msg {
    SelectionChange,
}

impl Component for TextComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextComponent { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectionChange => true,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let mut css: String = "".into();

        css.push_str(
            "
        #container {
            position: relative;
            white-space: pre;
        }

        #bg {
            color: rgba(0,0,0,0);
            position:absolute;
            left:0px;
            top:0px;
            z-index: -1;
        }

        .ta-zone-0 { background-color: #FDAC53; }
        .ta-zone-1 { background-color: #9BB7D4; }
        .ta-zone-2 { background-color: #5B55A30; }
        "
            .into(),
        );

        html! {
            <div id="container">
                <style type="text/css">{css}</style>
                <div>{self.props.text.content.clone()}</div>
                <div id="bg">{self.chars()}</div>
            </div>
        }
    }
}

impl TextComponent {
    fn chars(&self) -> Html {
        self
            .props
            .text
            .content
            .char_indices()
            .map(|(ind, c)| html! {
                <CharComponent
                    content={c}
                    pos={ind}
                    zones={
                        self.props.text.zones.iter().enumerate().filter_map(|(i, zone)| zone.contains(ind).as_some(i)).collect::<Vec<usize>>()
                    }
                    />
            })
            .collect::<Html>()
    }
}
