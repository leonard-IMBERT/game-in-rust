
use crate::game::tools::{
    Quadrilater,
    SPoint
};
use crate::game::scene::Scene;
use crate::game::keyboard::KeyboardManager;

use wasm_bindgen::JsValue;
use web_sys::{
    HtmlCanvasElement,
    CanvasRenderingContext2d,
};

use euclid::point2;

pub struct BasicScene {
    position: SPoint,
    size: f32,
    keyboard: KeyboardManager,
}

const ACC: f32 = 1./120.;

impl BasicScene {
    pub fn new(start_point: SPoint, size: f32, keyboard: &KeyboardManager) -> BasicScene {
        BasicScene {
            position: start_point,
            size,
            keyboard: keyboard.clone(),
        }
    }


    fn get_square(&self) -> Quadrilater {
        Quadrilater{
            p1: point2(self.position.x - self.size / 2., self.position.y - self.size / 2.),
            p2: point2(self.position.x + self.size / 2., self.position.y - self.size / 2.),
            p3: point2(self.position.x + self.size / 2., self.position.y + self.size / 2.),
            p4: point2(self.position.x - self.size / 2., self.position.y + self.size / 2.),
        }
    }
}

impl Scene for BasicScene {
    fn on_start(&self, _canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        Ok(())
    }

    fn on_close(&self, _canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        Ok(())
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>{
        let square = self.get_square();
        ctx.move_to(square.p1.x as f64, square.p1.y as f64);
        ctx.begin_path();
        ctx.line_to(square.p2.x as f64, square.p2.y as f64);
        ctx.line_to(square.p3.x as f64, square.p3.y as f64);
        ctx.line_to(square.p4.x as f64, square.p4.y as f64);
        ctx.line_to(square.p1.x as f64, square.p1.y as f64);
        ctx.close_path();
        ctx.set_fill_style(&JsValue::from("black"));
        ctx.fill();

        Ok(())
    }

    fn tick(&self, dtime: f64) -> BasicScene {
        let k = &self.keyboard;
        let p = &self.position;
        //console::log_3(&"I'm here : ".into(), &p.x.into(), &p.y.into());
        let pos: SPoint = point2(
            match (k.left.get(), k.right.get()) {
                (true, false) => p.x - ACC * dtime as f32,
                (false, true) => p.x + ACC * dtime as f32,
                _ => p.x
            },
            match (k.up.get(), k.down.get()) {
                (true, false) => p.y - ACC * dtime as f32,
                (false, true) => p.y + ACC * dtime as f32,
                _ => p.y
            }
        );



        BasicScene {
            position: pos,
            size: self.size,
            keyboard: self.keyboard.clone()
        }
    }
}
