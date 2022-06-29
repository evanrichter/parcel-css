# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzz && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/parse /parse && \
    echo done

RUN echo building non-instrumented harnesses && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzz --disable-instrumentation && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/parse /parse_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /parse  /parse_no_inst /
