# sloop

Sloop is an experimental build system for Rust programming language. 

_This is cargo. It can ship a lot of crates:_
![](macroquad.rs/cargo.jpg)

_This is sloop. It can barely ship very few crates:_
![](macroquad.rs/sloop.jpg)

# Motivation

`sloop` is an experiment on making a build system tailored for a handful of hand-picked, audited and well curated dependencies.

# How it looks like 

```bash
.
├── build.rs
├── deps
│   ├── libc
│   └── miniquad
├── src
│   └── triangle.rs
```


```rust
fn main() -> Result<(), ()> {
    let libc = sloop::DependencyBuilder::new("deps/libc")
        .build()?;
    let miniquad = sloop::DependencyBuilder::new("deps/miniquad")
        .with_dependency(&libc)
        .build()?;

    sloop::Builder::new()
        .binary()
        .name("TriangleOnTheSloop")
        .entrypoint("src/triangle.rs")
        .with_dependency(&libc)
        .with_dependency(&miniquad)
        .build()
}
```

```bash
> ls
build.rs  deps/  src/ 

> sloop
Building libc
Building miniquad
Building TriangleOnTheSloop src/triangle.rs
Done!

> ls
build.rs  deps/  src/  TriangleOnTheSloop*
```
Example from *miniquad_triangle*, [sloop-example-projects](https://github.com/not-fl3/sloop-example-projects/)

`sloop` is a rust library, allowing to write build scripts in rust.

