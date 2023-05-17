# Raytracer README

This is a raytracer written in Rust that generates images using various command-line arguments.

## Usage

To run the raytracer, follow the steps below:

1. Install Rust: Make sure you have Rust installed on your system. You can download it from the official website: [https://www.rust-lang.org](https://www.rust-lang.org)

2. Clone the repository: Clone this repository to your local machine using Git.

3. Navigate to the project directory: Use the `cd` command to go into the cloned repository's directory.

4. Build the project: Run the following command to build the raytracer.

5. Run the raytracer: Execute the raytracer with the desired command-line arguments. The available options are as follows:

- `-w [width]`: Specifies the width of the output image in pixels.
- `-h [height]`: Specifies the height of the output image in pixels.
- `-r [recursion_depth]`: Sets the maximum recursion depth for ray reflections and refractions.
- `-k [ambient_occlusion_samples]`: Specifies the number of ambient occlusion samples per pixel.
- `-i [input_file]`: Specifies the input file containing the scene description.

6. Output: Once the raytracer finishes execution, the resulting image will be saved in the project directory as `output.png`.

## Examples

Here are a few examples to help you get started:

- Generate a 800x600 image with a recursion depth of 4, 100 ambient occlusion samples, and using the `scene.txt` file as input:
cargo run --release -w 800 -h 600 -r 4 -k 100 -i scene.txt