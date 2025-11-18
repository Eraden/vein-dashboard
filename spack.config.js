const { config } = require("@swc/core/spack");

module.exports = config({
    entry: {
        web: __dirname + "/web/index.ts",
    },
    output: {
        path: __dirname + "/dist",
    },
    options: {
        jsc: {
            baseUrl: ''
        }
        // root: __dirname + "/web"
    }
});
