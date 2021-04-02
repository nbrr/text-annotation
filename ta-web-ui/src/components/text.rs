use boolinator::*;
use yew::prelude::*;

use ta::{EnrichedText,Interval};

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
    CharClick(usize),
    CharHover(usize),
}

impl Component for TextComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextComponent { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CharClick(pos) => 
                if let Some((p1,p2)) = self.props.ongoing_selection {
                    if let Some(i) =Interval::new(p1.min(p2),p2.max(p1)) {
                      self.props.intervals.push(i);
                      self.props.ongoing_selection = None
                    }
                } else {
                    self.props.ongoing_selection= Some((pos, pos))
                },
            Msg::CharHover(pos) => 
                self.props.ongoing_selection = self
                    .props
                    .ongoing_selection
                    .and_then(|(p1, _)| Some((p1, pos)))
            
        }
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
                    selected={self.props.ongoing_selection.filter(|(p1,p2)| p1 <= &ind && p2 >= &ind || p2 <= &ind && p1 >= &ind).is_some()}
                    parent_text={self.link.callback(|x|x)}
                    />
            })
            .collect::<Html>();

        let mut css : String = "".into();

        for (i,_) in self.props.text.zones.iter().enumerate(){
             css.push_str(format!(".ta-zone-{} {{ background-color: pink; }}", i).as_str())
        }

        css.push_str(".ta-char:hover {
            background-color: red;
        }
        
        .ta-char-selected {
            background-color: blue;
        }"
        .into());

        html! {
            <div>
                <style type="text/css">{css}</style>
                {chars}
            </div>
        }
    }
}
