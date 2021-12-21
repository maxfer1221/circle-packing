# circle-packing 

### About
`circle_count` number of circles are placed into a canvas every frame, whose radii expand at `circle_rate` units per frame. Once a circle collides with another circle or it hits the edge of the screen, the circle will stop expanding.

Circle colors are dictated by pixel colors in the original image (i.e., a circle positioned at (x,y) will have the same color as the pixel at (x,y) in the original image)

Feature extraction prioritizes sampling of striking pixels for the circle packing, leading to more detailed images.

A hash grid is used to speed up distance calculations. Only nearby circles are tested against to dictate whether a collision has been found.

## Usage
1. `git clone https://github.com/maxfer1221/circle-packing`
2. `cd circle-packing`
3. `cargo run path/to/image circle_count circle_rate feature_threshold thread_count step_size`, e.g.: `cargo run test\ images/example1.jpg 100 1 10 10 1`

`circle_count`: Number of circles to add per frame. Larger numbers will lead to smaller circles and more detailed images.

`circle_rate`: Expansion rate for circle radii. Larger numbers will lead to larger circles before collisions.

`feature_threshold`: (feature extraction specific) Dictates feature sensitivity. Higher values mean fewer, more striking features will be found.

`thread_count`: Dictates how many threads the program can spawn. Minimum of 1.

`step_size`: (feature extraction specific) Dictates how many pixels the program samples. A step size of 1 would sample every pixel, a step size of 2 would sample 1/4th of the pixels (half the width, half the height), etc.

### Circle Packing [Examples](https://github.com/maxfer1221/circle-packing/tree/main/out)
<p float="left">
 <img display="inline" src="https://github.com/maxfer1221/circle-packing/blob/main/test%20images/example1.jpg?raw=true" alt="example_1" height="320">
 <img src="https://github.com/maxfer1221/circle-packing/blob/main/out/example1.png?raw=true" alt="example_1_out" height="320">
</p>
<p float="left">
 <img display="inline" src="https://github.com/maxfer1221/circle-packing/blob/main/test%20images/example2.jpeg?raw=true" alt="example_2" width="400">
 <img src="https://github.com/maxfer1221/circle-packing/blob/main/out/example2.png?raw=true" alt="example_2_out" width="400">
</p>
<p float="left">
 <img display="inline" src="https://github.com/maxfer1221/circle-packing/blob/main/test%20images/example3.jpg?raw=true" alt="example_3" width="400">
 <img src="https://github.com/maxfer1221/circle-packing/blob/main/out/example3.png?raw=true" alt="example_3_out" width="400">
</p>
Created through: `cargo run test\ images/example_._ 200 1 10 10 1`

### Dependencies
 - sdl2-gfx (`sudo apt install libsdl2-gfx-dev`)

### Libraries used
 - [sdl2](https://crates.io/crates/sdl2): Circle packing display
 - [image](https://crates.io/crates/image): Basic iage manipulation
 - [crossbeam](https://crates.io/crates/crossbeam): Thread synchronization and scoping

### TBD
 - ~~Hash Grid usage to expedite circle collision detection (current solution compares every circle)~~
