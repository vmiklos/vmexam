const path = require('path');

module.exports = {
    entry : './main.ts',
    resolve : {extensions : [ ".ts", ".js" ]},
    module : {
        rules : [ {
            use : "ts-loader",
            exclude : /node_modules/,
        } ]
    },
    output : {
        filename : 'bundle.js',
        path : path.resolve(__dirname, './'),
    },
};
