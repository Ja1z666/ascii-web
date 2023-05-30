use gloo::console::log;
use image::{DynamicImage, GenericImageView};
use js_sys::Uint8Array;
use {std::ops::Deref, wasm_bindgen::JsCast, web_sys::HtmlInputElement, yew::prelude::*};

#[derive(Clone)]
struct Parameters {
    img: DynamicImage,
    scale: i32,
}

impl Parameters {
    fn load_ascii(&self) {
        // let image = image::load_from_memory(self.img).unwrap();
        // let (width, height) = image.dimensions();
        // log!(width, height);
    }
}

#[function_component]
fn App() -> Html {
    let parameters = use_state(|| Parameters {
        img: DynamicImage::new_rgba8(0, 0),
        scale: 16,
    });
    let cloned_parameters_scale = parameters.clone();
    let cloned_parameters_file = parameters.clone();

    let onchange_scale = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<i32>()
            .unwrap();
        cloned_parameters_scale.set(Parameters {
            scale: value,
            ..cloned_parameters_scale.deref().clone()
        });
    });

    let onchange_file = Callback::from(move |event: Event| {
        let file = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .files()
            .unwrap()
            .get(0)
            .unwrap();

        let array = Uint8Array::new(&file);
        let bytes: Vec<u8> = array.to_vec();
        let image = image::load_from_memory(&bytes).unwrap();
    });

    let submit = Callback::from(move |_| {
        parameters.load_ascii();
    });

    html! {
        <div class="container">
            <div class="main">
                <div class="logo">
                <h1>{"Ascii converter!"}</h1>
                <p>{"|、"}<br />{"(˚ˎ 。7"}<br />{"|、˜〵"}<br />{"じしˍ,)ノ"}</p>
                </div>
                <div class="settings">
                <div class="file">
                    <input onchange={onchange_file} type="file" id="image" accept="image/*" />
                    <p id="confirm">{"✓"}</p>
                </div>
                <input onchange={onchange_scale} type="range" min="1" max="64" id="scale" />
                </div>
                <input onclick={submit} class="submit" type="button" value="Submit" />
            </div>
            <textarea id="text" rows="4" cols="50" spellcheck="false" disabled={true}></textarea>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
