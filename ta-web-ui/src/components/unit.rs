use boolinator::*;
use yew::prelude::*;

pub struct CharComponent {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub content: char,
    pub pos: usize,
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
        true
    }

    fn view(&self) -> Html {
        html! {
            <span
                class=("ta-char")
                id={format!("ta-char-pos-{}", self.props.pos)}
            >
                    {self.props.content}
            </span>
        }
    }
}
