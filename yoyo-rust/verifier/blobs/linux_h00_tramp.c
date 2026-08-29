/* Stage 11-B builds from linux_h00_tramp.S via scripts/build-linux-h00-tramp.sh
 * Hybrid: dynamic -lc only (no libdl NEEDED); dlopen@PLT + in-process ELF dyn sym walk.
 * Reference only — committed blob is linux_h00_tramp.elf from .S.
 */
#if 0
#include <dlfcn.h>
int main(void) {
    void *h = dlopen("./libyoyo_runtime.so", RTLD_LAZY);
    if (!h) return 1;
    int (*fn)(void) = (int (*)(void))dlsym(h, "yoyo_runtime_selfhost_main");
    if (!fn) return 2;
    return fn();
}
#endif
