use boolinator::*;
use yew::prelude::*;

use super::text;

pub struct CharComponent {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub content: char,
    pub pos: usize,
    pub zones: Vec<usize>,
    pub parent_text: Callback<text::Msg>,
    pub selected: bool,
}

pub enum Msg {
    Click,
    Hover,
}

impl Component for CharComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CharComponent { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => self
                .props
                .parent_text
                .emit(text::Msg::CharClick(self.props.pos)),
            Msg::Hover => self
                .props
                .parent_text
                .emit(text::Msg::CharHover(self.props.pos)),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut classes: Vec<String> = self
            .props
            .zones
            .clone()
            .iter()
            .map(|i| format!("ta-zone-{}", i))
            .collect();
        classes.push("ta-char".into());
        if self.props.selected {
            classes.push("ta-char-selected".into());
        }
        html! {
            <span
                class={classes}
                id={format!("ta-char-pos-{}", self.props.pos)}
                onclick=self.link.callback(|_| Msg::Click)
                onmouseover=self.link.callback(|_| Msg::Hover)
            >
                    {self.props.content}
            </span>
        }
    }
}
