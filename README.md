# async_chanx

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_chanx.svg?branch=master)](https://travis-ci.org/najamelan/async_chanx)
[![Docs](https://docs.rs/async_chanx/badge.svg)](https://docs.rs/async_chanx)
[![crates.io](https://img.shields.io/crates/v/async_chanx.svg)](https://crates.io/crates/async_chanx)


> Impl Sink for common async channel implementations.

This library implements the missing Sink implementation for common async channels like tokio-sync. It introduces a common error type for channel senders and wraps some implementations that already provide Sink, to make it ergonomic to abstract out over them.

This serves 2 purposes:
- for libraries: let your users decide which channel implementation to use in your library. The main benefit of this is where this will change the behavior of your lib: bounded vs unbounded, backpressure vs overwriting older messages etc.
- conveniently compare the performance of channel implementations for your workload by making them plug and play.

DEVELOPMENT STATUS: alpha! I decided to push this on crates.io to facilitate progressing on and publishing alpha versions of other crates that use this. It completely lacks polish and is not recommended for production until it reaches 0.1. Testing, examples and documentation are largely missing and algorithms still need to be reviewed.

Currently I'm waiting to see if async-channel will implement `Sink`, so I don't have to do it here and then I will work on polishing this some more.

## Table of Contents

- [Install](#install)
   - [Upgrade](#upgrade)
   - [Dependencies](#dependencies)
   - [Security](#security)
- [Usage](#usage)
   - [Basic Example](#basic-example)
   - [API](#api)
- [Contributing](#contributing)
   - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add async_chanx`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

   async_chanx: ^0.1
```

With Cargo.toml
```toml
[dependencies]

    async_chanx = "0.1"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/async_chanx/blob/master/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies. Cargo will automatically handle it's dependencies for you.

There are no optional features.


### Security




## Usage



### Basic example

```rust

```

## API

API documentation can be found on [docs.rs](https://docs.rs/async_chanx).


## Contributing

Please check out the [contribution guidelines](https://github.com/najamelan/async_chanx/blob/master/CONTRIBUTING.md).


### Testing


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](https://github.com/stumpsyn/policies/blob/master/citizen_code_of_conduct.md#4-unacceptable-behavior) are not welcome here and might get you banned. If anyone, including maintainers and moderators of the project, fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

