import { Context } from "../../pkg/index.js"

function tick(context: Context, time: number) {
    context.tick(time);
    request_tick(context);
}

function request_tick(context: Context) {
    window.requestAnimationFrame(time => tick(context, time));
}

export default request_tick;
