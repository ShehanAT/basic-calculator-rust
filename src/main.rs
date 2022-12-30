use std::collections::HashMap;
use std::result;
use iced_wgpu::Renderer;
use download::CalculatorState;
use iced::{executor, theme, subscription};
use iced::widget::{button, column, container, progress_bar, text, Column, Row};
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
    input_string: String,
    output_string: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartCalculating,
    DoneCalculating(String),
    Add,
    Equals,
    One,
    Two,
    Three,
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
                input_string: "".to_string(),
                output_string: "4 + 4".to_string(),
            },
            Command::perform(Calculator::calculate("".to_string()), Message::DoneCalculating),
        )
    }

    fn title(&self) -> String {
        String::from("Download progress - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::One => {
                self.input_string += "1";
                Command::none()
            },
            Message::Two => {
                self.input_string += "2";
                Command::none()
            },
            Message::Three => {
                self.input_string += "3";
                Command::none()
            },
            Message::Add => {
                self.input_string += "+";
                Command::none()
            },
            Message::Equals => {
                self.input_string += "=";
                let dup_str = self.input_string.clone();
                println!("Input String: {}", self.input_string);
                Command::perform(Calculator::calculate(dup_str), Message::DoneCalculating)
            },
            Message::StartCalculating => {
                println!("Message::StartCalculating");
                Command::perform(Calculator::calculate("4 + 4".to_string()), Message::DoneCalculating)
            },
            Message::DoneCalculating(result) => {
                println!("Message::DoneCalculating, Output: {}", result);
                self.output_string = result;
                Command::none()
            },
            _ => {
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(self.downloads.iter().map(Download::subscription))
    }

    fn view(&self) -> Element<Message> {

        let num_col1: Row<Message, Renderer<Backend, ..>> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::End)
            .spacing(20)
            .push(
                button("1")
                .style(theme::Button::Text)
                .on_press(Message::One)
            )
            .push(
                button("2")
                .style(theme::Button::Text)
                .on_press(Message::Two)
            )
            .push(
                button("3")
                .style(theme::Button::Text)
                .on_press(Message::Three)
            ).into();

        let num_col2 = Row::new()
        .width(Length::Units(500))
        .spacing(20)
        .push(
            button("+")
            .style(theme::Button::Text)
            .on_press(Message::Add)
        )
        .push(
            button("=")
            .style(theme::Button::Text)
            .on_press(Message::Equals)
        );

    
        let downloads = Column::with_children(
            self.downloads.iter().map(Download::view).collect(),
        )
        .push(
            button("Start Calculation!")
            .style(theme::Button::Primary)
            .on_press(Message::StartCalculating)
            
        )
        .push(
            text(format!("Output String: {}", self.output_string))
        )
        .push(
            button("1")
            .style(theme::Button::Text)
            .on_press(Message::One)
        )
        .push(
            button("2")
            .style(theme::Button::Text)
            .on_press(Message::Two)
        )
        .push(
            button("3")
            .style(theme::Button::Text)
            .on_press(Message::Three)
        )
        .push(
            button("+")
            .style(theme::Button::Text)
            .on_press(Message::Add)
        )
        .push(
            button("=")
            .style(theme::Button::Text)
            .on_press(Message::Equals)
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
    CalculatorResult { output_string: String },
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
    }

    pub fn view(&self) -> Element<Message> {
        let current_progress = match &self.state {
            State::Idle { .. } => 0.0,
            State::Downloading { progress } => *progress,
            State::Finished { .. } => 100.0,
            State::Errored { .. } => 0.0,
            State::CalculatorEvaluating { done } => todo!(),
            State::CalculatorResult { output_string } => todo!(),
        };

        let progress_bar = progress_bar(0.0..=100.0, current_progress);

        Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Alignment::Center)
            .push(progress_bar)
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

