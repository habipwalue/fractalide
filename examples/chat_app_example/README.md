# Chat Application Example for Fractalide

This example demonstrates how to build a real-time chat application using the Fractalide architecture. It implements a simple chat system with rooms, users, and messages.

## Architecture Overview

The chat application example follows Fractalide's dataflow programming model:

1. **Edges**: Define the data types that flow between nodes
   - `chat_message`: Represents a chat message with user ID, username, content, timestamp, and room ID
   - `user_event`: Represents user events like joining or leaving a room

2. **Nodes**: Implement the chat application logic
   - `message_store`: Stores and retrieves chat messages for each room
   - `user_registry`: Tracks users in each room and provides user information
   - `room_manager`: Subgraph that connects message store and user registry

3. **Application**: Simulates a chat session with users joining, sending messages, and leaving

## How It Works

1. Users join a chat room by sending a `user_event` with type `join`
2. The `user_registry` tracks which users are in which rooms
3. Users send messages by creating `chat_message` objects
4. The `message_store` stores messages for each room
5. Users can leave a room by sending a `user_event` with type `leave`
6. Applications can request the list of messages or users in a room

## Running the Example

To build and run the chat application:

```bash
cd chat_app_example
nix-build
./result/bin/chat_app
```

The application will simulate a chat session with users joining, sending messages, and leaving.

## Extending the Example

You can extend this example by:

1. Adding a web interface using the web server example
2. Implementing private messaging between users
3. Adding support for multiple chat rooms
4. Implementing message persistence with a database
5. Adding real-time notifications for typing indicators

## Learning from this Example

This example demonstrates advanced Fractalide concepts:

- **State Management**: Maintaining application state across multiple messages
- **Composite Nodes**: Using subgraphs to create higher-level components
- **Event-Driven Architecture**: Responding to different types of events
- **Data Modeling**: Using Cap'n Proto schemas to define message types
- **Simulation**: Testing the application with simulated user interactions