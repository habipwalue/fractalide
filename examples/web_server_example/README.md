# Web Server Example for Fractalide

This example demonstrates a simple web server built using the Fractalide architecture. It showcases how to implement network communication, request routing, and concurrent processing.

## Architecture Overview

The web server example follows Fractalide's dataflow programming model:

1. **Edges**: Define the data types that flow between nodes
   - `http_request`: Represents an HTTP request with method, path, headers, and body
   - `http_response`: Represents an HTTP response with status, headers, and body
   - `route_info`: Represents routing information for request handling

2. **Nodes**: Implement the web server components
   - `http_server`: Listens for incoming HTTP connections
   - `router`: Routes requests to appropriate handlers based on path
   - `static_file_handler`: Serves static files from the filesystem
   - `api_handler`: Handles API requests and returns JSON responses
   - `logger`: Logs requests and responses

3. **Routing**: Connects the server components to form a web server

## How It Works

1. The HTTP server listens for incoming connections
2. When a request arrives, it's parsed and sent to the router
3. The router determines which handler should process the request
4. The handler processes the request and generates a response
5. The response is sent back to the client

## Running the Example

To build and run the web server:

```bash
cd web_server_example
cargo run
```

The server will start listening on port 8080. You can access it at http://localhost:8080.

## Implementation Details

This example demonstrates several key Fractalide concepts:

- **Concurrent Processing**: Handling multiple requests simultaneously
- **Message Routing**: Directing requests to appropriate handlers
- **Stateful Agents**: Maintaining connection state
- **Error Handling**: Proper handling of network errors and invalid requests

## Extending the Example

You can extend this example by:

1. Adding more route handlers
2. Implementing authentication and authorization
3. Adding database connectivity
4. Supporting WebSockets or Server-Sent Events

## Learning from this Example

This example demonstrates advanced Fractalide concepts:

- **Network Communication**: How to handle network I/O in a dataflow system
- **Message Routing**: How to direct messages based on content
- **Concurrent Processing**: How to handle multiple requests simultaneously
- **Stateful Agents**: How to maintain state in a dataflow system