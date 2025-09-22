FROM ubuntu:24.04

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies
RUN apt-get update && apt-get install -y \
    autoconf \
    bison \
    re2c \
    pkg-config \
    libxml2-dev \
    libsqlite3-dev \
    libssl-dev \
    libcurl4-openssl-dev \
    libonig-dev \
    libzip-dev \
    curl \
    ca-certificates \
    make \
    lsb-release \
    software-properties-common \
    gnupg \
    bzip2 \
    && rm -rf /var/lib/apt/lists/*

# Install LLVM/Clang 21 from apt.llvm.org
RUN curl -fSL https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - \
    && echo "deb http://apt.llvm.org/noble/ llvm-toolchain-noble-21 main" > /etc/apt/sources.list.d/llvm.list \
    && apt-get update \
    && apt-get install -y clang-21 lld-21 \
    && update-alternatives --install /usr/bin/cc cc /usr/bin/clang-21 100 \
    && update-alternatives --install /usr/bin/c++ c++ /usr/bin/clang++-21 100 \
    && rm -rf /var/lib/apt/lists/*

ENV CC=clang-21
ENV CXX=clang++-21
ENV LDFLAGS="-fuse-ld=lld-21"

# Download, build, and install PHP 8.4.13
WORKDIR /usr/src
RUN curl -fSL https://www.php.net/distributions/php-8.4.13.tar.bz2 -o php.tar.bz2 \
    && tar -xjf php.tar.bz2 \
    && cd php-8.4.13 \
    && ./buildconf --force \
    && ./configure \
        --prefix=/usr/local \
        --enable-mbstring \
        --with-curl \
        --with-openssl \
        --with-zlib \
        --enable-bcmath \
        --enable-opcache \
    && make -j$(nproc) \
    && make install \
    && cd .. \
    && rm -rf php.tar.bz2

# Install Rust via rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.90.0
ENV PATH="/root/.cargo/bin:${PATH}"

ADD --chmod=777 https://getcomposer.org/download/2.8.12/composer.phar /usr/local/bin/composer

# Verify installations
RUN php --version && php-config --version && rustc --version && clang-21 --version && ld.lld-21 --version

WORKDIR /usr/local/src/ext/

CMD ["/bin/bash"]
