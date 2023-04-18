# Android Rust FFI example

## Introduction

This serves as a small app that runs a rust runtime in an android service. This runtime pings an http endpoint every 10 seconds. 


## Build: 

```
cargo build --target i686-linux-android --release
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
```

## What it does

The idea would be just a PoC of running a Rust Tokio runtime in a background(not the UI) thread, inside of a service in android. This runtime would also be able to query
the JVM for certain methods existing on certain classes to invoke over time. This would allow the rust runtime to call data/functionality out of the android runtime.

## Take note: 

This was build on 64-bit windows OS, so NDK specific implementations might be different. It would be best for you to generate your own ndk linkers, add those to the config.toml in .cargo and then generate new target compilations. 
