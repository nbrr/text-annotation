use boolinator::*;
use yew::prelude::*;

use super::char::*;
use ta::EnrichedText;

pub struct TextComponent {
    pub props: Props,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub text: EnrichedText<String>,
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
            .collect::<Html>();

        html! {
            <div>
                {chars}
            </div>
        }
    }
}
