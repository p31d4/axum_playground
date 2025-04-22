FROM messense/rust-musl-cross:x86_64-musl as builder

ARG PROJECT

WORKDIR /axum_workdir

# Copy src code
ADD ${PROJECT} .

# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# -----------------------------------------------------------------------------
# Create a new image with a minimal image
#FROM scratch
FROM alpine

ARG PROJECT

COPY --from=builder /axum_workdir/target/x86_64-unknown-linux-musl/release/${PROJECT} /${PROJECT}
ENTRYPOINT ["/bin/sh"]
EXPOSE 3000
