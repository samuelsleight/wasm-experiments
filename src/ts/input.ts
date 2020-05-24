import { Context, Direction } from './context.js'

function has_focus(): boolean {
    return document.activeElement == document.body;
}

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

function key_event(event: KeyboardEvent, callback: (direction: Direction) => void) {
    if(!has_focus()) {
        return;
    }

    const direction = key_directions[event.code];

    if(direction !== undefined) {
        callback(direction);
    }
}

function resize_event(context: Context) {
    const canvas = <HTMLCanvasElement>document.getElementById("webgl");
    canvas.width = innerWidth;
    canvas.height = innerHeight;

    context.resize_viewport(innerWidth, innerHeight);
}

export default (context: Context) => {
    window.onkeydown = (event: KeyboardEvent) => key_event(event, context.start_scroll.bind(context));
    window.onkeyup = (event: KeyboardEvent) => key_event(event, context.stop_scroll.bind(context));
    window.onresize = () => resize_event(context);

    resize_event(context);
}
