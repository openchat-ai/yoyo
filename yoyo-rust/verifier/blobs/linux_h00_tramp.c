/* Stage 10-B: tiny dlopen trampoline for ELF H_00 path.
 * Written to cwd by H_00 stub, then execve'd. Expects ./libyoyo_runtime.so.
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
