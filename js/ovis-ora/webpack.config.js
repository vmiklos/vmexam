const path = require('path');

module.exports = {
    entry : './main.ts',
    devtool : 'inline-source-map',
    resolve : {extensions : [ ".ts" ]},
    module : {
        rules : [ {
            use : "ts-loader",
            exclude : /node_modules/,
        } ]
    },
    output : {
        filename : 'bundle.js',
        path : path.resolve(__dirname, '.'),
    },
};
