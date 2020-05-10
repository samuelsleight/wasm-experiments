use crate::{
    webgl::WebGlContext,
    renderer::Renderer
};

use enumset::{
    EnumSet,
    EnumSetType
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(EnumSetType)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[wasm_bindgen]
pub struct Context {
    renderer: Renderer,

    last_time: f32,

    current_directions: EnumSet<Direction>,
    current_offset: (i32, i32),
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Context, JsValue> {
        crate::utils::set_panic_hook();

        Ok(Context{
            renderer: Renderer::new(WebGlContext::from_canvas_with_id("webgl")?.into_context())?,

            last_time: 0.0,

            current_directions: EnumSet::new(),
            current_offset: (0, 0),
        })
    }

    #[wasm_bindgen]
    pub fn resize_viewport(&self, width: u32, height: u32) {
        self.renderer.resize_viewport(width, height);
    }

    #[wasm_bindgen]
    pub fn start_scroll(&mut self, direction: Direction) {
        self.current_directions.insert(direction);
    }

    #[wasm_bindgen]
    pub fn stop_scroll(&mut self, direction: Direction) {
        self.current_directions.remove(direction);
    }

    fn current_direction_scroll_value(&self, direction: Direction) -> i32 {
        if self.current_directions.contains(direction) {
            1
        } else {
            0
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self, time: f32) {
        let delta = time - self.last_time;
        self.last_time = time;

        let scroll_speed_per_second = 100.0;

        let fraction_of_second = delta / 1000.0;
        let scroll_speed = scroll_speed_per_second * fraction_of_second;

        self.current_offset.0 += ((self.current_direction_scroll_value(Direction::Right) - self.current_direction_scroll_value(Direction::Left)) as f32 * scroll_speed) as i32;
        self.current_offset.1 += ((self.current_direction_scroll_value(Direction::Up) - self.current_direction_scroll_value(Direction::Down)) as f32 * scroll_speed) as i32;

        self.renderer.render(time, self.current_offset);
    }
}


