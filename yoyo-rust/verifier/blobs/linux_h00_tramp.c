/* Historical Stage 10-B C trampoline (gcc+CRT ~14KB).
 * Stage 11-B builds from linux_h00_tramp_mmap.c via scripts/build-linux-h00-tramp.sh
 * (static syscall mmap loader, ~9KB). Kept only as readable reference of the pre-mmap dlopen contract.
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
