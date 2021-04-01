#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
use components::text::*;

use ta::{EnrichedText, Zone};

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let s: String ="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras finibus lacus tempor augue molestie, eget porttitor elit pellentesque. In hac habitasse platea dictumst. Mauris eu neque orci. Vivamus gravida pharetra sagittis. Pellentesque imperdiet risus neque, sit amet hendrerit augue dapibus sit amet. Vestibulum euismod vehicula volutpat. Curabitur ac ex et purus mollis cursus nec eget nunc. Nullam nunc sapien, maximus et imperdiet eu, ullamcorper in est. Integer diam magna, suscipit ac nisi sit amet, vehicula volutpat nisi.
        
        Nullam ultricies velit sed est hendrerit, id ornare mauris euismod. In sed molestie est. Sed egestas eleifend felis in lacinia. Sed luctus tortor eu nibh elementum facilisis. In vitae scelerisque sapien. Suspendisse facilisis purus tincidunt consectetur consequat. Morbi vehicula eu risus a tristique. Nulla viverra tincidunt orci vitae eleifend. Proin id mi ultrices nisl dignissim blandit lacinia eget ante. Morbi sed varius tellus. Nulla volutpat fringilla lorem, non porta massa scelerisque eu.
        
        Suspendisse sit amet tempus risus. Aliquam bibendum sit amet est non vestibulum. Phasellus purus libero, venenatis vel tempor in, faucibus nec massa. Fusce venenatis ex vel dolor posuere luctus. Integer nibh nibh, accumsan a massa sit amet, ultricies vehicula nisi. Maecenas a tellus turpis. Nam ut urna diam. Proin non purus ac augue laoreet ultricies. Vivamus nec fermentum arcu, ut tempor lectus.
        
        Fusce sed metus sit amet nisl porttitor aliquam. Praesent ullamcorper arcu eget lorem ultricies, a pretium ex venenatis. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Aliquam maximus lorem tincidunt lacinia consequat. In eget libero non metus ultrices finibus vel ut nunc. Curabitur diam lectus, condimentum in dignissim quis, posuere in lacus. Praesent convallis tristique massa eu mollis. Morbi sodales est ipsum, posuere lobortis nunc porttitor non. Quisque vehicula purus in odio accumsan faucibus.
        
        Curabitur nec pulvinar odio. Vestibulum consequat aliquam sem sed vulputate. Nunc lorem mauris, tristique quis fermentum non, tempor in dui. Fusce a dignissim tortor. Vivamus quis tempor risus. Nam tincidunt interdum ante, et pulvinar tellus ultricies cursus. Nullam massa lectus, gravida quis elit ut, semper blandit tellus. In egestas condimentum est sit amet dapibus. Vivamus fringilla posuere urna, eu pretium velit finibus eu. Donec eget mi felis. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Mauris fermentum leo eros, sit amet cursus est congue vitae. Nunc laoreet sed nunc non convallis. Phasellus in ultrices felis, sit amet semper tortor. Mauris egestas nibh sed lectus dapibus malesuada. Donec ullamcorper consectetur diam et sagittis.".into();
        let mut et = EnrichedText::new(s);
        let zone0: Zone<String> = vec![2..8, 50..120, 135..160].into();
        let zone1: Zone<String> = vec![7..13, 200..600].into();
        let zone2: Zone<String> = vec![(300..400)].into();

        et.add(zone0);
        et.add(zone1);
        et.add(zone2);

        html! {
            <TextComponent text={et}></TextComponent>
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
