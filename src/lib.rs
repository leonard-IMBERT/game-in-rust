extern crate console_error_panic_hook;

use euclid::point2;

mod game;

use game::{
    manager::GameManager,
    keyboard::KeyboardManager,
    scene::basic_scene::BasicScene,
};

use std::{
    cell::RefCell,
    panic,
    rc::Rc,
};

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use web_sys::HtmlCanvasElement;

fn window() -> web_sys::Window { web_sys::window().expect("no global `window` exist") }
fn document() -> web_sys::Document { window().document().expect("`window` has no `document`") }
fn body() -> web_sys::HtmlElement { document().body().expect("`document` has no `body`") }

fn canvas() -> web_sys::HtmlCanvasElement {
    body().query_selector("canvas#game").map(|c_opt| match c_opt {
        Some(c) => c.dyn_into::<HtmlCanvasElement>().expect("Canvas is not a HtmlCanvasElement"),
        None => {
            let c = document().create_element("canvas").expect("Unexpected error when creating an element")
                .dyn_into::<HtmlCanvasElement>().expect("Canvas is not a HtmlCanvasElement");
            c.set_id("game");
            body().append_child(&c).expect("Unexpected error when appending child");
            c
        }
    }).expect("Was not able to found the canvas nor create it")
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let manager: GameManager<BasicScene> = GameManager::new(Rc::new(canvas()));
    manager.init(&body());

    let keyb = KeyboardManager::new(document())?;

    let scene = BasicScene::new(point2(100., 100.), 10., &keyb);

    let mut manager = manager.set_scene(scene);

    let loop_closure: Rc<RefCell<Option<Closure<_>>>> = Rc::new(RefCell::new(None));
    let setting = loop_closure.clone();

    let mut back_now : Option<f64> = None;
    *setting.borrow_mut() = Some(Closure::new(move |time: f64| {
        back_now = match back_now {
            None => Some(time),
            Some(t) => Some(t),
        };

        let dtime = time - back_now.unwrap_or(time);

        manager = manager.tick(dtime);

        window().request_animation_frame(loop_closure.borrow()
                                         .as_ref().unwrap()
                                         .as_ref().unchecked_ref())
            .expect("Failed to request animation");
    }));

    window().request_animation_frame(setting.borrow()
                                     .as_ref().unwrap()
                                     .as_ref().unchecked_ref())
        .expect("Failed to request animation");

    Ok(())
}
