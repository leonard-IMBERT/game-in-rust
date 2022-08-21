use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Document, KeyboardEvent};

#[derive(Clone)]
pub struct KeyboardManager {
    pub right: Rc<Cell<bool>>,
    pub left: Rc<Cell<bool>>,
    pub up: Rc<Cell<bool>>,
    pub down: Rc<Cell<bool>>,
}

impl KeyboardManager {
    pub fn new(document: Document) -> Result<KeyboardManager, JsValue> {
        let keyb = KeyboardManager {
            right: Rc::new(Cell::new(false)),
            left:  Rc::new(Cell::new(false)),
            up:    Rc::new(Cell::new(false)),
            down:  Rc::new(Cell::new(false)),
        };

        let keyb_cop_d = keyb.clone();
        let handle_down = Closure::<dyn Fn(KeyboardEvent)>::new(move |evt: KeyboardEvent| {
            match evt.key().as_str() {

                "w" => keyb_cop_d.up.set(true),
                "a" => keyb_cop_d.left.set(true),
                "s" => keyb_cop_d.down.set(true),
                "d" => keyb_cop_d.right.set(true),
                _ => ()
            }});

        document.add_event_listener_with_callback("keydown", handle_down.as_ref().unchecked_ref())?;
        handle_down.forget();

        let keyb_cop_u = keyb.clone();
        let handle_up = Closure::<dyn Fn(KeyboardEvent)>::new(move |evt: KeyboardEvent| {
            match evt.key().as_str() {
                "w" => keyb_cop_u.up.set(false),
                "a" => keyb_cop_u.left.set(false),
                "s" => keyb_cop_u.down.set(false),
                "d" => keyb_cop_u.right.set(false),
                _ => ()
            }});

        document.add_event_listener_with_callback("keyup", handle_up.as_ref().unchecked_ref())?;

        handle_up.forget();

        Ok(keyb)
    }
}
