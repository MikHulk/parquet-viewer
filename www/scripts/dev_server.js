#!/usr/bin/env node
import { copyFile, constants } from 'node:fs/promises';
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

try {
  await copyFile('./index.html', 'dist/index.html');
  console.log('index.html was copied to destination');
} catch {
  console.warn('index.html could not be copied');
}

await ctx.watch()

let { host, port } = await ctx.serve({
  servedir: './dist',
})
