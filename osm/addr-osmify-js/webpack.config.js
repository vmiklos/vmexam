const path = require('path');

module.exports = {
    entry : './main.js',
    devtool : "source-map",
    output : {
        filename : 'bundle.js',
        path : path.resolve(__dirname, '.'),
    },
};
