# Byteplug - Minimalistic Multimedia Library

**Byteplug** is a multimedia library for the Rust language which aims to provide a decent set of tools to write applications (or games) for desktop (or mobile).

While it appears to cover a lot of areas (animation, graphical user interface, etc.), it actually provides the bare minimum and each item is carefully crafted to be extended and integrate well with other libraries. For example, you may be pleasantly surprised by the **widget** module which only provides the common and expected input logic, while defining absolutely no default appearance.

It was heavily inspired by the SFML framework and other extensions developed by its community (such
as the Thor library) coupled with my decade of experience in the field and my unhealthy obsession for perfection.

## Features

The library is under development. For now, the only thing that is likely to remain the same is the
list of module in their 'least dependant' order.

- geometry
- animation

- image
- audio
- video

- draw

- controller
- widget
- application
- game

Until now, it's only tested on the **Linux** platform but as I go in the implementation and my
personal use of this framework for personal projects, I will extend the testing to the other
platform and their official support will gradually come.

## Usage

To use `byteplug`, first add the dependency to your Cargo.toml file.

```
[dependencies]
byteplug = "0.0.1"
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
