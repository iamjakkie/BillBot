use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
mod services;
mod models;
mod pages;

use pages::{Home, Analysis};

#[derive(Clone, PartialEq)]
enum Route {
    Home,
    Analysis,
}

#[function_component(App)]
fn app() -> Html {
    let route = use_state(|| Route::Home);
    
    let on_navigate = {
        let route = route.clone();
        Callback::from(move |new_route: Route| {
            route.set(new_route);
        })
    };
    
    html! {
        <div class="app">
            <nav class="navbar">
                <h1>{"BillBot"}</h1>
                <div class="nav-links">
                    <button 
                        onclick={{
                            let on_navigate = on_navigate.clone();
                            move |_| on_navigate.emit(Route::Home)
                        }}
                        class={if *route == Route::Home { "active" } else { "" }}
                    >
                        {"Upload"}
                    </button>
                    <button 
                        onclick={{
                            let on_navigate = on_navigate.clone();
                            move |_| on_navigate.emit(Route::Analysis)
                        }}
                        class={if *route == Route::Analysis { "active" } else { "" }}
                    >
                        {"Analysis"}
                    </button>
                </div>
            </nav>
            <main class="main-content">
                { match &*route {
                    Route::Home => html! { <Home {on_navigate} /> },
                    Route::Analysis => html! { <Analysis /> },
                }}
            </main>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
