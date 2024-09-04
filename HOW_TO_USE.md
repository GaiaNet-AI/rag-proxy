# Build
``` bash
# debug
cargo build

# release
cargo build --release
```

# Run
```bash
# help
wasmedge target/wasm32-wasi/release/rag-proxy.wasm --help

# output

Usage: rag-proxy.wasm [OPTIONS]

Options:
      --lister-addr <LISTER_ADDR>                    [default: 0.0.0.0:8181]
  -b, --base-url <BASE_URL>                          [default: http://localhost:8080/v1]
      --embedding-model-name <EMBEDDING_MODEL_NAME>  [default: embedding]
      --embedding-base-url <EMBEDDING_BASE_URL>      [default: http://localhost:8080/v1]
      --vss-config <VSS_CONFIG>                      [default: ./config.json]
  -h, --help                                         Print help
  -V, --version                                      Print version
```
* --base-url
    This parameter is used to specify the URL of the chat server (llamaedge-api-server).
* --embedding-model-name
    This parameter is used to specify the name of the embedding model.
* --embedding-base-url
    This parameter is used to specify the URL of the embedding server.
    That means chat and embedding can be two separate services.
* --vss-config
    This parameter specifies the config file path of the vector database. (qdrant or rusqlite-vss)

```bash
# run
wasmedge target/wasm32-wasi/release/rag-proxy.wasm
```