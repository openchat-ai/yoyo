/* Stage 11-B / post-v1.0 OW-IAT: syscall-only ELF manual-map trampoline.
 * Replaces dlopen with open/read/mmap + in-process PT_LOAD map (mirrors Win pe_manual_map).
 * Loads cwd ./libyoyo_runtime.so after host libc/libgcc (hardcoded paths).
 * Rebuild: scripts/build-linux-h00-tramp.sh
 */
typedef unsigned long long u64;
typedef unsigned int u32;
typedef unsigned short u16;
typedef unsigned char u8;
typedef long long i64;

#define SYS_read 0
#define SYS_write 1
#define SYS_open 2
#define SYS_close 3
#define SYS_mmap 9
#define SYS_mprotect 10
#define SYS_exit 60

#define O_RDONLY 0
#define MAP_PRIVATE 0x02
#define MAP_ANONYMOUS 0x20
#define PROT_READ 1
#define PROT_WRITE 2
#define PROT_EXEC 4

#define PT_LOAD 1
#define PT_DYNAMIC 2
#define DT_NULL 0
#define DT_NEEDED 1
#define DT_STRTAB 5
#define DT_SYMTAB 6
#define DT_RELA 7
#define DT_RELASZ 8
#define DT_RELAENT 9
#define DT_STRSZ 10
#define DT_SYMENT 11
#define DT_PLTGOT 3
#define DT_JMPREL 23
#define DT_PLTRELSZ 2

#define ELF_MAGIC 0x464c457fU
#define EM_X86_64 62
#define ELFCLASS64 2
#define STT_FUNC 2
#define STB_GLOBAL 1
#define STB_WEAK 2
#define R_X86_64_GLOB_DAT 6
#define R_X86_64_JUMP_SLOT 7
#define R_X86_64_RELATIVE 8
#define R_X86_64_DTPMOD64 37

#define PAGE_SIZE 4096
#define PAGE_MASK (~(u64)(PAGE_SIZE - 1))

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

struct elf64_rela {
    u64 r_offset;
    u64 r_info;
    i64 r_addend;
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
    const char *path;
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
    while (*a && *a == *b) { a++; b++; }
    return (unsigned char)*a - (unsigned char)*b;
}

static int strncmp(const char *a, const char *b, int n) {
    for (int i = 0; i < n; i++) {
        if (a[i] != b[i]) return (unsigned char)a[i] - (unsigned char)b[i];
        if (a[i] == 0) return 0;
    }
    return 0;
}

static void *my_memset(void *s, int c, u64 n) {
    unsigned char *p = (unsigned char *)s;
    for (u64 i = 0; i < n; i++) p[i] = (unsigned char)c;
    return s;
}

static void *my_memcpy(void *d, const void *s, u64 n) {
    unsigned char *dd = (unsigned char *)d;
    const unsigned char *ss = (const unsigned char *)s;
    for (u64 i = 0; i < n; i++) dd[i] = ss[i];
    return d;
}

static u64 align_up(u64 v, u64 a) { return (v + a - 1) & ~(a - 1); }

static u64 read_file(const char *path, u64 *out_len) {
    long fd = syscall3(SYS_open, (long)path, O_RDONLY, 0);
    if (fd < 0) return 0;
    u64 cap = 8 << 20;
    u8 *buf = (u8 *)syscall6(SYS_mmap, 0, (long)cap, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if ((long)buf < 0) { syscall3(SYS_close, fd, 0, 0); return 0; }
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

typedef unsigned char u8;

static u64 find_dyn(const struct elf64_hdr *eh, u64 base) {
    const struct elf64_phdr *ph = (const struct elf64_phdr *)((const u8 *)eh + eh->e_phoff);
    for (u16 i = 0; i < eh->e_phnum; i++) {
        if (ph[i].p_type == PT_DYNAMIC)
            return base + ph[i].p_vaddr;
    }
    return 0;
}

static void parse_dyn(struct loaded *L) {
    const struct elf64_dyn *d = (const struct elf64_dyn *)L->dyn;
    for (;;) {
        if (d->d_tag == DT_NULL) break;
        if (d->d_tag == DT_STRTAB) L->strtab = L->base + d->d_val;
        if (d->d_tag == DT_SYMTAB) L->symtab = L->base + d->d_val;
        if (d->d_tag == DT_STRSZ) L->strsz = (u32)d->d_val;
        if (d->d_tag == DT_SYMENT) L->syment = (u16)d->d_val;
        d++;
    }
}

static u64 sym_lookup(const struct loaded *L, const char *name) {
    if (!L->symtab || !L->strtab) return 0;
    u16 ent = L->syment ? L->syment : 24;
    for (u32 i = 0; i < 8192; i++) {
        const struct elf64_sym *s = (const struct elf64_sym *)(L->symtab + (u64)i * ent);
        unsigned char info = s->st_info;
        unsigned char bind = info >> 4;
        unsigned char type = info & 0xf;
        if (type != STT_FUNC && type != 0) continue;
        if (bind != STB_GLOBAL && bind != STB_WEAK) continue;
        const char *sn = (const char *)(L->strtab + s->st_name);
        if (strcmp(sn, name) != 0) continue;
        return L->base + s->st_value;
    }
    return 0;
}

static u64 sym_lookup_ver(const struct loaded *L, const char *name) {
    /* match "name" or "name@..." */
    if (!L->symtab || !L->strtab) return 0;
    u16 ent = L->syment ? L->syment : 24;
    for (u32 i = 0; i < 8192; i++) {
        const struct elf64_sym *s = (const struct elf64_sym *)(L->symtab + (u64)i * ent);
        unsigned char info = s->st_info;
        unsigned char bind = info >> 4;
        unsigned char type = info & 0xf;
        if (type != STT_FUNC && type != 0) continue;
        if (bind != STB_GLOBAL && bind != STB_WEAK) continue;
        const char *sn = (const char *)(L->strtab + s->st_name);
        int n = 0;
        while (name[n] && sn[n] == name[n]) n++;
        if (name[n] == 0 && (sn[n] == 0 || sn[n] == '@')) return L->base + s->st_value;
    }
    return 0;
}

static int map_elf(const u8 *file, u64 file_len, struct loaded *L, const struct loaded *deps, int ndeps) {
    if (file_len < sizeof(struct elf64_hdr)) return -1;
    const struct elf64_hdr *eh = (const struct elf64_hdr *)file;
    if (*(const u32 *)eh->e_ident != ELF_MAGIC) return -1;
    if (eh->e_ident[4] != ELFCLASS64) return -1;
    if (eh->e_machine != EM_X86_64) return -1;

    u64 min_v = (u64)-1, max_v = 0;
    const struct elf64_phdr *ph = (const struct elf64_phdr *)(file + eh->e_phoff);
    for (u16 i = 0; i < eh->e_phnum; i++) {
        if (ph[i].p_type != PT_LOAD) continue;
        if (ph[i].p_vaddr < min_v) min_v = ph[i].p_vaddr;
        u64 end = ph[i].p_vaddr + ph[i].p_memsz;
        if (end > max_v) max_v = end;
    }
    u64 map_addr = min_v & PAGE_MASK;
    u64 map_len = align_up(max_v - map_addr, PAGE_SIZE);
    u64 map = (u64)syscall6(SYS_mmap, 0, (long)map_len, PROT_READ | PROT_WRITE | PROT_EXEC,
                             MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if ((long)map < 0) return -1;
    my_memset((void *)map, 0, map_len);
    u64 base = map - map_addr;

    for (u16 i = 0; i < eh->e_phnum; i++) {
        if (ph[i].p_type != PT_LOAD) continue;
        u64 dest = base + ph[i].p_vaddr;
        u64 filesz = ph[i].p_filesz;
        if (ph[i].p_offset + filesz > file_len) return -1;
        my_memcpy((void *)dest, file + ph[i].p_offset, filesz);
        if (ph[i].p_memsz > filesz)
            my_memset((void *)(dest + filesz), 0, ph[i].p_memsz - filesz);
    }

    L->base = base;
    L->dyn = find_dyn((const struct elf64_hdr *)(base + map_addr), base);
    if (!L->dyn) return -4;
    parse_dyn(L);

    /* RELA */
    u64 rela = 0, relasz = 0, relaent = 24;
    const struct elf64_dyn *d = (const struct elf64_dyn *)L->dyn;
    for (;;) {
        if (d->d_tag == DT_NULL) break;
        if (d->d_tag == DT_RELA) rela = L->base + d->d_val;
        if (d->d_tag == DT_RELASZ) relasz = d->d_val;
        if (d->d_tag == DT_RELAENT) relaent = d->d_val;
        d++;
    }
    u64 nrela = relaent ? relasz / relaent : 0;
    for (u64 ri = 0; ri < nrela; ri++) {
        const struct elf64_rela *r = (const struct elf64_rela *)(rela + ri * relaent);
        u32 type = (u32)(r->r_info & 0xffffffff);
        u32 sym = (u32)(r->r_info >> 32);
        u64 *where = (u64 *)(L->base + r->r_offset);
        if (type == R_X86_64_RELATIVE) {
            *where = L->base + (u64)r->r_addend;
        } else if (type == R_X86_64_GLOB_DAT || type == R_X86_64_JUMP_SLOT) {
            u64 val = 0;
            const struct elf64_sym *s = (const struct elf64_sym *)(L->symtab + (u64)sym * (L->syment ? L->syment : 24));
            const char *name = (const char *)(L->strtab + s->st_name);
            for (int di = 0; di < ndeps; di++) {
                val = sym_lookup_ver(&deps[di], name);
                if (val) break;
            }
            if (!val) return -2;
            *where = val + (u64)r->r_addend;
        } else if (type == R_X86_64_DTPMOD64) {
            *where = L->base;
        }
    }

    /* JMPREL */
    u64 jmprel = 0, pltrelsz = 0;
    d = (const struct elf64_dyn *)L->dyn;
    for (;;) {
        if (d->d_tag == DT_NULL) break;
        if (d->d_tag == DT_JMPREL) jmprel = L->base + d->d_val;
        if (d->d_tag == DT_PLTRELSZ) pltrelsz = d->d_val;
        d++;
    }
    u64 nplt = relaent ? pltrelsz / relaent : 0;
    for (u64 ri = 0; ri < nplt; ri++) {
        const struct elf64_rela *r = (const struct elf64_rela *)(jmprel + ri * relaent);
        u32 type = (u32)(r->r_info & 0xffffffff);
        if (type != R_X86_64_JUMP_SLOT) continue;
        u32 sym = (u32)(r->r_info >> 32);
        u64 *where = (u64 *)(L->base + r->r_offset);
        const struct elf64_sym *s = (const struct elf64_sym *)(L->symtab + (u64)sym * (L->syment ? L->syment : 24));
        const char *name = (const char *)(L->strtab + s->st_name);
        u64 val = 0;
        for (int di = 0; di < ndeps; di++) {
            val = sym_lookup_ver(&deps[di], name);
            if (val) break;
        }
        if (!val) return -3;
        *where = val + (u64)r->r_addend;
    }

    return 0;
}

static struct loaded libs[4];
static int nlibs;

static int load_one(const char *path) {
    u64 len = 0;
    u64 file = read_file(path, &len);
    if (!file) return -1;
    struct loaded *L = &libs[nlibs];
    L->path = path;
    int rc = map_elf((const u8 *)file, len, L, libs, nlibs);
    if (rc == 0) nlibs++;
    return rc;
}

void _start(void) {
    const char *libc_path = "/lib/x86_64-linux-gnu/libc.so.6";
    const char *gcc_path = "/lib/x86_64-linux-gnu/libgcc_s.so.1";
    const char *rt_path = "./libyoyo_runtime.so";

    if (load_one(libc_path) != 0) syscall3(SYS_exit, 1, 0, 0);
    if (load_one(gcc_path) != 0) syscall3(SYS_exit, 2, 0, 0);
    if (load_one(rt_path) != 0) syscall3(SYS_exit, 3, 0, 0);

    struct loaded *rt = &libs[nlibs - 1];
    u64 fn = sym_lookup(rt, "yoyo_runtime_selfhost_main");
    if (!fn) syscall3(SYS_exit, 4, 0, 0);

    int (*mainfn)(void) = (int (*)(void))fn;
    int ec = mainfn();
    syscall3(SYS_exit, ec, 0, 0);
}
