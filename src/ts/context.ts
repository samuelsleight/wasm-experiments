export default import("../../pkg/index.js").then(module => {
    return new module.Context;
})
