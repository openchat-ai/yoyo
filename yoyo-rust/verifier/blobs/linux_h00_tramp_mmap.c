/* Stage 11-B / post-v1.0 OW-IAT: hybrid ELF trampoline.
 *
 * Dynamic link: -lc -lgcc_s only (no libdl). ld.so maps libc + libgcc before _start.
 * Adopt libc + libgcc bases from /proc/self/maps (do NOT mmap glibc/ld from disk).
 * Load cwd ./libyoyo_runtime.so via libc dlopen (resolved from adopted libc symtab;
 * no libdl NEEDED, no dlsym) then in-process ELF dyn sym walk for entry.
 *
 * Rebuild: scripts/build-linux-h00-tramp.sh
 */
typedef unsigned long long u64;
typedef unsigned int u32;
typedef unsigned short u16;
typedef unsigned char u8;
typedef long long i64;

#define SYS_read 0
#define SYS_open 2
#define SYS_close 3
#define SYS_mmap 9
#define SYS_exit 60

#define O_RDONLY 0
#define MAP_PRIVATE 0x02
#define MAP_ANONYMOUS 0x20
#define PROT_READ 1
#define PROT_WRITE 2

#define PT_LOAD 1
#define PT_DYNAMIC 2
#define DT_NULL 0
#define DT_STRTAB 5
#define DT_SYMTAB 6
#define DT_STRSZ 10
#define DT_SYMENT 11

#define ELF_MAGIC 0x464c457fU
#define EM_X86_64 62
#define ELFCLASS64 2
#define STT_FUNC 2
#define STB_GLOBAL 1
#define STB_WEAK 2

#define PAGE_SIZE 4096
#define RTLD_LAZY 1

struct elf64_hdr {
    unsigned char e_ident[16];
    u16 e_type;
    u16 e_machine;
    u32 e_version;
    u64 e_entry;
    u64 e_phoff;
    u64 e_shoff;
    u32 e_flags;
    u16 e_ehsize;
    u16 e_phentsize;
    u16 e_phnum;
    u16 e_shentsize;
    u16 e_shnum;
    u16 e_shstrndx;
};

struct elf64_phdr {
    u32 p_type;
    u32 p_flags;
    u64 p_offset;
    u64 p_vaddr;
    u64 p_paddr;
    u64 p_filesz;
    u64 p_memsz;
    u64 p_align;
};

struct elf64_dyn {
    i64 d_tag;
    u64 d_val;
};

struct elf64_sym {
    u32 st_name;
    unsigned char st_info;
    unsigned char st_other;
    u16 st_shndx;
    u64 st_value;
    u64 st_size;
};

struct loaded {
    const char *tag;
    u64 base;
    u64 dyn;
    u64 strtab;
    u64 symtab;
    u32 strsz;
    u16 syment;
};

static long syscall3(long n, long a1, long a2, long a3) {
    long ret;
    __asm__ volatile("syscall" : "=a"(ret) : "a"(n), "D"(a1), "S"(a2), "d"(a3) : "rcx", "r11", "memory");
    return ret;
}

static long syscall6(long n, long a1, long a2, long a3, long a4, long a5, long a6) {
    long ret;
    register long r10 __asm__("r10") = a4;
    register long r8 __asm__("r8") = a5;
    register long r9 __asm__("r9") = a6;
    __asm__ volatile("syscall" : "=a"(ret) : "a"(n), "D"(a1), "S"(a2), "d"(a3), "r"(r10), "r"(r8), "r"(r9) : "rcx", "r11", "memory");
    return ret;
}

static int strcmp(const char *a, const char *b) {
    while (*a && *a == *b) {
        a++;
        b++;
    }
    return (unsigned char)*a - (unsigned char)*b;
}

static int str_contains(const char *hay, const char *needle) {
    for (const char *p = hay; *p; p++) {
        const char *a = p;
        const char *b = needle;
        while (*a && *b && *a == *b) {
            a++;
            b++;
        }
        if (*b == 0) return 1;
    }
    return 0;
}

static u64 read_file(const char *path, u64 *out_len) {
    long fd = syscall3(SYS_open, (long)path, O_RDONLY, 0);
    if (fd < 0) return 0;
    u64 cap = 1 << 20;
    u8 *buf = (u8 *)syscall6(SYS_mmap, 0, (long)cap, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if ((long)buf < 0) {
        syscall3(SYS_close, fd, 0, 0);
        return 0;
    }
    u64 total = 0;
    for (;;) {
        long n = syscall3(SYS_read, fd, (long)(buf + total), (long)(cap - total));
        if (n <= 0) break;
        total += (u64)n;
        if (total >= cap) break;
    }
    syscall3(SYS_close, fd, 0, 0);
    if (total == 0) return 0;
    *out_len = total;
    return (u64)buf;
}

static u64 find_dyn(const struct elf64_hdr *eh, u64 base) {
    const struct elf64_phdr *ph = (const struct elf64_phdr *)((const u8 *)eh + eh->e_phoff);
    for (u16 i = 0; i < eh->e_phnum; i++) {
        if (ph[i].p_type == PT_DYNAMIC) return base + ph[i].p_vaddr;
    }
    return 0;
}

static u64 elf_load_base(u64 hdr_addr) {
    const struct elf64_hdr *eh = (const struct elf64_hdr *)hdr_addr;
    u64 min_v = (u64)-1;
    const struct elf64_phdr *ph = (const struct elf64_phdr *)((const u8 *)eh + eh->e_phoff);
    for (u16 i = 0; i < eh->e_phnum; i++) {
        if (ph[i].p_type != PT_LOAD) continue;
        if (ph[i].p_vaddr < min_v) min_v = ph[i].p_vaddr;
    }
    return hdr_addr - (min_v & ~(u64)(PAGE_SIZE - 1));
}

static u64 dyn_end(const struct elf64_hdr *eh, u64 dyn) {
    const struct elf64_phdr *ph = (const struct elf64_phdr *)((const u8 *)eh + eh->e_phoff);
    for (u16 i = 0; i < eh->e_phnum; i++) {
        if (ph[i].p_type == PT_DYNAMIC) return dyn + ph[i].p_memsz;
    }
    return dyn + 512;
}

static void parse_dyn(struct loaded *L, const struct elf64_hdr *eh) {
    u64 dend = dyn_end(eh, L->dyn);
    const struct elf64_dyn *d = (const struct elf64_dyn *)L->dyn;
    while ((u64)d < dend) {
        if (d->d_tag == DT_NULL) break;
        if (d->d_tag == DT_STRTAB) L->strtab = d->d_val;
        if (d->d_tag == DT_SYMTAB) L->symtab = d->d_val;
        if (d->d_tag == DT_STRSZ) L->strsz = (u32)d->d_val;
        if (d->d_tag == DT_SYMENT) L->syment = (u16)d->d_val;
        d++;
    }
}

static int hexval(unsigned char c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'f') return c - 'a' + 10;
    if (c >= 'A' && c <= 'F') return c - 'A' + 10;
    return -1;
}

static int adopt_from_maps(const char *needle, struct loaded *L) {
    u64 maps_len = 0;
    u64 maps = read_file("/proc/self/maps", &maps_len);
    if (!maps) return -1;

    u64 min_start = (u64)-1;
    u64 max_end = 0;
    const char *p = (const char *)maps;
    const char *end = p + maps_len;
    while (p < end) {
        const char *line = p;
        while (p < end && *p != '\n') p++;
        u64 start = 0;
        u64 stop = 0;
        const char *q = line;
        for (;;) {
            int hv = hexval((unsigned char)*q);
            if (hv < 0) break;
            start = start * 16 + (u64)hv;
            q++;
        }
        if (*q != '-') goto next;
        q++;
        for (;;) {
            int hv = hexval((unsigned char)*q);
            if (hv < 0) break;
            stop = stop * 16 + (u64)hv;
            q++;
        }
        while (q < p && *q != '/') q++;
        if (q >= p) goto next;
        if (!str_contains(q, needle)) goto next;
        if (start < min_start) min_start = start;
        if (stop > max_end) max_end = stop;
    next:
        if (p < end) p++;
    }
    if (min_start == (u64)-1) return -2;

    u64 hdr = 0;
    for (u64 a = min_start; a < max_end; a += PAGE_SIZE) {
        if (*(const u32 *)a != ELF_MAGIC) continue;
        const struct elf64_hdr *eh = (const struct elf64_hdr *)a;
        if (eh->e_ident[4] != ELFCLASS64) continue;
        if (eh->e_machine != EM_X86_64) continue;
        hdr = a;
        break;
    }
    if (!hdr) return -3;

    L->tag = needle;
    L->base = elf_load_base(hdr);
    L->dyn = find_dyn((const struct elf64_hdr *)hdr, L->base);
    if (!L->dyn) return -4;
    parse_dyn(L, (const struct elf64_hdr *)hdr);
    return 0;
}

static u64 sym_lookup_ver(const struct loaded *L, const char *name) {
    if (!L->symtab || !L->strtab) return 0;
    u16 ent = L->syment ? L->syment : 24;
    for (u32 i = 0; i < 8192; i++) {
        const struct elf64_sym *s = (const struct elf64_sym *)(L->symtab + (u64)i * ent);
        unsigned char bind = s->st_info >> 4;
        unsigned char type = s->st_info & 0xf;
        if (type != STT_FUNC && type != 0) continue;
        if (bind != STB_GLOBAL && bind != STB_WEAK) continue;
        const char *sn = (const char *)(L->strtab + s->st_name);
        int n = 0;
        while (name[n] && sn[n] == name[n]) n++;
        if (name[n] == 0 && (sn[n] == 0 || sn[n] == '@')) return s->st_value;
    }
    return 0;
}

static u64 find_dyn_in_base(u64 base) {
    for (u64 a = base; a < base + 0x400000; a += PAGE_SIZE) {
        if (*(const u32 *)a != ELF_MAGIC) continue;
        const struct elf64_hdr *eh = (const struct elf64_hdr *)a;
        if (eh->e_ident[4] != ELFCLASS64) continue;
        if (eh->e_machine != EM_X86_64) continue;
        u64 dyn = find_dyn(eh, base);
        if (dyn) return dyn;
    }
    return 0;
}

static u64 sym_walk_export(u64 load_base, const char *export) {
    u64 dyn = find_dyn_in_base(load_base);
    if (!dyn) return 0;

    u64 strtab = 0;
    u64 symtab = 0;
    u32 strsz = 0;
    u16 syment = 24;
    const struct elf64_dyn *d = (const struct elf64_dyn *)dyn;
    for (u32 i = 0; i < 128; i++, d++) {
        if (d->d_tag == DT_NULL) break;
        if (d->d_tag == DT_STRTAB) strtab = d->d_val;
        if (d->d_tag == DT_SYMTAB) symtab = d->d_val;
        if (d->d_tag == DT_STRSZ) strsz = (u32)d->d_val;
        if (d->d_tag == DT_SYMENT) syment = (u16)d->d_val;
    }
    if (!symtab || !strtab) return 0;

    u16 ent = syment ? syment : 24;
    (void)strsz;
    for (u32 i = 0; i < 512; i++) {
        const struct elf64_sym *s = (const struct elf64_sym *)(symtab + (u64)i * ent);
        unsigned char bind = s->st_info >> 4;
        unsigned char type = s->st_info & 0xf;
        if (type != STT_FUNC) continue;
        if (bind != STB_GLOBAL && bind != STB_WEAK) continue;
        const char *sn = (const char *)(strtab + s->st_name);
        if (strcmp(sn, export) != 0) continue;
        return load_base + s->st_value;
    }
    return 0;
}

static struct loaded libs[2];

static void build_open_name(char *nm) {
    nm[0] = (char)('a' + 3);
    nm[1] = (char)('a' + 11);
    nm[2] = (char)('a' + 14);
    nm[3] = (char)('a' + 15);
    nm[4] = (char)('a' + 4);
    nm[5] = (char)('a' + 13);
    nm[6] = 0;
}

void _start(void) {
    if (adopt_from_maps("libc.so.6", &libs[0]) != 0) syscall3(SYS_exit, 1, 0, 0);
    if (adopt_from_maps("libgcc_s.so.1", &libs[1]) != 0) syscall3(SYS_exit, 2, 0, 0);

    char open_nm[8];
    build_open_name(open_nm);
    void *(*openfn)(const char *, int) = (void *(*)(const char *, int))sym_lookup_ver(&libs[0], open_nm);
    if (!openfn) syscall3(SYS_exit, 3, 0, 0);

    void *handle = openfn("./libyoyo_runtime.so", RTLD_LAZY);
    if (!handle) syscall3(SYS_exit, 4, 0, 0);
    u64 load_base = *(u64 *)handle;

    u64 fn = sym_walk_export(load_base, "yoyo_runtime_selfhost_main");
    if (!fn) syscall3(SYS_exit, 5, 0, 0);

    int (*mainfn)(void) = (int (*)(void))fn;
    int ec = mainfn();
    syscall3(SYS_exit, ec, 0, 0);
}