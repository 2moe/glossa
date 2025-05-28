# syntax=docker/dockerfile:1
FROM scratch

COPY glossa-cli.wasm /
ENTRYPOINT [ "/glossa-cli.wasm" ]
