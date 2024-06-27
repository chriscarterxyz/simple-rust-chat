use std::io::{Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::collections::{HashMap, VecDeque};

use crate::message::{Message};

const LOCAL: &str = "127.0.0.1:9876";

fn read_message(stream: &mut TcpStream) -> Message {
    let mut reader = BufReader::new(stream);
    let received: Vec<u8> = reader.fill_buf().expect("fill_buf").to_vec();
    reader.consume(received.len());
    let result = String::from_utf8(received).unwrap();
    serde_json::from_str(&result).unwrap()
}

fn write_message(stream: &mut TcpStream, message: &Message) {
    let mut data = serde_json::to_string(message).unwrap();
    data.push_str("\n");
    let _ = stream.write(data.as_bytes());
}

pub fn main() {
    let mut message_queue = HashMap::<String, VecDeque<Message>>::new();

    let listener = TcpListener::bind(&LOCAL).unwrap();
    for incoming in listener.incoming() {
        let mut stream: TcpStream = incoming.expect("could not get stream");

        let message: Message = read_message(&mut stream);

        match message {
            Message::Join(username) => {
                message_queue.insert(username.clone(), VecDeque::<Message>::new());
                for (user, messages) in &mut message_queue {
                    if *user != username {
                        messages.push_back(Message::Join(username.clone()));
                    }
                }
            },
            Message::Chat(data) => {
                for (user, messages) in &mut message_queue {
                    if *user != data.from {
                        messages.push_back(Message::Chat(data.clone()));
                    }
                }
            },
            Message::Leave(username) => {
                message_queue.remove(&username);
                for (user, messages) in &mut message_queue {
                    if *user != username {
                        messages.push_back(Message::Leave(username.clone()));
                    }
                }
            },
            Message::Poll(username) => {
                let user_queue = message_queue.get_mut(&username);
                match user_queue {
                    Some(queue) => {
                        loop {
                            let front: Option<Message> = queue.pop_front();
                            match front {
                                Some(message) => write_message(&mut stream, &message),
                                None => break

                            }
                        }
                    },
                    None => write_message(&mut stream, &Message::None),
                }
            }
            Message::None => {}
        }
    }  
}
