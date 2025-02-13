const path = require('path');
const htmlPlugin = require('html-webpack-plugin');
const copyPlugin = require('copy-webpack-plugin');
const dotenvPlugin = require('dotenv-webpack');
const miniCssPlugin = require('mini-css-extract-plugin');

module.exports = {
    entry: './src/index.tsx',
    output: {
        path: path.resolve(__dirname, 'build'),
        filename: 'static/js/[name].[contenthash].js',
        clean: true,
        assetModuleFilename: 'static/media/[name][ext]',
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
        fallback: {
            process: false
        }
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
            {
                test: /\.css$/i,
                use: [
                    miniCssPlugin.loader,
                    'css-loader',
                    'sass-loader'
                ],
            },
            {
                test: /\.(woff|woff2|eot|ttf|otf)$/i,
                type: 'asset/resource',
                generator: {
                    filename: 'static/media/[name][ext]'
                }
            },
            {
                test: /\.(png|svg|jpg|jpeg|gif|ico)$/i,
                type: 'asset/resource',
                generator: {
                    filename: 'static/media/[name][ext]'
                }
            }
        ],
    },
    plugins: [
        new htmlPlugin({
            template: './public/index.html',
        }),
        new copyPlugin({
            patterns: [{ 
                from: 'public',
                globOptions: { ignore: ['**/index.html'] },
            }],
        }),
        new dotenvPlugin({ expand: true, systemvars: true }),
        new miniCssPlugin({
            filename: 'static/css/[name].[contenthash].css',
        })
    ],
    devtool: 'source-map'
};