const path = require('path');

module.exports = {
    entry : './main.js',
    mode : 'development',
    devtool : "source-map",
    module : {
        rules : [ {
            test : /\.js/,
            exclude : /node_modules/,
            use : "@jsdevtools/coverage-istanbul-loader"
        } ],
    },
    output : {
        filename : 'bundle.js',
        path : path.resolve(__dirname, '.'),
    },
};
