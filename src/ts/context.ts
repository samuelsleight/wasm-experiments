import {WorldgenCallback} from "./worldgen"
export default import("../../pkg/index.js").then(module => {
    return new module.Context;
})
