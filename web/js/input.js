import { Direction } from './context.js'

function has_focus() {
    return document.activeElement == document.body;
}

export default context => {
    const key_directions = {
        "ArrowUp": Direction.Up,
        "KeyW": Direction.Up,
        "ArrowDown": Direction.Down,
        "KeyS": Direction.Down,
        "ArrowLeft": Direction.Left,
        "KeyA": Direction.Left,
        "ArrowRight": Direction.Right,
        "KeyD": Direction.Right,
    }

    function key_event(e, f) {
        if(!has_focus()) {
            return;
        }

        const direction = key_directions[e.code];

        if(direction !== undefined) {
            f(direction);
        }
    }

    window.onkeydown = e => key_event(e, context.start_scroll.bind(context));
    window.onkeyup = e => key_event(e, context.stop_scroll.bind(context));

    window.onresize = () => {
        const canvas = document.getElementById("webgl");
        canvas.width = innerWidth;
        canvas.height = innerHeight;

        context.resize_viewport(innerWidth, innerHeight);
    }

    window.onresize();
}
