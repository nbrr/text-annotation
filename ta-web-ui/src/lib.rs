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
        let s: String ="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras finibus lacus tempor augue molestie, eget porttitor elit pellentesque.
In hac habitasse platea dictumst. Mauris eu neque orci.
Vivamus gravida pharetra sagittis. Pellentesque imperdiet risus neque, sit amet hendrerit augue dapibus sit amet.
Vestibulum euismod vehicula volutpat. Curabitur ac ex et purus mollis cursus nec eget nunc.
Nullam nunc sapien, maximus et imperdiet eu, ullamcorper in est.
Integer diam magna, suscipit ac nisi sit amet, vehicula volutpat nisi.

Лорем ипсум долор сит амет, лаборе демоцритум вис ат, ан сед симул яуаерендум. Еа децоре оптион елаборарет пер, ех нам аццусата темпорибус диссентиет.
Дицам инермис пробатус яуо те. Те фугит яуаестио еос. Ат тантас мунере аппареат сеа, цу еам фацете деленити.

निर्देश संस्थान हमारी उसके उपलब्ध असक्षम व्याख्यान साधन देखने औषधिक गटकउसि वातावरण संस्थान दिये बिन्दुओ शारिरिक कार्यलय जानते वर्ष खरिदे बेंगलूर पुष्टिकर्ता दारी कर्य प्रति वैश्विक विश्लेषण मुश्किल संपुर्ण प्रौध्योगिकी गएआप तरीके

需健政茂値半谷改点推頂事写報道意。止一赤華今右奥科委真一京私年理暖国検。
圏稼買会信行謙携県教投提審一個捕。必弟対真界携代天島速要楽情際。
止高希績責功速告伊地会有遠丈戻堺査。月活月記世南像棟進運速物待著会円卓半国実。
属速増切岩謝定解葉組民本願扱歩年。友変井挑競解通齢誇測確待観伊気。真路点一引術泉月人視局広成尻縁配月会。

대통령은 법률안의 일부에 대하여 또는 법률안을 수정하여 재의를 요구할 수 없다, 헌법재판소 재판관은 탄핵 또는 금고 이상의 형의 선고에 의하지 아니하고는 파면되지 아니한다.
국회는 헌법개정안이 공고된 날로부터 60일 이내에 의결하여야 하며. 국군은 국가의 안전보장과 국토방위의 신성한 의무를 수행함을 사명으로 하며.".into();
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
