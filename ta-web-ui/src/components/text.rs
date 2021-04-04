use yew::prelude::*;

use ta::EnrichedText;

pub struct TextComponent {
    pub props: Props,
    _link: ComponentLink<Self>,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub text: EnrichedText<String>,
    #[prop_or_default]
    pub ongoing_selection: Option<(usize, usize)>,
}

pub enum Msg {}

impl Component for TextComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextComponent { props, _link: link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
        .ta-zone-2 { background-color: #B55A30; }

        .ta-zone-0.ta-zone-1 { background-color: #CCB294; }
        .ta-zone-0.ta-zone-2 { background-color: #D98342; }
        .ta-zone-1.ta-zone-2 { background-color: #A59292; }
        .ta-zone-0.ta-zone-1.ta-zone-2 { background-color: red; }
        "
            .into(),
        );

        let partition = self.props.text.partition_by_zones();
        let mut text_chars = self.props.text.content.chars();
        let text_with_zones: Vec<Html> = partition
            .iter()
            .map(|(length, zones)| {
                let segment_chars: String = text_chars.by_ref().take(*length).collect();
                if !zones.is_empty() {
                    let classes: Vec<String> =
                        zones.iter().map(|n| format!("ta-zone-{}", n)).collect();
                    html!(<span class={classes}>{segment_chars}</span>)
                } else {
                    html!(segment_chars)
                }
            })
            .collect();

        html! {
            <div id="container">
                <style type="text/css">{css}</style>
                <div>{&self.props.text.content}</div>
                <div id="bg">{text_with_zones}</div>
            </div>
        }
    }
}
