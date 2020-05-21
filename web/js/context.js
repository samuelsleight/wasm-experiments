import init, { Context, Direction } from "../pkg/wasm_experiments.js";

export default(init().then(() => {
    let context = new Context;
    context.Direction = Direction;
    return context;
}));
