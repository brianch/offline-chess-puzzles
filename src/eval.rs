use async_std::io::prelude::BufReadExt;
use iced::{Subscription, subscription};
use lazy_static::__Deref;
use std::process::Stdio;
use async_std::channel::{unbounded, Receiver};
use async_std::process::{Command, Child};
#[cfg(target_os = "windows")]
use async_std::os::windows::process::CommandExt;
use async_std::io::{BufReader, WriteExt};
use async_std::io::timeout;
use std::time::Duration;

use crate::Message;

pub const STOP_COMMAND: &str = "STOP";
pub const EXIT_APP_COMMAND: &str = "EXIT";

pub enum EngineState {
    Start(Engine),
    Thinking(Child, String, Receiver<String>),
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
                        let (sender, receiver) = unbounded();
                        let mut cmd = Command::new(engine_data.engine_path);
                        cmd.kill_on_drop(true).stdin(Stdio::piped()).stdout(Stdio::piped());
                        #[cfg(target_os = "windows")]
                        //"CREATE_NO_WINDOW" flag
                        // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
                        cmd.creation_flags(0x08000000);
                        let mut child = cmd.spawn().expect("Error calling engine");

                        let pos = String::from("position fen ") + &engine_data.position + &String::from("\n");
                        let limit = String::from("go ") + &engine_data.search_up_to + &"\n";

                        child.stdin.as_mut().unwrap().write_all(b"uci\n").await.expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(b"isready\n").await.expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(b"ucinewgame\n").await.expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(b"setoption name UCI_AnalyseMode value true\n").await.expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(pos.as_bytes()).await.expect("Error communicating with engine");
                        child.stdin.as_mut().unwrap().write_all(limit.as_bytes()).await.expect("Error communicating with engine");

                        (Some(Message::EngineReady(sender)), EngineState::Thinking(child, engine_data.search_up_to, receiver))

                    } EngineState::Thinking(mut child, search_up_to, receiver) => {
                        let msg = receiver.try_recv();
                        if let Ok(msg) = msg {
                            if &msg == STOP_COMMAND || &msg == EXIT_APP_COMMAND {
                                drop(receiver);
                                child.stdin.as_mut().unwrap().write_all(b"stop\n").await.expect("Error communicating with engine");
                                child.stdin.as_mut().unwrap().write_all(b"quit\n").await.expect("Error communicating with engine");
                                let terminate_timeout = timeout(Duration::from_millis(50),
                                    child.status()
                                ).await;
                                if let Err(_) = terminate_timeout {
                                    eprintln!("Engine didn't quit, killing the process now...");
                                    if let Err(e) = child.kill() {
                                        eprintln!("Error killing the engine process: {e}");
                                    }
                                }
                                return (Some(Message::EngineStopped(&msg == EXIT_APP_COMMAND)), EngineState::TurnedOff);
                            } else {
                                let pos = String::from("position fen ") + &msg + &String::from("\n");
                                let limit = String::from("go ") + &search_up_to + &"\n";

                                child.stdin.as_mut().unwrap().write_all(b"stop\n").await.expect("Error communicating with engine");
                                //child.stdin.as_mut().unwrap().write_all(b"setoption name UCI_AnalyseMode value true\n").await.expect("Error communicating with engine");
                                //child.stdin.as_mut().unwrap().write_all(b"ucinewgame\n").await.expect("Error communicating with engine");
                                child.stdin.as_mut().unwrap().write_all(pos.as_bytes()).await.expect("Error communicating with engine");
                                child.stdin.as_mut().unwrap().write_all(limit.as_bytes()).await.expect("Error communicating with engine");
                            }
                        }
                        let mut buf_str = String::new();
                        let mut eval = None;
                        let mut best_move = None;

                        if let Some(out) = child.stdout.as_mut() {
                            let mut reader = BufReader::new(out);
                            loop {
                                let read_timeout = timeout(Duration::from_millis(50),
                                    reader.read_line(&mut buf_str)
                                ).await;
                                if let Ok(read_result) = read_timeout {
                                    if read_result == 0 {
                                        break;
                                    }
                                    let vector: Vec<&str> = buf_str.split_whitespace().collect::<Vec<&str>>();
                                    if let Some(index) = vector.iter().position(|&x| x == "score") {
                                        let eval_num = vector.get(index+2).unwrap().parse::<f32>().ok();
                                        if let Some(e) = eval_num {
                                            if vector.get(index+1).unwrap() == &"mate" {
                                                eval = Some(String::from("Mate in ") + &e.to_string());
                                            } else {
                                                eval = Some(format!("{:.2}",(e / 100.)));
                                            }
                                        }
                                        for i in (index + 3)..vector.len() {
                                            if let Some(token) = vector.get(i) {
                                                if String::from(token.deref()) == "pv" {
                                                    // I thought we could just unwrap, but at least Koivisto sometimes
                                                    // returns lines with nothing in the pv
                                                    if let Some(best) = vector.get(i+1) {
                                                        best_move = Some(best.to_string());
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    buf_str.clear();
                                } else {
                                    break;
                                }
                            }
                        }
                        (Some(Message::UpdateEval((eval, best_move))), EngineState::Thinking(child, search_up_to, receiver))
                    } EngineState::TurnedOff => {
                        (Some(Message::EngineStopped(false)), EngineState::TurnedOff)
                    }
                }
            }
        )
    }
}
