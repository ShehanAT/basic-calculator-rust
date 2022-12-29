use std::collections::HashMap;
use std::result;

use download::CalculatorState;
use iced::{executor, theme, subscription};
use iced::widget::{button, column, container, progress_bar, text, Column};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};

mod download;
mod parser;

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Debug)]
struct Example {
    downloads: Vec<Download>,
    single_download: Download,
    last_id: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartCalculating,
    DoneCalculating(String),
}

impl Application for Example {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Example, Command<Message>) {
        (
            Example {
                downloads: vec![Download::new(0)],
                last_id: 0,
                single_download: Download::new(10),
            },
            Command::perform(Calculator::calculate("4 + 4".to_string()), Message::DoneCalculating),
        )
    }

    fn title(&self) -> String {
        String::from("Download progress - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        println!("Passing update(), message: {:?}", message);
        match message {
            Message::StartCalculating => {
                
                Command::perform(Calculator::calculate("4 + 4".to_string()), Message::DoneCalculating)
            }
            Message::DoneCalculating(result) => {
                println!("Message::DoneCalculating, Output: {}", result);
                Command::none()
                // println!("{:?}", progress);
            },
            _ => {
                Command::none()
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        // println!("Passing subscription()");
        // println!("{:?}", self.single_download);
        // Command::perform(Calculator::calculate("4 + 4".to_string()), Message::DoneCalculating);
        Subscription::batch(self.downloads.iter().map(Download::subscription))
    }

    fn view(&self) -> Element<Message> {
        let downloads = Column::with_children(
            self.downloads.iter().map(Download::view).collect(),
        )
        .push(
            button("Start Calculation!")
            .style(theme::Button::Primary)
            .on_press(Message::StartCalculating)
        )
        .spacing(20)
        .align_items(Alignment::End);

        container(downloads)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }
}

#[derive(Debug)]
struct Download {
    id: usize,
    state: State,
}

#[derive(Debug)]
enum State {
    Idle,
    Downloading { progress: f32 },
    Finished,
    Errored,
    CalculatorEvaluating { done: bool },
    CalculatorResult { input_string: String },
}

impl Download {
    pub fn new(id: usize) -> Self {
        Download {
            id,
            state: State::Idle,
        }
    }

    pub fn start(&mut self) {
        match self.state { // This is match statement that sets the state from Idle, Finishing or Errored to Downloading 
            State::Idle { .. }
            | State::Finished { .. }
            | State::Errored { .. } => {
                self.state = State::Downloading { progress: 0.0 };
            }
            _ => {}
        }
    }

    // pub fn start_calculating(&mut self) {
    //     match self.state {
    //         State::Idle { .. }
    //         | State::Finished { .. }
    //         | State::Errored { .. } => {

    //             Command::perform(Calculator::calculate("4 + 4".to_string()), Message::DoneCalculating);
    //             self.state = State::CalculatorEvaluating { done: false };
    //         }
    //         _ => {}
    //     }

    //     // if let State::CalculatorEvaluating { done } = &mut self.state {
    //     //     println!("State::CalculatorEvaluating");
    //     //     download::download_start_calculating(10, "4 + 4")
    //     //     .map(Message::DoneCalculating);
         

    //     // }
    // }

    pub fn progress(&mut self, new_progress: download::Progress) {
        if let State::Downloading { progress } = &mut self.state {
            match new_progress {
                download::Progress::Started => {
                    *progress = 0.0;
                }
                download::Progress::Advanced(percentage) => {
                    *progress = percentage;
                }
                download::Progress::Finished => {
                    self.state = State::Finished;
                }
                download::Progress::Errored => {
                    self.state = State::Errored;
                }
                download::Progress::CalculationFinished(_) => todo!(),
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
        // match self.state {
        //     State::Downloading { .. } => {
        //         download::file(self.id, "https://speed.hetzner.de/100MB.bin?")
        //             .map(Message::DownloadProgressed)
        //     },
        //     State::CalculatorEvaluating { .. } => {
        //         todo!();
        //         // download::download_start_calculating(0, "4 + 4")
        //         //     .map(Message::DoneCalculating)
        //     }
        //     _ => Subscription::none(),
        // }
    }

    pub fn view(&self) -> Element<Message> {
        let current_progress = match &self.state {
            State::Idle { .. } => 0.0,
            State::Downloading { progress } => *progress,
            State::Finished { .. } => 100.0,
            State::Errored { .. } => 0.0,
            State::CalculatorEvaluating { done } => todo!(),
            State::CalculatorResult { input_string } => todo!(),
        };

        let progress_bar = progress_bar(0.0..=100.0, current_progress);

        // let calculator_start_btn = 

        // let control2: Element<_> = match &self.state {
        //     State::Idle => button("Start Calculation!")
        //     .style(theme::Button::Primary)
        //     .on_press(Message::StartCalculating)
        //     .into(),
        //     State::Downloading { progress } => todo!(),
        //     State::Finished => todo!(),
        //     State::Errored => todo!(),
        //     State::CalculatorEvaluating { done } => todo!(),
        //     State::CalculatorResult { input_string } => todo!(),

        // };
        // let control: Element<_> = match &self.state {
        //     State::Idle => button("Start the download!")
        //         .on_press(Message::StartCalculating)
        //         .into(),
        //     State::Finished => {
        //         column!["Download finished!", button("Start again")]
        //             .spacing(10)
        //             .align_items(Alignment::Center)
        //             .into()
        //     }
        //     State::Downloading { .. } => {
        //         text(format!("Downloading... {:.2}%", current_progress)).into()
        //     }
        //     State::Errored => column![
        //         "Something went wrong :(",
        //         button("Try again").on_press(Message::Download(self.id)),
        //     ]
        //     .spacing(10)
        //     .align_items(Alignment::Center)
        //     .into(),
        //     State::CalculatorEvaluating { done } => todo!(),
        //     State::CalculatorResult { input_string } => todo!(),
        // };

        Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Alignment::Center)
            .push(progress_bar)
            // .push(control)
            // .push(control2)
            .into()
    }
}


pub struct Calculator {
    pub input_string: String,
    pub output_string: String,
}

impl Calculator {


    
    pub async fn calculate(
        input_string: String,
    ) -> String {

        println!("Passing single_download.calculate()");

                
        let result_output = Self::evaluate_expr(&input_string).await;
        match result_output {
            Ok(result) => result.to_string(),
            Err(result) => result
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
    
        let result = Self::evaluate(expression_text, &mut env);
        match result.await {
            Ok(value) => {
                Ok(value)
            }
            Err(s) => {
                Err(s)
            }
        }
    }

}

