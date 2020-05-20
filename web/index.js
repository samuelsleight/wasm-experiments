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

    let current_menu = null;

    function set_current_menu(element) {
        if(current_menu !== null) {
            current_menu.style.display = "none";
        }

        current_menu = element;

        if(current_menu !== null) {
            current_menu.style.display = "block";
        }
    }

    function generate_world(e) {
        e.preventDefault();

        const seed = document.getElementById("settings_seed").value;
        context.generate_world(seed);
    }

    function init_menu() {
        const settings_button = document.getElementById("menu_settings");
        const settings_section = document.getElementById("settings");
        settings_button.onmouseover = () => set_current_menu(settings_section);

        const reset_button = document.getElementById("menu_reset");
        reset_button.onmouseover = () => set_current_menu(null);
        reset_button.onclick = () => context.generate_world("default");

        const settings_form = document.getElementById("form_settings");
        settings_form.onsubmit = generate_world

        const overlay = document.getElementById("overlay");
        overlay.onmouseleave = () => set_current_menu(null);
    }

    resize();

    init_menu();

    window.onresize = resize;
    window.onkeydown = keydown;
    window.onkeyup = keyup;

    window.requestAnimationFrame(tick);
})
