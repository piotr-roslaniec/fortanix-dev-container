FROM ubuntu:bionic

# This Dockerfile adds a non-root 'vscode' user with sudo access. However, for Linux,
# this user's GID/UID must match your local user UID/GID to avoid permission issues
# with bind mounts. Update USER_UID / USER_GID if yours is not 1000. See
# https://aka.ms/vscode-remote/containers/non-root-user for details.
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -yq --no-install-recommends ca-certificates build-essential ocaml ocamlbuild \
    automake autoconf libtool wget python libssl-dev libssl-dev libcurl4-openssl-dev protobuf-compiler git \
    libprotobuf-dev alien cmake debhelper uuid-dev libxml2-dev lsb-release

COPY install-psw.patch ./

RUN git clone -b sgx_2.6 --depth 1 https://github.com/intel/linux-sgx

RUN cd linux-sgx && \
    patch -p1 -i ../install-psw.patch && \
    ./download_prebuilt.sh 2> /dev/null && \
    make -s -j$(nproc) && make -s -j$(nproc) sdk_install_pkg deb_pkg && \
    ./linux/installer/bin/sgx_linux_x64_sdk_2.6.100.51363.bin --prefix=/opt/intel && \
    cd linux/installer/deb/ && \
    dpkg -i libsgx-urts_2.6.100.51363-bionic1_amd64.deb libsgx-enclave-common_2.6.100.51363-bionic1_amd64.deb && \
    cd ../../../.. && rm -rf linux-sgx/

# Configure apt and install packages
RUN apt-get update && \
    apt-get -y install --no-install-recommends apt-utils dialog git curl iproute2 procps lsb-release && \
    # Create a non-root user to use if preferred - see https://aka.ms/vscode-remote/containers/non-root-user.
    groupadd --gid $USER_GID $USERNAME  && \
    useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME && \
    # [Optional] Add sudo support for the non-root user
    apt-get install -y sudo && \
    echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME && \
    chmod 0440 /etc/sudoers.d/$USERNAME

# Switch to non-root user
USER ${USERNAME}

# Install Rust components
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

# Install Fortanix EDP

# Install Rust
RUN rustup update && \
    rustup default nightly && \
    rustup target add x86_64-fortanix-unknown-sgx --toolchain nightly && \
    rustup component add rls rust-analysis rust-src rustfmt clippy

# Install Fortanix EDP utilities
RUN sudo apt-get -y install pkg-config libssl-dev protobuf-compiler && \
    cargo install fortanix-sgx-tools sgxs-tools

# Configure Cargo integration with Fortanix EDP
RUN printf '[target.x86_64-fortanix-unknown-sgx]\nrunner = "ftxsgx-runner-cargo"\n' >> /home/${USERNAME}/.cargo/config

# Clean up
RUN sudo apt-get autoremove -y && \
    sudo apt-get clean -y && \
    sudo rm -rf /var/lib/apt/lists/*

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=

# Run AESM service in the background
RUN printf '\nsudo LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/opt/intel/libsgx-enclave-common/aesm/ /opt/intel/libsgx-enclave-common/aesm/aesm_service &\n' >> /home/${USERNAME}/.bashrc

CMD ["/bin/bash"]
