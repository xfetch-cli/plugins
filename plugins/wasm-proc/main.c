/*
 * wasm-proc: xfetch WebAssembly plugin in freestanding C.
 *
 * Reads /proc through the sandboxed filesystem capability (the manifest
 * preopens /proc read-only) and reports load average, memory usage and
 * uptime. No wasi-libc is used: the only imports are the WASI preview 1
 * functions it actually calls.
 */

typedef unsigned int u32;
typedef unsigned long long u64;
typedef unsigned char u8;

__attribute__((import_module("wasi_snapshot_preview1"), import_name("fd_read")))
extern u32 fd_read(u32 fd, const void *iovs, u32 iovs_len, u32 *nread);

__attribute__((import_module("wasi_snapshot_preview1"), import_name("fd_write")))
extern u32 fd_write(u32 fd, const void *iovs, u32 iovs_len, u32 *nwritten);

__attribute__((import_module("wasi_snapshot_preview1"), import_name("fd_close")))
extern u32 fd_close(u32 fd);

__attribute__((import_module("wasi_snapshot_preview1"), import_name("path_open")))
extern u32 path_open(u32 fd, u32 dirflags, const u8 *path, u32 path_len, u32 oflags,
                     u64 rights_base, u64 rights_inheriting, u32 fdflags, u32 *opened_fd);

#define FD_READ_RIGHT 2ULL
#define PREOPEN_FD 3

typedef struct {
    u8 *buf;
    u32 len;
} iovec;

static u8 filebuf[8192];
static u8 output[384];

/* Reads an allowlisted /proc file into filebuf, NUL terminated. */
static u32 read_proc(const char *name, u32 name_len) {
    u32 fd = 0;
    if (path_open(PREOPEN_FD, 0, (const u8 *)name, name_len, 0, FD_READ_RIGHT, 0, 0, &fd) != 0) {
        return 0;
    }
    iovec io = {filebuf, sizeof(filebuf) - 1};
    u32 nread = 0;
    if (fd_read(fd, &io, 1, &nread) != 0) {
        fd_close(fd);
        return 0;
    }
    fd_close(fd);
    filebuf[nread] = 0;
    return nread;
}

static u32 append(u32 pos, const char *s) {
    while (*s) {
        output[pos++] = (u8)*s++;
    }
    return pos;
}

static u32 append_u32(u32 pos, u32 value) {
    char digits[10];
    u32 count = 0;
    if (value == 0) {
        digits[count++] = '0';
    }
    while (value > 0) {
        digits[count++] = (char)('0' + value % 10);
        value /= 10;
    }
    while (count > 0) {
        output[pos++] = (u8)digits[--count];
    }
    return pos;
}

/* Appends a kibibyte count as GiB with one decimal (e.g. 31.2). */
static u32 append_gib(u32 pos, u32 kib) {
    u32 tenths = (kib * 10) / 1048576;
    pos = append_u32(pos, tenths / 10);
    output[pos++] = '.';
    output[pos++] = (u8)('0' + tenths % 10);
    return append(pos, " GiB");
}

static int is_digit(u8 c) { return c >= '0' && c <= '9'; }

/* Finds `prefix` in filebuf and returns the first integer after it. */
static u32 parse_field(const char *prefix, u32 prefix_len) {
    for (u32 i = 0; filebuf[i] != 0; i++) {
        u32 match = 1;
        for (u32 j = 0; j < prefix_len; j++) {
            if (filebuf[i + j] != (u8)prefix[j]) {
                match = 0;
                break;
            }
        }
        if (!match) {
            continue;
        }
        u32 k = i + prefix_len;
        while (filebuf[k] != 0 && !is_digit(filebuf[k])) {
            k++;
        }
        u32 value = 0;
        while (is_digit(filebuf[k])) {
            value = value * 10 + (u32)(filebuf[k] - '0');
            k++;
        }
        return value;
    }
    return 0;
}

__attribute__((export_name("_start")))
void _start(void) {
    u32 pos = 0;
    pos = append(pos, "{\"lines\":[");

    u32 n = read_proc("loadavg", 7);
    if (n > 0) {
        pos = append(pos, "\"load: ");
        u32 spaces = 0;
        for (u32 i = 0; i < n && spaces < 3; i++) {
            if (filebuf[i] == ' ') {
                spaces++;
                if (spaces == 3) {
                    break;
                }
            }
            output[pos++] = filebuf[i];
        }
        pos = append(pos, "\",");
    }

    n = read_proc("uptime", 6);
    if (n > 0) {
        u32 seconds = 0;
        for (u32 i = 0; is_digit(filebuf[i]); i++) {
            seconds = seconds * 10 + (u32)(filebuf[i] - '0');
        }
        u32 days = seconds / 86400;
        u32 hours = (seconds % 86400) / 3600;
        u32 minutes = (seconds % 3600) / 60;
        pos = append(pos, "\"uptime: ");
        if (days > 0) {
            pos = append_u32(pos, days);
            pos = append(pos, "d ");
        }
        pos = append_u32(pos, hours);
        pos = append(pos, "h ");
        pos = append_u32(pos, minutes);
        pos = append(pos, "m\",");
    }

    n = read_proc("meminfo", 7);
    if (n > 0) {
        u32 total = parse_field("MemTotal:", 9);
        u32 available = parse_field("MemAvailable:", 13);
        if (total > 0) {
            u32 used = total > available ? total - available : 0;
            pos = append(pos, "\"memory: ");
            pos = append_u32(pos, used * 100 / total);
            pos = append(pos, "% used (");
            pos = append_gib(pos, used);
            pos = append(pos, " / ");
            pos = append_gib(pos, total);
            pos = append(pos, ")\"");
        }
    }

    /* Trim a possible trailing comma. */
    if (output[pos - 1] == ',') {
        pos--;
    }
    pos = append(pos, "]}");

    iovec out = {output, pos};
    u32 written = 0;
    fd_write(1, &out, 1, &written);
}
