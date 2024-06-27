use ctrlc;
use std::process;
use std::{thread, time};
use std::net::{TcpStream};
use std::io::{BufRead, BufReader, Write, stdin, stdout};

use crate::message::{Message, MessageData};

const LOCAL: &str = "127.0.0.1:9876";


fn deserialize(message_str: String) -> Message {
    let msg = serde_json::from_str(&message_str);
    match msg {
        Ok(msg) => msg,
        Err(e) => {
            println!("{}", e);
            Message::None
        }
    }
}

fn serialize(message: Message) -> String {
    serde_json::to_string(&message).unwrap()
}

fn poll(username: &String) {
    let poll: Message = Message::Poll(username.clone());
    let data = serialize(poll);

    let mut stream: TcpStream = TcpStream::connect(&LOCAL).expect("cannot connect");
    let _ = stream.write_all(data.as_bytes());
    let _ = stream.flush();

    let mut reader = BufReader::new(stream);
    let _ = reader.fill_buf().expect("fill_buf").to_vec();

    for line in reader.lines() {
        let message: Message = deserialize(line.expect("line"));

        match message {
            Message::Join(username) => println!("{username} joined"),
            Message::Chat(data) => println!("{}: {}", data.from, data.content),
            Message::Leave(username) => println!("{username} left"),
            Message::Poll(_) => {}, // should never happen
            Message::None => {}
        }
    }
}

fn send(message: Message) {
    let data = serialize(message);
    let mut stream: TcpStream = TcpStream::connect(&LOCAL).expect("cannot connect");
    let _ = stream.write(data.as_bytes());  
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut buffer = String::new();

    let _ = stdout().flush(); // what's this for
    stdin().read_line(&mut buffer).expect("invalid input");

    if let Some('\n') = buffer.chars().next_back() {
        buffer.pop();
    }

    return buffer;
}


pub fn main() {

    let username = input("username: ");
    send(Message::Join(username.clone()));

    let username2 = username.clone();
    let username3 = username.clone();

    let _ = ctrlc::set_handler(move || { 
        send(Message::Leave(username3.clone()));
        process::exit(0x0100);
    });

    thread::spawn(move || {
        loop {
            poll(&username2);
            thread::sleep(time::Duration::from_millis(3000));
        }
    });

    loop {

        let buffer = input("");

        let chat = Message::Chat(MessageData {
            from: username.clone(),
            content: buffer,
        });

        send(chat);

    }
}
