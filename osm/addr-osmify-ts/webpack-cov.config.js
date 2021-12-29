const path = require('path');

module.exports = {
    entry : './main.ts',
    resolve : {extensions : [ ".ts" ]},
    module : {
        rules : [ {
            use : [ "@jsdevtools/coverage-istanbul-loader", "ts-loader" ],
            exclude : /node_modules/,
        } ]
    },
    output : {
        filename : 'bundle.js',
        path : path.resolve(__dirname, '.'),
    },
};
