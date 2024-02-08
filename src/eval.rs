use iced::{Subscription, subscription};
use std::process::Stdio;
use tokio::sync::mpsc::{self, Receiver};
use tokio::process::{Command, Child};
use tokio::io::{BufReader, AsyncWriteExt, AsyncBufReadExt};

use iced::futures::SinkExt;

use tokio::time::timeout;
use std::time::Duration;

use crate::Message;

pub const STOP_COMMAND: &str = "STOP";
pub const EXIT_APP_COMMAND: &str = "EXIT";

pub enum EngineState {
    Start(Engine),
    Thinking(Child, String, Receiver<String>),
    TurnedOff,
}

#[derive(PartialEq)]
pub enum EngineStatus {
    Started,
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

    pub fn run_engine(engine: Engine) -> Subscription<Message> {
        subscription::channel(
            std::any::TypeId::of::<Engine>(),
            100,
            move |mut output| {
                let engine = engine.clone();

                async move  {
                    let mut state = EngineState::Start(engine);
                    loop {
                        match &mut state {
                            EngineState::Start(engine) => {
                                let (sender, receiver) = mpsc::channel(100);
                                let mut cmd = Command::new(engine.engine_path.clone());
                                cmd.kill_on_drop(true).stdin(Stdio::piped()).stdout(Stdio::piped());
                                #[cfg(target_os = "windows")]
                                //"CREATE_NO_WINDOW" flag
                                // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
                                cmd.creation_flags(0x08000000);
                                let mut child = cmd.spawn().expect("Error calling engine");

                                let pos = String::from("position fen ") + &engine.position + &String::from("\n");
                                let limit = String::from("go ") + &engine.search_up_to + "\n";
                                let mut uciok = false;
                                let mut readyok = false;

                                child.stdin.as_mut().unwrap().write_all(b"uci\n").await.expect("Error communicating with engine");
                                let mut reader = BufReader::new(child.stdout.as_mut().unwrap());
                                let mut buf_str = String::new();
                                loop {
                                    let uciok_timeout = timeout(Duration::from_millis(7000),
                                        reader.read_line(&mut buf_str)
                                    ).await;
                                    if uciok_timeout.is_err() {
                                        break;
                                    } else if buf_str.contains("uciok") {
                                        uciok = true;
                                        break;
                                    }
                                }
                                if uciok {
                                    child.stdin.as_mut().unwrap().write_all(b"ucinewgame\n").await.expect("Error communicating with engine");
                                    child.stdin.as_mut().unwrap().write_all(b"isready\n").await.expect("Error communicating with engine");
                                    buf_str = String::new();
                                    loop {
                                        let readyok_timeout = timeout(Duration::from_millis(7000),
                                            reader.read_line(&mut buf_str)
                                        ).await;
                                        if readyok_timeout.is_err() {
                                            break;
                                        } else if buf_str.contains("readyok") {
                                            readyok = true;
                                            break;
                                        }
                                    }
                                    if readyok {
                                        child.stdin.as_mut().unwrap().write_all(b"setoption name UCI_AnalyseMode value true\n").await.expect("Error communicating with engine");
                                        child.stdin.as_mut().unwrap().write_all(pos.as_bytes()).await.expect("Error communicating with engine");
                                        child.stdin.as_mut().unwrap().write_all(limit.as_bytes()).await.expect("Error communicating with engine");

                                        output.send(Message::EngineReady(sender)).await.expect("Error on the mpsc channel in the engine subscription");
                                        state = EngineState::Thinking(child, engine.search_up_to.to_string(), receiver);
                                        continue;
                                    }
                                }
                                eprintln!("Engine took too long to start, aborting...");
                                child.stdin.as_mut().unwrap().write_all(b"stop\n").await.expect("Error communicating with engine");
                                child.stdin.as_mut().unwrap().write_all(b"quit\n").await.expect("Error communicating with engine");
                                let terminate_timeout = timeout(Duration::from_millis(1000),
                                    child.wait()
                                ).await;
                                if let Err(e) = terminate_timeout {
                                    eprintln!("Error: {e}");
                                    eprintln!("Engine didn't quit, killing the process now... ");
                                    let kill_result = timeout(Duration::from_millis(500),
                                        child.kill()
                                    ).await;
                                    if let Err(e) = kill_result {
                                        eprintln!("Error killing the engine process: {e}");
                                    }
                                }
                                output.send(Message::EngineStopped(false)).await.expect("Error on the mpsc channel in the engine subscription");
                                state = EngineState::TurnedOff;
                            } EngineState::Thinking(child, search_up_to, receiver) => {
                                let msg = receiver.try_recv();
                                if let Ok(msg) = msg {
                                    if msg == STOP_COMMAND || msg == EXIT_APP_COMMAND {
                                        child.stdin.as_mut().unwrap().write_all(b"stop\n").await.expect("Error communicating with engine");
                                        child.stdin.as_mut().unwrap().write_all(b"quit\n").await.expect("Error communicating with engine");
                                        let terminate_timeout = timeout(Duration::from_millis(1000),
                                            child.wait()
                                        ).await;
                                        if let Err(e) = terminate_timeout {
                                            eprintln!("Error: {e}");
                                            eprintln!("Engine didn't quit, killing the process now... ");
                                            let kill_result = timeout(Duration::from_millis(500),
                                                child.kill()
                                            ).await;
                                            if let Err(e) = kill_result {
                                                eprintln!("Error killing the engine process: {e}");
                                            }
                                        }
                                        output.send(Message::EngineStopped(msg == EXIT_APP_COMMAND)).await.expect("Error on the mpsc channel in the engine subscription");
                                        state = EngineState::TurnedOff;
                                        continue;
                                    } else {
                                        let pos = String::from("position fen ") + &msg + &String::from("\n");
                                        let limit = String::from("go ") + &search_up_to + "\n";
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
                                        if let Ok(Ok(read_result)) = read_timeout {
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
                                                        if token == &"pv" {
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
                                output.send(Message::UpdateEval((eval, best_move))).await.expect("Error on the mpsc channel in the engine subscription");
                            } EngineState::TurnedOff => {
                                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                            }
                        }
                    }
                }
            }
        )
    }
}
