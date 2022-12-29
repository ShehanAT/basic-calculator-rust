use iced_native::subscription;

use std::borrow::Borrow;
use std::{hash::Hash, collections::HashMap};

use std::io;
use std::io::prelude::*;
use std::fmt::Debug;
use crate::parser;


// Just a little utility function
pub fn file<I: 'static + Hash + Copy + Send + Sync, T: ToString>(
    id: I,
    input_string: T,
) -> iced::Subscription<(I, Progress)> {
    subscription::unfold(id, State::Ready(input_string.to_string()), move |state| {
        download(id, state)
    })
}

pub fn download_start_calculating<I: 'static + Hash + Copy + Send + Sync, T: ToString>(
    id: I, 
    input_string: T,
) -> iced::Subscription<(I, Progress)> {
    // println!("Passing download_start_calculating(), id: {:?}, input_string: {:?}", id, input_string.to_string());
    subscription::unfold(id, CalculatorState::Ready(input_string.to_string()), move |state| {
        println!("Passing subscription::unfold");
        calculate(id, state);
        // download(id, state)
    })
}


#[derive(Debug, Hash, Clone)]
pub struct Download<I> {
    id: I,
    url: String,
}

async fn calculate<I: Copy>(
    id: I,
    state: CalculatorState,
) -> (Option<(I, Progress)>, CalculatorState) {

    println!("Passing single_download.calculate()");
    match state {
        CalculatorState::Ready(input_string) => {
            
            let result_output = evaluate_expr(&input_string).await;
            // println!("{:?}", result_output.unwrap());
            // let response = reqwest::get("https://speed.hetzner.de/100MB.bin?").await;

            match result_output {
                Ok(result_output) => {
                    (Some((id, Progress::CalculationFinished(String::from(result_output.to_string().as_str())))), CalculatorState::Done(String::from(result_output.to_string().as_str())))
                    // CalculatorState::Done(String::from(result_output.to_string().as_str()))

                },
                Err(result_output) => {
                    (Some((id, Progress::CalculationFinished(result_output))), CalculatorState::Done(String::from("Error!")))

                    // CalculatorState::Done(String::from("Error"))
                }
            }

        },
        CalculatorState::Downloading { result } => {
            let response = reqwest::get("https://file-examples.com/wp-content/uploads/2017/02/file-sample_1MB.doc").await;

            match response {
                Ok(response) => {
                    if let Some(total) = response.content_length() {
                        (
                            Some((id, Progress::Started)),
                            CalculatorState::Downloading { result: (0) }, 
                            // Some((id, Progress::Finished)), State::Finished
                            Some((id, Progress::CalculationFinished(result_output.unwrap().to_string()))), State::Finished
                            // Some((id, Progress::CalculationFinished(result_output.unwrap_err())), State::Finished
                        )
                    } else {
                        (Some((id, Progress::Errored)), State::Finished)
                    }
                }
                Err(_) => (Some((id, Progress::Errored)), State::Finished),
            }
        },

        CalculatorState::Done(output_string) => {
            // CalculatorState::Done(output_string)
            (Some((id, Progress::CalculationFinished(output_string))), CalculatorState::Finished)
        },
        CalculatorState::Finished => {
            iced::futures::future::pending().await
        },
       
    }

}

async fn download<I: Copy>(
    id: I,
    state: State,
) -> (Option<(I, Progress)>, State) {
 
    match state {
        State::Ready(input_string) => {
            
            let result_output = evaluate_expr(&input_string).await;
            // println!("{:?}", result_output.unwrap());
            let response = reqwest::get("https://speed.hetzner.de/100MB.bin?").await;

            match response {
                Ok(response) => {
                    if let Some(total) = response.content_length() {
                        (
                            // Some((id, Progress::Started)),
                            // State::Downloading {
                            //     response,
                            //     total,
                            //     downloaded: 0,
                            //     // result: result_output.unwrap(),
                            // },
                            // Some((id, Progress::Finished)), State::Finished
                            Some((id, Progress::CalculationFinished(result_output.unwrap().to_string()))), State::Finished
                            // Some((id, Progress::CalculationFinished(result_output.unwrap_err())), State::Finished
                        )
                    } else {
                        (Some((id, Progress::Errored)), State::Finished)
                    }
                }
                Err(_) => (Some((id, Progress::Errored)), State::Finished),
            }
        }
        State::Downloading {
            mut response,
            total,
            downloaded,
            // result
        } => match response.chunk().await {
            
            Ok(Some(chunk)) => {
                let downloaded = downloaded + chunk.len() as u64;

                let percentage = (downloaded as f32 / total as f32) * 100.0;

                (
                    Some((id, Progress::Advanced(percentage))),
                    State::Downloading {
                        response,
                        total,
                        downloaded,
                        // result
                    },
                )
            }
            Ok(None) => (Some((id, Progress::Finished)), State::Finished),
            Err(_) => (Some((id, Progress::Errored)), State::Finished),
        },
        State::Finished => {
            // We do not let the stream die, as it would start a
            // new download repeatedly if the user is not careful
            // in case of errors.
            iced::futures::future::pending().await
        }
    }
}


async fn evaluate(input: &str, env: &mut HashMap<String, f64>) -> Result<f64, String> {
    let mut p = parser::Parser::new(input);
    let ast = p.parse()?;
    match ast.eval(env) {
        Some(result) => Ok(result),
        None => Err("No value for that expression!".to_string())
    }
}

async fn evaluate_expr(input_string: &str) -> Result<f64, String> {
    use std::f64;
    let mut env = HashMap::new();
    env.insert("wow".to_string(), 35.0f64);
    env.insert("pi".to_string(), f64::consts::PI);

 
    print!(">> ");
    

    let mut input = input_string;


    let expression_text = input.trim_right();

    let result = evaluate(expression_text, &mut env);
    match result.await {
        Ok(value) => {
            Ok(value)
        }
        Err(s) => {
            Err(s)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    CalculationFinished(String),
    Errored,
}

pub enum State {
    Ready(String),
    Downloading {
        response: reqwest::Response,
        total: u64,
        downloaded: u64,
        // result: f64,
    },
    Finished,
}

pub enum CalculatorState {
    Ready(String),
    Done(String),
    Downloading {
        result: f64,
        // result: f64,
    },
    Finished,
}