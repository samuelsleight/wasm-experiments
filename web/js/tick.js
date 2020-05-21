export default function(context) {
    function tick(time) {
        context.tick(time);
        window.requestAnimationFrame(tick);
    }

    window.requestAnimationFrame(tick);
}
