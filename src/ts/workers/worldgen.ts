import {expose} from "threads"

import("../../../pkg/index.js").then(module => {
    expose((num: number) => {
        return module.does_this_extra_work(num)
    })
})
