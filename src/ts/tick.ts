import { Context } from './context.js'

export default (context: Context) => {
    function tick(time: number) {
        context.tick(time);
        request_tick();
    }

    function request_tick() {
        window.requestAnimationFrame(tick);
    }

    request_tick();
}
