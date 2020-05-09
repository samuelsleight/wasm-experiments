import context_promise from './context.js'

context_promise.then(context => {
    function tick(time) {
        context.tick(time);
        window.requestAnimationFrame(tick);
    }

    function resize() {
        const canvas = document.getElementById("webgl");
        canvas.width = innerWidth;
        canvas.height = innerHeight;
        context.resize_viewport(innerWidth, innerHeight);
    }

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

    function keydown(e) {
        const direction = key_directions[e.code];

        if(direction !== undefined)
        {
            context.start_scroll(direction);
        }
    }

    function keyup(e) {
        const direction = key_directions[e.code];

        if(direction !== undefined)
        {
            context.stop_scroll(direction);
        }
    }

    resize();

    window.onresize = resize;
    window.onkeydown = keydown;
    window.onkeyup = keyup;

    window.requestAnimationFrame(tick);
})
