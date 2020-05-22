import init, { Context } from './context.js'

import * as Input from './input.js'
import * as Menu from './menu.js'
import * as Tick from './tick.js'

interface Module {
    default: (context: Context) => void;
}

init.then((context: Context) => {
    for(const module of <(Module)[]>[Input, Menu, Tick]) {
        module.default(context);
    }
})
