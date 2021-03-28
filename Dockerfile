FROM rust as server-planner
WORKDIR /deltachess-server
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef 
COPY server .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as server-cacher
WORKDIR /deltachess-server
RUN cargo install cargo-chef
COPY --from=server-planner /deltachess-server/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as server-builder
WORKDIR /deltachess-server
COPY server .
# Copy over the cached dependencies
COPY --from=server-cacher /deltachess-server/target target
COPY --from=server-cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin deltachess-server

FROM node as client-cacher
WORKDIR /deltachess-client
COPY client/package.json .
COPY client/yarn.lock .
RUN yarn

FROM node as client-builder
WORKDIR /deltachess-client
COPY client .
COPY --from=client-cacher /deltachess-client/node_modules node_modules
RUN yarn build

FROM nginx as runtime
WORKDIR /deltachess
COPY --from=server-builder /deltachess-server/target/release/deltachess-server /usr/local/bin
COPY --from=client-builder /detlachess-client/public/* /usr/share/nginx
COPY nginx/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 8080
CMD /usr/local/bin/deltachess-server & nginx 