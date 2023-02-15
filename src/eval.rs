use iced::{Subscription, subscription};
use lazy_static::__Deref;
use std::{process::{Stdio, Child}, io::{BufReader, Write, BufRead}, sync::mpsc};

use crate::Message;

pub const STOP_COMMAND: &str = "STOP";

pub enum EngineState {
    Start(Engine),
    Thinking(Child, String, mpsc::Receiver<String>),
    TurnedOff,
}

#[derive(Debug, Clone)]
pub struct Engine {
    pub engine_path: String,
    pub search_up_to: String, 
    pub position: String,
}

impl Engine {

    pub fn new(path: Option<String>, limit: String, position: String) -> Self {
        Self {
            engine_path: path.unwrap_or_default(),
            search_up_to: limit,
            position: position,
        }
    }

    pub fn run(&self) -> Subscription<Message> {
        struct Engine;

        subscription::unfold(
            std::any::TypeId::of::<Engine>(),
            EngineState::Start(self.clone()),
            |state| async move {
                match state {
                    EngineState::Start(engine_data) => {
                        let (sender, receiver) = mpsc::channel();

                        let mut child = std::process::Command::new(engine_data.engine_path)
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("Error calling engine");                            
                        let pos = String::from("position fen ") + &engine_data.position + &String::from("\n");
                        let limit = String::from("go ") + &engine_data.search_up_to + &"\n";

                        child.stdin.as_mut().unwrap().write_all(b"uci\n").expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(b"isready\n").expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(b"ucinewgame\n").expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(b"setoption name UCI_AnalyseMode value true\n").expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(pos.as_bytes()).expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(limit.as_bytes()).expect("Error communicating with engine");

                        (Some(Message::EngineReady(sender)), EngineState::Thinking(child, engine_data.search_up_to, receiver))

                    } EngineState::Thinking(mut child, search_up_to, receiver) => {
                        let msg = receiver.try_recv();
                        if let Ok(msg) = msg {
                            if &msg == STOP_COMMAND {
                                child.stdin.as_mut().unwrap().write_all(b"stop\n").expect("Error communicating with engine");
                                child.stdin.as_mut().unwrap().write_all(b"quit\n").expect("Error communicating with engine");
                                if let Err(e) = child.kill() {
                                    eprintln!("Error killing the engine process: {} ", e);
                                }                                
                                return (Some(Message::EngineStopped), EngineState::TurnedOff);
                            } else {
                                let pos = String::from("position fen ") + &msg + &String::from("\n");
                                let limit = String::from("go ") + &search_up_to + &"\n";

                                child.stdin.as_mut().unwrap().write_all(b"stop\n").expect("Error communicating with engine");
                                child.stdin.as_mut().unwrap().write_all(b"setoption name UCI_AnalyseMode value true\n").expect("Error communicating with engine");
                                child.stdin.as_mut().unwrap().write_all(b"ucinewgame\n").expect("Error communicating with engine");
                                child.stdin.as_mut().unwrap().write_all(pos.as_bytes()).expect("Error communicating with engine");
                                child.stdin.as_mut().unwrap().write_all(limit.as_bytes()).expect("Error communicating with engine");
                            }
                        }
                        let stdout = child.stdout.as_mut().unwrap();
                        let mut buf_str = String::new();
                        let mut reader = BufReader::new(stdout);
                        let mut eval = None;
                        let mut best_move = None;
                        let Ok(_bytes)  = ({
                            reader.read_line(&mut buf_str)
                        }) else {
                            panic!("error reading engine output")
                        };
                        let vector: Vec<&str> = buf_str.split_whitespace().collect::<Vec<&str>>();
                        if let Some(index) = vector.iter().position(|&x| x == "mate") {
                            let mate_in = vector.get(index+1).unwrap();
                            let eval_num = mate_in.parse::<f32>().ok();
                            if let Some(e) = eval_num {
                                eval = Some(String::from("Mate in ") + &e.to_string());
                            }
                            for i in (index + 1)..vector.len() {
                                if let Some(token) = vector.get(i) {
                                    if String::from(token.deref()) == "pv" {
                                        best_move = Some(vector.get(i+1).unwrap().to_string());
                                        break;
                                    }
                                }
                            }
                        } else if let Some(index) = vector.iter().position(|&x| x == "score") {
                            let score = vector.get(index+2).unwrap();
                            let eval_num = score.parse::<f32>().ok();
                            if let Some(e) = eval_num {
                                eval = Some(format!("{:.2}",(e / 100.)));
                            }
                            for i in (index + 1)..vector.len() {
                                if let Some(token) = vector.get(i) {
                                    if String::from(token.deref()) == "pv" {
                                        best_move = Some(vector.get(i+1).unwrap().to_string());
                                        break;
                                    }
                                }
                            }
                        }
                        // Just to ping the engine and guarantee we'll have something to read,
                        // it's really stupid, but the read_line (or read) never returns if the engine
                        // don't send anything.
                        child.stdin.as_mut().unwrap().write_all(b"isready\n").expect("Error communicating with engine");
                        (Some(Message::UpdateEval((eval, best_move))), EngineState::Thinking(child, search_up_to, receiver))
                    } EngineState::TurnedOff => {
                        (Some(Message::EngineStopped), EngineState::TurnedOff)
                    }
                }
            }
        )
    }
}
