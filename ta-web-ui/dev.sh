#!/bin/sh

wasm-pack build --dev --target web --out-name wasm --out-dir ./static && miniserve ./static/ --index index.html -p 8001
