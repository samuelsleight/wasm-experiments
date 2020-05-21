import context_promise from './context.js'

context_promise.then(context => {
    function start(module) {
        import(module).then(module => module.default(context));
    }

    start('./input.js');
    start('./menu.js');
    start('./tick.js');
})
