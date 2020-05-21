function has_focus() {
    return document.activeElement == document.body;
}

export default context => {
    const key_directions = {
        "ArrowUp": context.Direction.Up,
        "KeyW": context.Direction.Up,
        "ArrowDown": context.Direction.Down,
        "KeyS": context.Direction.Down,
        "ArrowLeft": context.Direction.Left,
        "KeyA": context.Direction.Left,
        "ArrowRight": context.Direction.Right,
        "KeyD": context.Direction.Right,
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
