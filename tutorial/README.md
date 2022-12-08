# Fixes to Rusty Engine set-up

I had to do a few things to create the `tutorial` project for rusty_engine and get it to compile:

## 1

Add "tutorial" to the workspace config in the top-level Cargo.toml:

```
[workspace]
members = [ "example/*", "exercise/*", "tutorial" ]
resolver = "2"
```

## 2

Add a missing library (see [failed to run custom build command for `libudev-sys v0.1.4`](https://users.rust-lang.org/t/failed-to-run-custom-build-command-for-libudev-sys-v0-1-4/53926/2)):

```
sudo apt install libudev-dev
```

## 3

Add `resolver = "2"` to the workspace config in the top-level Cargo.toml to resolve the build error

```
error: Metal API enabled on non-Apple OS. If your project is not using resolver="2" in Cargo.toml, it should.
```

(see [resolver = "2" is not the default in workspaces #9996](https://github.com/rust-lang/cargo/issues/9996#issuecomment-949193342)):

```
[workspace]
members = [ "example/*", "exercise/*", "tutorial" ]
resolver = "2"
```
