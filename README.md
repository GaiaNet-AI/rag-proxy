# A proxy server for RAG prompts

This is a alternative to the `rag-api-server` project. It is a web server that

* Takes an OpenAI API compatible request
* Parse the last user message
* Compute an embedding vector for the message using a local `llama-api-server` on the Gaia node (eg., `http://localhost:8080`)
* Search for context from the local Qdrant instance (eg., `http://localhost:6333`)
* Modify the request system prompt with the RAG search result
* Send the request to the local `llama-api-server` on the Gaia node (eg., `http://localhost:8080`)
* Respond with the `llama-api-server` response

