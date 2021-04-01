use boolinator::*;
use yew::prelude::*;

use super::unit::*;

pub struct TextComponent {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub content: String,
}

pub enum Msg {}

impl Component for TextComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextComponent { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let chars = self
            .props
            .content
            .char_indices()
            .map(|(ind, c)| html! {<CharComponent content={c} pos={ind}/>})
            .collect::<Html>();

        html! {
            <div>
                {chars}
            </div>
        }
    }
}
