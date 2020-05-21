const modules = ['./input.js', './menu.js', './tick.js'];
import('./context.js').then(module => module.default.then(context => modules.forEach(module => import(module).then(module => module.default(context)))));
