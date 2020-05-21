import init, { Context, Direction } from "../pkg/wasm_experiments.js";

async function run() {
    await init();

    let context = new Context;
    context.Direction = Direction;
    return context;
}

export default(run());
