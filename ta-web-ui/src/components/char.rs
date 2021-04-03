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
}

pub enum Msg {}

impl Component for CharComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CharComponent { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
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
        html! {
            <span
                class={classes}
                id={format!("ta-char-pos-{}", self.props.pos)}
            >
                    {self.props.content}
            </span>
        }
    }
}
