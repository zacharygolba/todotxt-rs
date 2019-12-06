import path from "path";

import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";
import TerserWebpackPlugin from "terser-webpack-plugin";
import webpack, { Configuration } from "webpack";

const enum Environment {
  Development = "development",
  Production = "production",
}

export default function createConfig(mode = Environment.Development): Configuration {
  const isDebug = mode !== Environment.Production;
  const packageRoot = path.resolve(__dirname, "..");

  return {
    mode,
    devtool: isDebug ? "cheap-eval-source-map" : "source-map",
    entry: path.resolve(packageRoot, "src/main.tsx"),
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          loader: "ts-loader",
          options: {
            configFile: path.resolve(packageRoot, "config/typescript/app.tsconfig.json"),
          },
        },
      ],
    },
    optimization: {
      minimize: !isDebug,
      minimizer: [new TerserWebpackPlugin()],
    },
    output: {
      path: path.resolve(packageRoot, "build"),
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: path.resolve(packageRoot, "public/index.html"),
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(packageRoot, "rust"),
        extraArgs: isDebug ? "" : "--release",
      }),
    ],
    resolve: {
      extensions: [".js", ".json", ".jsx", ".ts", ".tsx", ".wasm"],
    },
  };
}
