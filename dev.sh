#!/bin/sh

miniflare \
    --binding TARGET_BASE_URL="" \
    --binding TARGET_FULL_URL="https://www.google.com" \
    --binding RESPONSE_CODE="302" \
    --modules \
    --modules-rule CompiledWasm=*.wasm \
    --wasm index.wasm=./build/worker/index.wasm \
    ./build/worker/shim.mjs
