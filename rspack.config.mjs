import { defineConfig } from '@rspack/cli';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default {
    entry: {
        web: './web/index.ts',
    },
    output: {
        //...
        path: path.resolve(__dirname, 'dist'),
        chunkFilename: '[id].js',
    },
    resolve: {
        extensions: ['.ts', '.tsx', '.mjs', '.js', '.jsx', '.json'],
        tsConfig: path.resolve(__dirname, './web/tsconfig.json'),
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                exclude: [/node_modules/],
                loader: 'builtin:swc-loader',
                options: {
                    jsc: {
                        parser: {
                            syntax: 'typescript',
                            dynamicImport: false,
                            privateMethod: true,
                            functionBind: true,
                            exportDefaultFrom: true,
                            exportNamespaceFrom: true,
                            decorators: true,
                            decoratorsBeforeExport: true,
                            topLevelAwait: true,
                            importMeta: true
                        },
                        keepClassNames: true,
                        transform: {
                        }
                    },
                    transform: {
                        keepClassNames: true,
                    }
                },
                type: 'javascript/auto',
            },
        ],
    },
};
