#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;

use dioxus_html_macro::*;
use dioxus::prelude::*;
use tracing::{info, Level};
use std::sync::{Arc, Mutex};

#[cfg(feature="server")]
use sqlite::Connection;


#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    HomePage { },
    #[route("/about")]
    AboutPage { },
}

#[cfg(feature="server")]
lazy_static! {
    static ref DATABASE : Arc<Mutex<sqlite::Connection>> =
                                                Arc::new(
                                                    Mutex::new(
                                                        Connection::open("test_database.db")
                                                        .expect("Couldn't open database!")
                                                    )
                                                );
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    // #[cfg(features="web")]
    wasm_logger::init(wasm_logger::Config::default());
    launch(App);
}

fn App() -> Element {
    html!(
        <p>
            "This message will appear on all pages.
            But the following router content will be different depending on the URL:"
        </p>
        <div> 
            {rsx!{Router::<Route> {}}}
        </div>
    )
}

#[component]
fn AboutPage() -> Element {
    html! (
        <p>"Welcome to the About Page!"</p>
        <Link to={Route::HomePage {}}>"Click here to go back to home."</Link>
    )  
}

#[component]
fn HomePage() -> Element {
    let mut query_input : Signal<String> = use_signal(|| String::from(""));
    let mut response_text : Signal<Option<String>> = use_signal(|| None );
    html! (
        <p>"Welcome to the home page."</p>
        <Link to={Route::AboutPage {}}>"Click here to go to the About page"</Link>
        <div> 
            <form onsubmit={move |_| async move {
                if let Ok(response) = query_database((*query_input.read()).to_string()).await {
                    response_text.set(response);
                }
            }}> 
                <p>"Query something from the database here."</p>
                <input value="{query_input}" oninput={move |evt| query_input.set(evt.value())} name="query_input" placeholder="Please enter a query string here" required={true}/>
                <input r#type="submit"> </input> 
                <p>"Response text is {response_text:?}"</p>
            </form>
        </div>
    )
}

#[server]
async fn query_database(data: String) -> Result<Option<String>, ServerFnError> {
    //this code only runs on the server
    info!("Server received: {}", data);

    Ok(Some(String::from("Blah")))
}


