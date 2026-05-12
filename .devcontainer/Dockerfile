FROM mcr.microsoft.com/devcontainers/rust:latest

# Install LLVM dependencies
RUN apt-get update && apt-get install -y \
    lsb-release \
    wget \
    gnupg \
    && rm -rf /var/lib/apt/lists/*

# Install LLVM using the official LLVM apt script
RUN wget https://apt.llvm.org/llvm.sh && \
    chmod +x llvm.sh && \
    ./llvm.sh 22 all && \
    rm llvm.sh

# Set LLVM sysroot for inkwell/llvm-sys
ENV LLVM_SYS_221_PREFIX=/usr/lib/llvm-22