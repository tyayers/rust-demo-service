####################################################################################################
## Builder
####################################################################################################
FROM rust:1.61.0 AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=rustservice
ENV UID=10001

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"


WORKDIR /rustservice

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /rustservice

# Copy our build
COPY --from=builder /rustservice/target/release/rustservice ./

# Use an unprivileged user.
USER rustservice:rustservice
EXPOSE 8080

CMD ["/rustservice/rustservice"]