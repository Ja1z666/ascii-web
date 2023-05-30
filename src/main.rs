use gloo::console::log;
use image::GenericImageView;
use js_sys::Uint8Array;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone)]
struct Parameters {
    bytes: Vec<u8>,
    scale: u32,
    ascii_symb: String,
    is_empty: bool,
}

impl Parameters {
    fn load_ascii(&self) {
        let image = image::load_from_memory(&self.bytes).unwrap();
        let (width, height) = image.dimensions();
        let mut result = String::new();
        let mut ascii = self.ascii_symb.split("").collect::<Vec<&str>>();
        ascii[0] = " ";
        ascii.pop();

        for y in 0..height {
            for x in 0..width {
                if y % (self.scale * 2) == 0 && x % self.scale == 0 {
                    let pix = image.get_pixel(x, y);
                    let mut intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                    if pix[3] == 0 {
                        intent = 0;
                    }
                    result += Self::get_ascii(intent, ascii.clone());
                }
            }
            if y % (self.scale * 2) == 0 {
                result += "\n";
            }
        }

        window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("text")
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .set_value(&result);
    }

    fn get_ascii(intent: u8, ascii_symb: Vec<&str>) -> &str {
        let i = (256 / ascii_symb.len()) as u8;
        let mut index = intent / i;
        if index > ascii_symb.len() as u8 - 1 {
            index -= 1;
        }
        return ascii_symb.get(index as usize).unwrap();
    }
}

#[function_component]
fn App() -> Html {
    let parameters = use_state(|| Parameters {
        bytes: Vec::new(),
        scale: 16,
        ascii_symb: String::from(".,-~+=@"),
        is_empty: true,
    });
    let cloned_parameters_scale = parameters.clone();
    let cloned_parameters_file = parameters.clone();
    let cloned_parameters_text = parameters.clone();

    let cloned_parameters_usage = parameters.clone();

    let onchange_scale = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<u32>()
            .unwrap();
        cloned_parameters_scale.set(Parameters {
            scale: value,
            ..cloned_parameters_scale.deref().clone()
        });
    });

    let onchange_file = Callback::from(move |event: Event| {
        let param_clone = cloned_parameters_file.clone();
        let file = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .files()
            .unwrap()
            .get(0)
            .unwrap();

        let sperma = file.array_buffer();
        let js_future = JsFuture::from(sperma);
        spawn_local(async move {
            let result = js_future.await;
            let output = match result {
                Ok(result) => result,
                Err(_) => todo!(),
            };
            let array = Uint8Array::new(&output);
            let bytes = array.to_vec();

            param_clone.set(Parameters {
                bytes,
                is_empty: false,
                ..param_clone.deref().clone()
            });
        });

        window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("text")
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .set_value("");
    });

    let onchange_text: Callback<Event> = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_parameters_text.set(Parameters {
            ascii_symb: value,
            ..cloned_parameters_text.deref().clone()
        });
    });

    let submit = Callback::from(move |_| {
        if parameters.is_empty {
            return window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("text")
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .set_value("Please choose picture...");
        } else {
            if parameters.ascii_symb.len() == 0 {
                return window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("text")
                    .unwrap()
                    .unchecked_into::<HtmlInputElement>()
                    .set_value("Please input symbols...");
            } else {
                parameters.load_ascii();
            }
        }
    });

    html! {
        <div class="container">
            <div class="main">
                <div class="logo">
                <h1>{"Ascii converter!"}</h1>
                <p><i>{"|、"}<br />{"(˚ˎ 。7"}<br />{"|、˜〵"}<br />{"じしˍ,)ノ"}</i></p>
                </div>
                <div class="settings">
                    <div class="file">
                        <input onchange={onchange_file} type="file" id="image" accept=".jpg, .jpeg, .png" />
                        if !cloned_parameters_usage.is_empty {
                            <p>{"✓"}</p>
                        }
                    </div>
                    <input onchange={onchange_scale} type="range" min="1" max="64" id="scale" />
                    <input onchange={onchange_text} type="text" maxlength="27" value={cloned_parameters_usage.ascii_symb.clone()} />
                </div>
                <input onclick={submit} class="submit" type="button" value="Submit" />
            </div>
            <textarea style={if cloned_parameters_usage.is_empty {"color: #ff7171"} else {"color: #ffffff"}} id="text" rows="4" cols="50" spellcheck="false" disabled={true}></textarea>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
