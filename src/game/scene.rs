use wasm_bindgen::JsValue;
use web_sys::{
    HtmlCanvasElement,
    CanvasRenderingContext2d,
};

pub mod basic_scene;

pub trait Scene {
    fn on_start(&self, canvas: &HtmlCanvasElement ) -> Result<(), JsValue>;
    fn on_close(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue>;
    fn tick(&self, dtime: f64) -> Self where Self: Sized;
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>;
}
