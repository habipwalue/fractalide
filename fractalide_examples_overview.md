# Fractalide Architecture Examples

This document provides an overview of the example applications created to demonstrate the Fractalide architecture. Each example showcases different aspects of Fractalide's dataflow programming model.

## Understanding Fractalide Architecture

Fractalide is a dataflow programming platform built on three core concepts:

1. **Nodes**: Computational units that process data
   - **Agents**: Nodes implemented in Rust that perform specific tasks
   - **Subgraphs**: Compositions of other nodes connected with edges

2. **Edges**: Typed connections between nodes
   - Defined using Cap'n Proto schemas
   - Ensure type safety in message passing

3. **Fractals**: Third-party libraries that can be composed into applications

## Example Applications

### 1. Calculator Example

A simple calculator that demonstrates basic Fractalide concepts:

- **Nodes**: Addition, subtraction, multiplication, division operations
- **Edges**: Number inputs and outputs
- **Subgraph**: Connects operation nodes to form a calculator

**Key Concepts**: Basic node implementation, simple edges, subgraph composition

### 2. Image Processing Pipeline

A more complex example that processes images through a series of transformations:

- **Nodes**: Image loader, grayscale converter, blur filter, image saver
- **Edges**: Raw and processed image data types
- **Pipeline**: Sequential processing of images through transformation nodes

**Key Concepts**: Complex data types, sequential processing, file I/O

### 3. Web Server Example

Demonstrates network communication and request handling:

- **Nodes**: HTTP server, router, static file handler
- **Edges**: HTTP request and response types
- **Routing**: Directing requests to appropriate handlers based on URL patterns

**Key Concepts**: Concurrent processing, message routing, stateful agents

### 4. Chat Application

Shows real-time communication and state management:

- **Nodes**: Message store, user registry, room manager
- **Edges**: Chat messages and user events
- **State**: Tracking users, rooms, and messages

**Key Concepts**: State management, event-driven architecture, composite nodes

## Implementation Patterns

These examples demonstrate several important implementation patterns in Fractalide:

1. **Agent Pattern**: Implementing nodes as Rust agents with input/output ports
   ```rust
   agent! {
     input(a: number, b: number),
     output(sum: number),
     fn run(&mut self) -> Result<Signal> {
       let a = self.input.a.recv()?;
       let b = self.input.b.recv()?;
       let mut sum = self.output.sum.allocate();
       sum.set_value(a.get_value()? + b.get_value()?);
       self.output.sum.send(sum)?;
       Ok(End)
     }
   }
   ```

2. **Subgraph Pattern**: Composing nodes using Flowscript
   ```
   a(${math.add}) output => input b(${math.multiply})
   b output => input c(${io.print})
   ```

3. **Edge Definition Pattern**: Using Cap'n Proto schemas
   ```capnp
   @0xabcdef1234567890;
   struct Number {
     value @0 :Float64;
   }
   ```

4. **Error Handling Pattern**: Propagating errors through the system
   ```rust
   if let Err(e) = result {
     let mut error_msg = self.output.error.allocate();
     error_msg.set_string(&format!("Error: {}", e));
     self.output.error.send(error_msg)?;
   }
   ```

## Best Practices

Based on these examples, here are some best practices for Fractalide development:

1. **Design edges first**: Define your data types before implementing nodes
2. **Keep nodes focused**: Each node should do one thing well
3. **Use subgraphs for composition**: Build complex behavior by composing simple nodes
4. **Handle errors gracefully**: Provide meaningful error messages and recovery paths
5. **Document your components**: Include clear documentation for nodes, edges, and subgraphs
6. **Test components individually**: Ensure each node works correctly in isolation
7. **Use meaningful port names**: Name ports based on their purpose, not their data type

## Conclusion

Fractalide's architecture provides a powerful way to build applications using dataflow programming. By separating computation (nodes) from communication (edges), it enables modular, reusable, and maintainable code. The examples provided demonstrate how to apply these concepts to build a variety of applications, from simple calculators to complex chat systems.