use lite_lib::component::Component;
use lite_lib::components::{button::Button, select::Select};
use lite_lib::listener::EventListener;
use lite_lib::utils::{document, window};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, HtmlSelectElement, Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    // let button = Button::new("My first button", document().body().unwrap(), || {
    //     log("An awsome mouse click")
    // });
    // button.render();

    // EventListener::new(
    //     document()
    //         .get_element_by_id("button")
    //         .unwrap()
    //         .dyn_into::<HtmlElement>()
    //         .unwrap(),
    //     "click",
    //     |_event| add_button(),
    // );

    // let select = Select::new(
    //     "My first select",
    //     document().body().unwrap(),
    //     vec!["First", "Second", "Third"],
    // );
    // select.render();

    // EventListener::new(
    //     document()
    //         .get_element_by_id("select")
    //         .unwrap()
    //         .dyn_into::<HtmlElement>()
    //         .unwrap(),
    //     "change",
    //     |_event| catch_select(_event),
    // );

    catch_select().await;

    Ok(())
}

fn add_button() {
    let button = Button::new("My second button", document().body().unwrap(), || {
        log("An second awsome mouse click")
    });
    button.render();
}

async fn catch_select() {
    let mut req = RequestInit::new();
    req.method("GET");
    req.mode(RequestMode::Cors);
    let request =
        Request::new_with_str_and_init("http://127.0.0.1:7878/data",&req).expect("Request could not be created");

    let response = JsFuture::from(window().fetch_with_request(&request))
        .await
        .expect("Could not unwrap response");

    // `response` is a `Response` object.
    assert!(response.is_instance_of::<Response>());
    let resp: Response = response.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let page = JsFuture::from(resp.text().unwrap()).await.unwrap().as_string().unwrap();
    
    log(&page);

    // let e_target = _event.current_target().unwrap();
    // let element = e_target.dyn_into::<HtmlSelectElement>().unwrap();

    // log(&element.value());
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
