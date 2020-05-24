import init, {Context, Direction} from "../pkg/wasm_experiments.js"

export {Context, Direction}

export default init().then(() => {
    return new Context;
})
