#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <linux/ptrace.h>
#include <linux/sched.h>   // For task_struct

#define LOG_SIZE 256

struct log_entry {
    char filename[LOG_SIZE];
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 4096 * 256); // Adjust the size as needed
} ringbuf_map SEC(".maps");

struct bpf_tracepoint__syscalls__sys_enter_execve {
    unsigned long long unused;
    long syscall_nr;
    const char **argv;
    const char **envp;
};

SEC("tracepoint/syscalls/sys_enter_execve")
int bpf_prog(struct bpf_tracepoint__syscalls__sys_enter_execve *ctx) {
    struct log_entry *entry;
    entry = bpf_ringbuf_reserve(&ringbuf_map, sizeof(*entry), 0);
    if (!entry) {
        return 0;
    }

    // Read the filename from the context
    bpf_probe_read_user_str(entry->filename, sizeof(entry->filename), ctx->argv[0]);

    // Check if the binary is in /usr/bin
    if (__builtin_memcmp(entry->filename, "/usr/bin/", 9) == 0) {
        bpf_ringbuf_submit(entry, 0);
    } else {
        bpf_ringbuf_discard(entry, 0);
    }

    return 0;
}

char _license[] SEC("license") = "GPL";
