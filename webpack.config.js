const path = require('path');

module.exports = {
    entry: path.join(__dirname, 'src', 'app.js'),
    output: {
        filename: 'bundle.js',
        path: path.join(__dirname, 'dist'),
    },
    resolve: {
        extensions: [ '.ts', '.js', '.pil' ]
    },
    module: {
        rules: [
            {
                test: /\.pil$/,
                loader: path.resolve(__dirname, './pil-loader.js'),
                exclude: /node_modules/
            },
            {
                test: /\.ts$/,
                loader: 'awesome-typescript-loader',
                exclude: /node_modules/,
            }
        ]
    },
    devServer: {
        contentBase: path.join(__dirname, ''),
        compress: true,
        port: 3000,
        publicPath: '/dist/',
    }
};
