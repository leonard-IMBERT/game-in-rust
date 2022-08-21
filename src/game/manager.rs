use wasm_bindgen::{UnwrapThrowExt, JsCast};
use std::rc::Rc;
use web_sys::{
    HtmlElement,
    HtmlCanvasElement,
    CanvasRenderingContext2d,
};

use super::scene::Scene;

pub struct GameManager<A: Scene> {
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    scene: Option<A>,
}

impl<A: Scene> GameManager<A> {
    pub fn init(&self, parent: &HtmlElement) {
        self.canvas.set_width(u32::try_from(parent.offset_width()).ok().unwrap_or(100));
        self.canvas.set_height(u32::try_from(parent.offset_height()).ok().unwrap_or(100));
    }

    pub fn new(canvas: Rc<HtmlCanvasElement>) -> GameManager<A> {
        let ctx = Rc::new(canvas.get_context("2d")
                .expect("Context 2d is not available on canvas")
                .unwrap_throw()
                .dyn_into::<CanvasRenderingContext2d>()
                .expect("ctx is not a CanvasRenderingContext2d"));
        GameManager {
            canvas,
            context: ctx,
            scene: None,
        }
    }

    pub fn set_scene<B: Scene>(&self, scene: B) -> GameManager<B> {
        GameManager {
            canvas: self.canvas.clone(),
            context: self.context.clone(),
            scene: Some(scene),
        }
    }

    pub fn tick(&self, dtime: f64) -> GameManager<A> {
        self.context.set_fill_style(&"white".into());
        self.context.fill_rect(0., 0., self.canvas.width() as f64, self.canvas.height() as f64);

        match &self.scene {
            Some(s) => s.draw(&self.context),
            None => Ok(()),
        }.expect("Exception during draw");
        GameManager {
            canvas: self.canvas.clone(),
            context: self.context.clone(),
            scene: self.scene.as_ref().map(|s| s.tick(dtime)),
        }
    }
}
