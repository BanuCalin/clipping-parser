# CircleCI rust image with stable
FROM rustlang/rust:nightly-buster-slim
# Install nightly toolchain
#RUN rustup toolchain install nightly
RUN apt-get update -y
RUN apt-get install -y curl gnupg coreutils
RUN rustup default nightly
RUN rustup component add llvm-tools-preview
RUN cargo install grcov
RUN cargo install rustfilt
RUN cargo install cargo-binutils
RUN cargo install cargo2junit
