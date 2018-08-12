#include <stdio.h>
#include <emscripten/emscripten.h>
#include <stdint.h>

extern "C" {

    uint32_t EMSCRIPTEN_KEEPALIVE factorial(uint32_t n) {
        if (n > 1) {
            return n * factorial(n - 1);
        }
        else {
            return 1;
        }
    }

}
