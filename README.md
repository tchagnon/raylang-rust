# raylang-rust

Simple raytracer for exploring programming languages.  Rust edition.

Based on [David Breen][david]'s [CS636 Class][cs636] assignments and my own
previous Haskell implementation [cs636-raytracer](/tchagnon/cs636-raytracer).

[david]: https://www.cs.drexel.edu/~david/
[cs636]: https://www.cs.drexel.edu/~david/Classes/CS431/index_Spring09.html

## Why?

For fun.  To learn new languages, and how usable they are for a heavily
computational program.  Rust in this case.

# Running Examples

To run the examples, you need some `.smf` model files in the `models/`
directory.  The example model files are originally from
https://www.cs.drexel.edu/~david/Classes/CS586/Models/

Use the following command to download them all:

```
wget -r -np -nd -A smf https://www.cs.drexel.edu/~david/Classes/CS586/Models/
```

## Building and Running

```
cargo build --release
cargo run scenes/scene0.toml --release
```

Running without `--release` will take significantly longer due to the lack of
optimization.

Output image will be written to the file specified in the `scene0.toml` file.

```
Wrote file "scene0.png"
```

![rendered image](doc/scene0.png)

## Versions and Status

Tagged versions correspond to assignments. For example, 0.1.0 implements the
requirements for assignment 1.  `master` may contain a partially completed next
version at any time.

The current version is: `0.1.0`
