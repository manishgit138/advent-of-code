FROM fredrikfornwall/rust-static-builder:latest
ADD . /build/
RUN /root/build.sh
RUN ls -lha /build/target/x86_64-unknown-linux-musl
RUN ls -lha /build/target/x86_64-unknown-linux-musl/release/
FROM scratch
COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/advent-of-code-grpc-server /
EXPOSE 50051
ENTRYPOINT ["/advent-of-code-grpc-server"]
