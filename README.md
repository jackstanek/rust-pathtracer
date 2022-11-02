# Rust pathtracer

This repository contains a very simple, single-threaded implementation of a
Monte Carlo pathtracer in Rust. Its functionality is very limited (the only
supported geometry at the moment is a sphere, and the only supported material
is Lambertian) but it demonstrates the principles behind Monte Carlo methods
in physically-based rendering.

Once I have some free time again, I'd like to go back and rewrite this from
scratch using a more extensible architecture, using  principles from [PBR
3e](https://www.pbr-book.org/3ed-2018/contents).


## Building

This can be built and run with `cargo`:

```
$ cargo run
```

This should show an output of three spheres in a horizontal line.
