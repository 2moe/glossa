# syntax=docker/dockerfile:1.16
FROM scratch

COPY glossa-cli.wasm /
ENTRYPOINT [ "/glossa-cli.wasm" ]
