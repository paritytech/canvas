# Dockerfile to build canvas docker image. Uses binary that was build externally.
FROM docker.io/library/ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG VERSION

LABEL io.parity.image.title="canvas" \
    io.parity.image.description="Canvas - A Smart Contracts Parachain" \
    io.parity.image.source="https://github.com/paritytech/canvas/blob/${VCS_REF}/scripts/dockerfiles/canvas_injected.Dockerfile" \
    io.parity.image.url="https://github.com/paritytech/canvas/blob/${VCS_REF}/scripts/dockerfiles/canvas_injected.Dockerfile" \
    io.parity.image.documentation="https://github.com/paritytech/canvas/blob/${VCS_REF}/README.md" \
    io.parity.image.created="${BUILD_DATE}" \
    io.parity.image.version="${VERSION}" \
    io.parity.image.revision="${VCS_REF}" \
    io.parity.image.authors="devops-team@parity.io" \
    io.parity.image.vendor="Parity Technologies" \
    io.parity.image.licenses="GPL-3.0 License"

# show backtraces
ENV RUST_BACKTRACE 1
ENV DEBIAN_FRONTEND=noninteractive

# add non-root user
RUN groupadd -g 1000 user && \
	useradd -u 1000 -g user -s /bin/sh -m user

# switch to non-root user
USER user

COPY --chown=root:root ./canvas /usr/local/bin/

# check if executable works in this container
RUN /usr/local/bin/canvas --version

EXPOSE 30333 9933 9944
ENTRYPOINT ["/usr/local/bin/canvas"]
