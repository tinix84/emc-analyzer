#!/bin/bash
# build.sh

echo "🔧 Building WASM module..."
cd wasm
wasm-pack build --target web --out-dir ../public/wasm
cd ..

echo "📦 Building Nuxt app..."
npm run build

echo "✅ Build complete!"