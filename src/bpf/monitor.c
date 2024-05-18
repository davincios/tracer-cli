#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <linux/ptrace.h>
#include <linux/sched.h>   // For task_struct

// Define the structure outside the function
struct bpf_tracepoint__syscalls__sys_enter_execve {
    unsigned long long unused;
    long syscall_nr;
    const char **argv;
    const char **envp;
};

SEC("tracepoint/syscalls/sys_enter_execve")
int bpf_prog(struct bpf_tracepoint__syscalls__sys_enter_execve *ctx) {
    char filename[256];
    
    // Read the filename from the context
    bpf_probe_read_kernel_str(filename, sizeof(filename), ctx->argv[0]);

    // Check if the binary is in /usr/bin
    if (__builtin_memcmp(filename, "/usr/bin/", 9) == 0) {
        bpf_printk("Executing command: %s\n", filename);
    }
    return 0;
}

char _license[] SEC("license") = "GPL";
