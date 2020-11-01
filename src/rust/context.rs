use crate::{
    webgl::WebGlContext,
    renderer::Renderer,
    world::World,
    callback::WorldgenCallback
};

use std::{
    sync::Arc,
    pin::Pin,
    task::Poll,
    future::Future,
};

use futures::poll;

use enumset::{
    EnumSet,
    EnumSetType
};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
#[derive(EnumSetType)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

#[wasm_bindgen]
pub struct FutureStore {
    callback: Arc<WorldgenCallback>,
    futures: Vec<Pin<Box<dyn Future<Output = JsValue>>>>,
}

impl FutureStore {
    pub fn new(callback: WorldgenCallback) -> FutureStore {
        FutureStore {
            callback: Arc::new(callback),
            futures: Vec::new()
        }
    }

    async fn update(mut self, num: f64) -> FutureStore {
        if self.futures.is_empty() {
            let callback = self.callback.clone();

            let future = async move {
                callback.does_this_work(num as f64).await
            };

            self.futures.push(Box::pin(future));
        }

        {
            let mut i = 0;

            while i != self.futures.len() {
                let done = match poll!(&mut self.futures[i]) {
                    Poll::Ready(value) => {
                        true
                    },

                    Poll::Pending => false
                };

                if done {
                    self.futures.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        self
    }
}

#[wasm_bindgen]
pub struct Context {
    renderer: Renderer,

    world: World,

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
            renderer: Renderer::new(WebGlContext::from_canvas_with_id("webgl")?)?,
            world: World::new("default", 256, 256),

            last_time: 0.0,

            current_directions: EnumSet::new(),
            current_offset: (0, 0),
        })
    }

    #[wasm_bindgen]
    pub fn future_store(callback: WorldgenCallback) -> Result<FutureStore, JsValue> {
        Ok(FutureStore::new(callback))
    }

    #[wasm_bindgen]
    pub fn resize_viewport(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        self.renderer.resize_viewport(width, height);
        Ok(self.world.resize(&self.renderer, width, height)?)
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
    pub fn generate_world(&mut self, seed: &str) -> Result<(), JsValue> {
        Ok(self.world.set_seed(seed)?)
    }

    #[wasm_bindgen]
    pub fn tick(&mut self, futures: FutureStore, time: f32) -> Result<js_sys::Promise, JsValue> {
        let delta = time - self.last_time;
        self.last_time = time;

        let scroll_speed_per_second = 200.0;

        let fraction_of_second = delta / 1000.0;
        let scroll_speed = scroll_speed_per_second * fraction_of_second;

        self.current_offset.0 += ((self.current_direction_scroll_value(Direction::Right) - self.current_direction_scroll_value(Direction::Left)) as f32 * scroll_speed) as i32;
        self.current_offset.1 += ((self.current_direction_scroll_value(Direction::Up) - self.current_direction_scroll_value(Direction::Down)) as f32 * scroll_speed) as i32;

        if self.current_offset.0 < 0 {
            self.current_offset.0 += 256;
            self.world.rotate_chunks(-1, 0)?;
        } else if self.current_offset.0 > 256 {
            self.current_offset.0 -= 256;
            self.world.rotate_chunks(1, 0)?;
        }

        if self.current_offset.1 < 0 {
            self.current_offset.1 += 256;
            self.world.rotate_chunks(0, -1)?;
        } else if self.current_offset.1 > 256 {
            self.current_offset.1 -= 256;
            self.world.rotate_chunks(0, 1)?;
        }

        self.renderer.render(self.world.chunks(), time, self.current_offset);

        Ok(future_to_promise(async move {
           Ok(futures.update(time as f64).await.into())
        }))
    }
}


