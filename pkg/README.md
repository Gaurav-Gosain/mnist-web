# MNIST Inference on Web Using Rust and Wasm (simd)

## Running

1. Build

   ```shell
   ./build-for-web.sh
   ```

2. Run the server

   ```shell
   ./run-server.sh
   ```

3. Open the [`http://localhost:8000/`](http://localhost:8000/) in the browser.

## Model

Layers:

1. Input Image (28,28, 1ch)
2. `Conv2d`(3x3, 8ch), `BatchNorm2d`, `Gelu`
3. `Conv2d`(3x3, 16ch), `BatchNorm2d`, `Gelu`
4. `Conv2d`(3x3, 24ch), `BatchNorm2d`, `Gelu`
5. `Linear`(11616, 32), `Gelu`
6. `Linear`(32, 10)
7. Softmax Output

The total number of parameters is 376,952.

The model is trained with 4 epochs and the final test accuracy is 98.67%.

The training and hyper parameter information in can be found in
[`burn` MNIST example](https://github.com/tracel-ai/burn/tree/main/examples/mnist).

## Resources

1. [Rust 🦀 and WebAssembly](https://rustwasm.github.io/docs/book/)
2. [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
