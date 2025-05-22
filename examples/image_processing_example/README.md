# Image Processing Example for Fractalide

This example demonstrates an image processing pipeline built using the Fractalide architecture. It showcases how to implement a sequential processing pipeline with complex data types.

## Architecture Overview

The image processing example follows Fractalide's dataflow programming model:

1. **Edges**: Define the data types that flow between nodes
   - `image_data`: Represents raw image data with dimensions and pixel values
   - `file_path`: Represents a file path for loading and saving images
   - `processing_result`: Represents the result of an image processing operation

2. **Nodes**: Implement the image processing operations
   - `image_loader`: Loads an image from a file
   - `grayscale_converter`: Converts an image to grayscale
   - `blur_filter`: Applies a blur filter to an image
   - `edge_detector`: Detects edges in an image
   - `image_saver`: Saves an image to a file

3. **Pipeline**: Connects the processing nodes to form an image processing pipeline

## How It Works

1. The user provides an input image file path
2. The image loader loads the image
3. The image flows through a series of processing nodes
4. Each node applies a transformation to the image
5. The final processed image is saved to an output file

## Running the Example

To build and run the image processor:

```bash
cd image_processing_example
cargo run
```

The application will prompt you to enter an input image path and an output image path, then process the image through the pipeline.

## Implementation Details

This example demonstrates several key Fractalide concepts:

- **Complex Data Types**: Using structured data for image representation
- **Sequential Processing**: Passing data through a series of transformations
- **File I/O**: Loading and saving data from the filesystem
- **Error Handling**: Proper handling of file not found and other errors

## Extending the Example

You can extend this example by:

1. Adding more image processing operations (resize, rotate, etc.)
2. Implementing parallel processing for different image regions
3. Creating a more complex processing graph with branches
4. Building a graphical user interface for interactive processing

## Learning from this Example

This example demonstrates advanced Fractalide concepts:

- **Complex Data Handling**: How to work with structured data types
- **Sequential Processing**: How to build processing pipelines
- **File I/O**: How to interact with the filesystem
- **Error Propagation**: How to handle and propagate errors in a pipeline