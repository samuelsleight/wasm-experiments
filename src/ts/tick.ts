import { Context, FutureStore } from "../../pkg/index.js"
import { WorldgenCallback } from "./worldgen";

async function tick(context: Context, time: number, futures: FutureStore) {
    let new_futures = await context.tick(futures, time);
    request_tick(context, new_futures);
}

function request_tick(context: Context, futures: FutureStore) {
    window.requestAnimationFrame(time => tick(context, time, futures));
}

export default (context: Context) => {
    let wgc = new WorldgenCallback;
    request_tick(context, Context.future_store(wgc));
}
