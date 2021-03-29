FROM rust as planner
WORKDIR /deltachess-server
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as cacher
WORKDIR /deltachess-server
RUN cargo install cargo-chef
COPY --from=planner /deltachess-server/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR /deltachess-server
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /deltachess-server/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin deltachess-server

FROM rust as runtime
WORKDIR /deltachess-server
COPY --from=builder /deltachess-server/target/release/deltachess-server /usr/local/bin
EXPOSE 5000
ENTRYPOINT ["/usr/local/bin/deltachess-server"]