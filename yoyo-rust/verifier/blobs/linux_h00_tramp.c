/* Historical Stage 10-B C trampoline (gcc+CRT ~14KB).
 * Stage 11-B builds from linux_h00_tramp.S via scripts/build-linux-h00-tramp.sh
 * (nostdlib, ~10KB). Kept only as readable reference of the dlopen contract:
 *   dlopen("./libyoyo_runtime.so") → dlsym("yoyo_runtime_selfhost_main") → call.
 */
#include <dlfcn.h>
#include <stdlib.h>

int main(void) {
    void *h = dlopen("./libyoyo_runtime.so", RTLD_LAZY);
    if (!h) return 1;
    int (*fn)(void) = (int (*)(void))dlsym(h, "yoyo_runtime_selfhost_main");
    if (!fn) return 2;
    return fn();
}
