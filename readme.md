PASSWORD GENERATOR (RUST + YEW)

A simple password generator built using Rust and the Yew web framework.
This project runs in the browser using WebAssembly (WASM) and allows users to generate secure passwords through a web interface.

FEATURES

Built entirely in Rust

Uses Yew for the frontend

Compiled to WebAssembly

Runs locally in a web browser

Simple and fast password generation

REQUIREMENTS

Before running this project, make sure you have:

Rust installed via rustup

A modern web browser (Chrome, Firefox, etc.)

INSTALLATION AND SETUP

INSTALL RUST USING RUSTUP

If Rust is not installed, download and install it from:

https://rustup.rs

Verify installation by running:

rustc --version

cargo --version

ADD THE WEBASSEMBLY TARGET

Run the following command in your terminal:

rustup target add wasm32-unknown-unknown

INSTALL TRUNK

Trunk is required to build and serve Yew applications.

Run:
cargo install trunk

RUN THE PROJECT

Navigate to the project root directory and run:

trunk serve

OPEN IN YOUR BROWSER

Once Trunk is running, open your browser and go to:

http://localhost:8080

You may use any web browser, such as Chrome.

TECH STACK

Rust

Yew

WebAssembly (WASM)

Trunk

PROJECT PURPOSE

This project was created to:

Practice Rust frontend development

Learn Yew and WebAssembly

Demonstrate Rust skills beyond backend and CLI applications
