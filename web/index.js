
import context_promise from './context.js'

async function run() {
    let context = await context_promise;

    function render(time) {
        context.render(time);
        window.requestAnimationFrame(render);
    }

    function resize() {
        const canvas = document.getElementById("webgl");
        canvas.width = innerWidth;
        canvas.height = innerHeight;
        console.log(context);
        context.resize_viewport(innerWidth, innerHeight);
    }

    resize();
    window.onresize = resize;

    window.requestAnimationFrame(render);
}

run();
