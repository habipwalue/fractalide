# Calculator Example for Fractalide

This example demonstrates a simple calculator built using the Fractalide architecture. It showcases basic node implementation, edge definition, and subgraph composition.

## Architecture Overview

The calculator example follows Fractalide's dataflow programming model:

1. **Edges**: Define the data types that flow between nodes
   - `number`: Represents a numerical value
   - `operation_result`: Represents the result of an operation with possible error

2. **Nodes**: Implement the calculator operations
   - `add`: Adds two numbers
   - `subtract`: Subtracts one number from another
   - `multiply`: Multiplies two numbers
   - `divide`: Divides one number by another
   - `display`: Displays the result

3. **Subgraph**: Connects the operation nodes to form a calculator

## How It Works

1. The user inputs two numbers and an operation
2. The numbers flow to the appropriate operation node
3. The operation node performs the calculation
4. The result flows to the display node
5. The display node shows the result

## Running the Example

To build and run the calculator:

```bash
cd calculator_example
cargo run
```

The application will prompt you to enter two numbers and an operation, then display the result.

## Implementation Details

This example demonstrates several key Fractalide concepts:

- **Simple Agents**: Each operation is implemented as a simple agent
- **Basic Edge Types**: Using simple data types for communication
- **Error Handling**: Proper handling of division by zero and other errors
- **Subgraph Composition**: Connecting nodes to form a calculator

## Extending the Example

You can extend this example by:

1. Adding more operations (power, square root, etc.)
2. Implementing a memory function
3. Creating a more complex expression parser
4. Building a graphical user interface

## Learning from this Example

This example demonstrates fundamental Fractalide concepts:

- **Basic Node Implementation**: How to create simple computational units
- **Edge Definition**: How to define data types for message passing
- **Subgraph Composition**: How to connect nodes to form an application
- **Error Handling**: How to handle and propagate errors in a dataflow system