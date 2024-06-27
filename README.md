# Simple Chat

A simple chat server implemented in Rust, mostly just for practice with Rust concepts and TCP sockets. It currently uses short polling and has a very basic command line interface for sending messages. It also only runs on `localhost` currently and has no authentication/user system, and no way to restore messages for a user if the server were to crash. 

Maybe, if I stay interested, I'll keep improving it and adding features. Or maybe I won't. 

You can start the server with `cargo run server`, and start a client with `cargo run client`. 
