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
  -b, --base-url <BASE_URL>                          [default: http://localhost:8080/v1]
      --embedding-model-name <EMBEDDING_MODEL_NAME>  [default: embedding]
      --embedding-base-url <EMBEDDING_BASE_URL>      [default: http://localhost:8080/v1]
      --vss-url <VSS_URL>                            [default: http://localhost:6663]
  -l, --vss-limit <VSS_LIMIT>                        [default: 3]
  -c, --vss-score-threshold <VSS_SCORE_THRESHOLD>    
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
* --vss-url
    This parameter specifies the URL of the vector database. (qdrant or rusqlite-vss)
* --vss-limit
    This parameter is used to limit the number of search results returned by the vector database.
* --vss-score-threshold
    This parameter is the score-threshold passed to the vector database during the search.

```bash
# run
wasmedge target/wasm32-wasi/release/rag-proxy.wasm
```