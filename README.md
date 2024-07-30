# sloop

Sloop is an experimental build system for Rust programming language. 

_This is cargo. It can ship a lot of crates:_
![](macroquad.rs/cargo.jpg)

_This is sloop. It can barely ship very few crates:_
![](macroquad.rs/sloop.jpg)


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

`sloop` is a rust library, allowing to write build scripts in rust.

# Motivation

In cargo, `build.rs` is very powerful and very easy to make, while project specific cargo subcommands are super uncommon practice. Sloop is the opposite - it is super easy to make a project-specific build pipeline, but virtually impossible for a dependency to invoke a custom build script.

Note that for now `sloop` is mostly an educational project with only one goal: show how `rustc` compile rust crates.
