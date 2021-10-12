# Dockerfile to build canvas docker image. Uses binary that was build externally.
FROM docker.io/library/ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG VERSION

LABEL org.opencontainers.image.title="canvas" \
    org.opencontainers.image.description="Canvas - A Smart Contracts Parachain" \
    org.opencontainers.image.source="https://github.com/paritytech/canvas/blob/${VCS_REF}/scripts/dockerfiles/canvas_injected.Dockerfile" \
    org.opencontainers.image.url="https://github.com/paritytech/canvas/blob/${VCS_REF}/scripts/dockerfiles/canvas_injected.Dockerfile" \
    org.opencontainers.image.documentation="https://github.com/paritytech/canvas/blob/${VCS_REF}/README.md" \
    org.opencontainers.image.created="${BUILD_DATE}" \
    org.opencontainers.image.version="${VERSION}" \
    org.opencontainers.image.revision="${VCS_REF}" \
    org.opencontainers.image.authors="devops-team@parity.io" \
    org.opencontainers.image.vendor="Parity Technologies" \
    org.opencontainers.image.licenses="GPL-3.0 License"

# show backtraces
ENV RUST_BACKTRACE 1
ENV DEBIAN_FRONTEND=noninteractive

RUN set -eux; \
	apt-get update && \
	apt-get install -y --no-install-recommends \
        ca-certificates && \
    update-ca-certificates && \
	groupadd -g 1000 user && \
	useradd -u 1000 -g user -s /bin/sh -m user && \
	# apt clean up
	apt-get autoremove -y && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*

# switch to non-root user
USER user

COPY --chown=root:root ./canvas /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/canvas"]
