import init, { Context } from "./pkg/wasm_experiments.js";

async function run() {
    await init();
    return new Context;
}

export default(run());
