# A Rust CLI tool: image reviewer
This is a rust cli tool that allows users to view images in a directory. The user can navigate between images and zoom in/out on them. 

## Project Setup
1. clone the repo:
```
git clone https://github.com/JuliaJHL/week7-rust-mini-proj.git
```
2. cd into the project:
```
cd week7-rust-mini-proj
```
3. compile the project:
```
cargo build
```
4. run the project:
```
cargo run
```

## examples
When you run the project, it will prompt you the usage:
```
Usage: cargo run <directory_path>
```
When you enter `cargo run ./examples`, it will show you the first image in the file by default.

<img width="500" alt="start" src="https://github.com/JuliaJHL/week7-rust-mini-proj/blob/main/readmepics/start.png">

It then prompts you with instructions:
```
Press 'q' to quit, 'n' for the next image, 'p' for the previous image, '+' to zoom in, and '-' to zoom out.
```
If you enter `n`, it will display the next image (`p` for previous):

<img width="500" alt="next" src="https://github.com/JuliaJHL/week7-rust-mini-proj/blob/main/readmepics/next.png">


If you enter `+`, it will zoom in the current image (`-` for zoom out):

<img width="500" alt="plus" src="https://github.com/JuliaJHL/week7-rust-mini-proj/blob/main/readmepics/plus.png">

If you enter `q`, you will quit.

<img width="500" alt="quit" src="https://github.com/JuliaJHL/week7-rust-mini-proj/blob/main/readmepics/quit.png">

## Notes:
* The effect of the enlarged image in the example is not obvious, because I set the size of the picture in the README.md to be the same.
* It may take a slightly long time for the tool to display the image. 

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
