#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != Linux ]]; then
    echo 'Run this check inside the Linux devcontainer.' >&2
    exit 1
fi

failed=0
printf 'Linux %s (%s)\n\n' "$(uname -r)" "$(uname -m)"
for tool in cargo rustc just node npm npx vi vim gdb lldb perf heaptrack heaptrack_print valgrind strace ltrace fio bpftrace trace-cmd ublk; do
    if command -v "$tool" >/dev/null 2>&1; then
        printf '  OK    %s\n' "$tool"
    else
        printf '  FAIL  %s is missing\n' "$tool"
        failed=1
    fi
done

if cargo nextest --version; then
    printf '  OK    cargo-nextest\n'
else
    failed=1
fi

if [[ -c /dev/ublk-control && -r /dev/ublk-control && -w /dev/ublk-control ]]; then
    printf '  OK    /dev/ublk-control is accessible\n'
else
    printf '  FAIL  /dev/ublk-control is unavailable; load ublk_drv in the Linux VM.\n'
    failed=1
fi

if [[ -r /proc/sys/kernel/io_uring_disabled ]] && [[ "$(</proc/sys/kernel/io_uring_disabled)" == 2 ]]; then
    printf '  FAIL  io_uring is disabled by the Linux VM kernel.\n'
    failed=1
fi

if perf stat -e task-clock -- true >/dev/null 2>&1; then
    printf '  OK    perf software events work\n'
else
    printf '  FAIL  perf events are unavailable; check container privileges and VM perf_event_paranoid.\n'
    failed=1
fi

if perf stat -e cycles -- true >/dev/null 2>&1; then
    printf '  OK    hardware performance counters work\n'
else
    printf '  INFO  Hardware counters are unavailable; use perf -e cpu-clock or -e task-clock.\n'
fi

exit "$failed"
