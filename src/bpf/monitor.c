#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <linux/ptrace.h>

SEC("tracepoint/syscalls/sys_enter_execve")
int bpf_prog(struct bpf_tracepoint__syscalls__sys_enter_execve *ctx) {
    char filename[256];
    bpf_probe_read_str(filename, sizeof(filename), (void *)ctx->filename);

    // Check if the binary is in /usr/bin
    if (bpf_strncmp(filename, "/usr/bin/", 9) == 0) {
        bpf_printk("Executing command: %s\n", filename);
    }
    return 0;
}

char _license[] SEC("license") = "GPL";
