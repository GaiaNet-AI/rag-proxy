# A proxy server for RAG prompts

This is a alternative to the `rag-api-server` project. It is a web server that

* Takes an OpenAI API compatible request
* Parse the last user message
* Compute an embedding vector for the message using a local `llama-api-server` on the Gaia node (eg., `http://localhost:8080`)
* Search for context from the local Qdrant instance (eg., `http://localhost:6333`)
* Modify the request system prompt with the RAG search result
* Send the request to the local `llama-api-server` on the Gaia node (eg., `http://localhost:8080`)
* Respond with the `llama-api-server` response

## How to use it

Step 1: Start a Gaia node without a knowledge snapshot. Eg., https://github.com/GaiaNet-AI/node-configs/tree/main/llama-3.1-8b-instruct

Step 2: Start the Qdrant server on the node manually.

```
nohup ~/gaianet/bin/qdrant &
```

Step 3: Load a few knowledge snapshots. Eg.,

```
curl -s -X POST http://localhost:6333/collections/knowledge_base_01/snapshots/upload?priority=snapshot \
    -H 'Content-Type:multipart/form-data' \
    -F 'snapshot=@knowledge_base_01.snapshot'
```

Step 4: Start a RAG proxy server for each knowledge snapshot.

```
nohup rag-proxy --port 8181 --vector-collection-name knowledge_base_01 &
```

Step 5: Map the RAG proxy's server port to a domain name in the `~/gaianet/gaianet-domain/frpc.toml` file

```
[[proxies]]
name = "kb01.us.gaianet.network"
type = "http"
localPort = 8181
subdomain = "kb01"
```

Step 6: Start the `frpc` service.

```
nohup ~/gaianet/bin/frpc -c ~/gaianet/gaianet-domain/frpc.toml &
```
