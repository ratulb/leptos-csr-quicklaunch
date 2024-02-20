use wasm_bindgen::prelude::wasm_bindgen;
use leptos::{IntoView, component, view, create_signal, mount_to_body,SignalUpdate, For, logging, SignalGet, SignalWith, create_memo}; 

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, mushy-mushy!");
}

#[wasm_bindgen(start)]
pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    age: u8,
}

#[component]
pub fn App() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            age: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            age: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            age: 15,
        },
    ]);
    view! {
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.age *= 2;
                    }
                });
            logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each age
        <For
            each=move || data.get().into_iter().enumerate()
            key=|(_, state)| state.key.clone()
            children=move |(index, _)| {
                let age = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.age).unwrap_or(0))
                });
                view! {
                    // <p>{move || age.get()}</p>
                    <p>{age}</p>
                }
            }
        />
    }
}

