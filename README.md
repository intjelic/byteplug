# Byteplug - Minimalistic Multimedia Library

**Byteplug** is a multimedia library for the Rust language which aims to provide a decent tools to write applications (or games) for desktop (or mobile).

While it appears to feature tools in many areas (animation, ui, etc.), it actually provides the bare minimum and each are carefully crafted to be extended and integrate well with external libraries.
For instance, if you had a bad experience with the thousands of poorly designed GUI toolkits out there, you might be plaisantly surprised by the **ui** momdule.

It was heavily inspired by similar framework (like SFML) and other extensions developed by the community (Thor) coupled with my decade of experience in the field and my relentless perfectionism.

## Features

The library is under development. For now, I can only list the modules.

- geometry
- animation
- image
- audio
- video
- draw
- controller
- application
- ui

Until now, it's only tested on the **Linux** platform but as I go in the implementation and my peronal use of this library, I will extend the testing to the other platform.

## Usage

To use `byteplug`, first add the dependency to your Cargo.toml file.

```
[dependencies]
byteplug = "0.1.0"
```

Next, you can start using it. You could start with this minimal example.

```
use byteplug::application::Application;

fn main() {
    let app = Application::new();
    app.run();
}
```

Or you could play around with the examples.

## Examples

The source code repository of the project also comes with a bunch of examples which you can compile to see if it runs fine on your computer.

Start with cloning the repository.

```
git clone https://github.com/intjelic/byteplug
```

Then run the an example with a standard `cargo run --example myexample` command from the directory. The examples are in the `examples/` and you just have to substitute `myexample` with the name of the example (without the `.rs` extension of course).

## Community

To be written.

## Contributing

To be written.
