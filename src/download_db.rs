use iced::futures::Stream;
use iced::stream;
use iced::futures::sink::SinkExt;
use iced::Subscription;
use std::fs::File;
use std::io::{Seek, Write};
use std::io::Cursor;
use std::fs::OpenOptions;
use zstd;

use crate::Message;
use crate::config::PUZZLES_DIRECTORY;

pub enum DownloadState {
    StartDownload {
        url: String,
        path: String,
    },
    DownloadInProgress {
        response: reqwest::Response,
        file: File,
        total: u64,
        downloaded: u64,
    },
    Finished,
}

pub fn download_lichess_db(url: String, path: String) -> Subscription<Message> {
    Subscription::run_with_id(std::any::TypeId::of::<DownloadState>(), download_stream(url, path))
}

fn download_stream(url: String, path: String) -> impl Stream<Item = Message> {
    let lichess_zst_file = String::from(PUZZLES_DIRECTORY) + "lichess_db_puzzle.csv.zst";
    stream::channel(100,
        |mut output| async move  {
            let mut state = DownloadState::StartDownload{ url , path: path.clone()};
            loop {
                match state {
                    DownloadState::StartDownload { url, path : _ } => {
                        let response = reqwest::get(url.clone()).await;

                        match response {
                            Ok(response) => {
                                if let Some(total) = response.content_length() {
                                    let file = OpenOptions::new().append(true).read(true).create(true).open(&lichess_zst_file).expect("Unable to create lichess db archive.");
                                    state = DownloadState::DownloadInProgress {
                                        response,
                                        file,
                                        total,
                                        downloaded: 0
                                    };
                                } else {
                                    state = DownloadState::Finished;
                                }
                            }
                            Err(_) => state = DownloadState::Finished,
                        }
                    } DownloadState::DownloadInProgress { mut response, mut file, total, downloaded } => {
                        match response.chunk().await {
                            Ok(Some(chunk)) => {
                                let downloaded = downloaded + chunk.len() as u64;
                                let percentage = (downloaded as f32 / total as f32) * 100.0;

                                let mut content =  Cursor::new(chunk);
                                std::io::copy(&mut content, &mut file).expect("Error writing to the lichess db archive.");

                                let _ = output.send(Message::DownloadProgress(format!(" {:.2}%", percentage))).await;
                                state = DownloadState::DownloadInProgress {
                                    response,
                                    file,
                                    total,
                                    downloaded: downloaded,
                                };
                            }
                            Ok(None) =>  {
                                file.flush().expect("Error flushing lichess db archive file.");
                                file.rewind().expect("Error rewinding lichess db archive file.");

                                let target = std::fs::File::create(path.clone()).expect(&("Error creating file ".to_owned() + &path));
                                zstd::stream::copy_decode(file, target).unwrap();

                                let _ = std::fs::remove_file(&lichess_zst_file);
                                let _ = output.send(Message::DBDownloadFinished).await;

                                state = DownloadState::Finished;
                            }
                            Err(_) => state = DownloadState::Finished,
                        }
                    } DownloadState::Finished => {
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    }
                }
            }
        }
    )
}
