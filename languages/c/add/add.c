#include <stdio.h>
#include <emscripten/emscripten.h>
#include <stdint.h>

uint32_t EMSCRIPTEN_KEEPALIVE add(uint32_t a, uint32_t b) {
    return a + b;
}
