# QT-Kirigami-in-rust
uses cxx to bridge only the most basic types from c++ to rust

the qt/ crate holds the c++ and rust definitions for all types used.

During build time, the c++ files are compiled and linked against system QT libraries (see qt/build.rs), which are then used as interop via [cxx](https://crates.io/crates/cxx).

To use the qt crate, add it as a dependency in Cargo.toml and create a new qt::QtApp Instance by giving it the path to a qml file. Run the app by calling the `run` member function.

Check qt/build.rs for required system dependencies, following the [kde getting started page](https://develop.kde.org/docs/kirigami/introduction-getting_started/) should provide everything you need.