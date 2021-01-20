FROM rust AS planner
WORKDIR lab
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json
# Our first FROM statement declares the build environment.
FROM rust AS cacher
WORKDIR lab
RUN cargo install cargo-chef
COPY --from=planner /lab/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR lab
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /lab/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin capricorn-lab
# Now, we need to build our _real_ Docker container, copying in `rust-actix-example`.
FROM alpine:latest
# RUN apk --no-cache add ca-certifißcates
COPY --from=builder  /lab/target/release/capricorn-lab  /usr/local/bin/collab
CMD ["collab"]