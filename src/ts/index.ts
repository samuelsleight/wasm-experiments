import init from "./context"

init.then(async ctx => {
    interface Module {
        default: (context: typeof ctx) => void;
    }

    for(const module of <(Module)[]>[await import("./input"), await import("./menu"), await import("./tick")]) {
        module.default(ctx);
    }
})
