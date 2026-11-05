FROM rust:1-trixie

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        autoconf autoconf-archive automake build-essential cmake libtool pkg-config \
        clang lld lldb llvm libclang-dev libssl-dev liburing-dev \
        gdb gdbserver linux-perf heaptrack valgrind strace ltrace \
        bpftrace trace-cmd fio kmod udev util-linux e2fsprogs xfsprogs \
        ca-certificates curl git jq just ripgrep shellcheck \
        bash-completion less man-db manpages manpages-dev procps psmisc python3 sudo \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add clippy rustfmt rust-src

ARG NEXTEST_VERSION=0.9
RUN case "$(dpkg --print-architecture)" in \
        arm64) nextest_platform=linux-arm ;; \
        amd64) nextest_platform=linux ;; \
        *) echo 'Unsupported nextest architecture' >&2; exit 1 ;; \
    esac \
    && curl -fsSL "https://get.nexte.st/${NEXTEST_VERSION}/${nextest_platform}" \
        | tar -xz -C /usr/local/cargo/bin \
    && cargo nextest --version

ARG UBLKSRV_VERSION=v1.9
RUN git clone --depth 1 --branch "${UBLKSRV_VERSION}" https://github.com/ublk-org/ublksrv.git /tmp/ublksrv \
    && cd /tmp/ublksrv \
    && autoreconf -fi \
    && ./configure --without-libnfs --without-libiscsi --without-gnutls \
    && make -j2 \
    && make install \
    && ldconfig \
    && rm -rf /tmp/ublksrv

RUN apt-get update \
    && apt-get install -y --no-install-recommends locales \
    && echo 'en_US.UTF-8 UTF-8' > /etc/locale.gen \
    && locale-gen \
    && rm -rf /var/lib/apt/lists/*

ENV LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 RUST_BACKTRACE=1
WORKDIR /workspaces/netblk
