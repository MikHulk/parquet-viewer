#!/usr/bin/env node

import * as esbuild from 'esbuild'
import {wasmLoader} from 'esbuild-plugin-wasm';

let ctx = await esbuild.context({
  entryPoints: ['index.js'],
  outfile: './dist/main.js',
  bundle: true,
  format: 'esm',
  target: ['firefox108'],
  minify: process.env.NODE_ENV === 'production',
  sourcemap: process.env.NODE_ENV === 'development',
  plugins: [
    wasmLoader(),
  ],
})

let { host, port } = await ctx.serve({
  servedir: './dist',
})
