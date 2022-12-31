use std::collections::HashMap;
use std::result;
use download::CalculatorState;
use iced::{executor, theme, subscription};
use iced::widget::{button, column, container, progress_bar, text, Column, Row, row};
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
    LeftParen,
    RightParen,
    Factorial,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    One,
    Two,
    Three,
    Four,
    Five,
    Six, 
    Seven,
    Eight,
    Nine,
    Zero,
    Negate,
    Decimal,
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
            Message::Four => {
                self.input_string += "4";
                Command::none()
            },
            Message::Five => {
                self.input_string += "5";
                Command::none()
            },
            Message::Six => {
                self.input_string += "6";
                Command::none()
            },
            Message::Seven => {
                self.input_string += "7";
                Command::none()
            },
            Message::Eight => {
                self.input_string += "8";
                Command::none()
            },
            Message::Nine => {
                self.input_string += "9";
                Command::none()
            },
            Message::Zero => { 
                self.input_string += "0";
                Command::none()
            },
            Message::Decimal => {
                self.input_string += ".";
                Command::none()
            },
            Message::Add => {
                self.input_string += " + ";
                Command::none()
            },
            Message::Subtract => {
                self.input_string += " - ";
                Command::none()
            },
            Message::Multiply => {
                self.input_string += " * ";
                Command::none()
            },
            Message::Divide => {
                self.input_string += " / ";
                Command::none()
            },
            Message::Equals => {
                self.input_string += " = ";
                let dup_str = self.input_string.clone();
                println!("Input String: {}", self.input_string);
                self.input_string = "".to_string();
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
        
        let output_text = text(format!("Output: {}", self.output_string));
        let input_text = text(format!("Input: {}", self.input_string));

        let left_paren_btn = 
            button("(")
            .style(theme::Button::Text)
            .on_press(Message::LeftParen);

        let right_paren_btn = 
            button(")")
            .style(theme::Button::Text)
            .on_press(Message::RightParen);

        let factorial_btn = 
            button("n!")
            .style(theme::Button::Text)
            .on_press(Message::Factorial);

        let add_btn = 
            button("+")
            .style(theme::Button::Text)
            .on_press(Message::Add);

        let subtract_btn = 
            button("-")
            .style(theme::Button::Text)
            .on_press(Message::Subtract);

        let multiply_btn = 
            button("x")
            .style(theme::Button::Text)
            .on_press(Message::Multiply);
        
        let divide_btn = 
            button("÷")
            .style(theme::Button::Text)
            .on_press(Message::Divide);
        
        let equals_btn = button("=")
            .style(theme::Button::Text)
            .on_press(Message::Equals);

        let one_btn = button("1")
            .style(theme::Button::Text)
            .on_press(Message::One);

        let two_btn = button("2")
            .style(theme::Button::Text)
            .on_press(Message::Two);

        let three_btn = button("3")
            .style(theme::Button::Text)
            .on_press(Message::Three);

        let four_btn = button("4")
            .style(theme::Button::Text)
            .on_press(Message::Four);;

        let five_btn = button("5")
            .style(theme::Button::Text)
            .on_press(Message::Five);

        let six_btn = button("6")
            .style(theme::Button::Text)
            .on_press(Message::Six);
        
        let seven_btn = button("7")
            .style(theme::Button::Text)
            .on_press(Message::Seven);

        let eight_btn = button("8")
            .style(theme::Button::Text)
            .on_press(Message::Eight);

        let nine_btn = button("9")
            .style(theme::Button::Text)
            .on_press(Message::Nine);
    
        let zero_btn = button("0")
            .style(theme::Button::Text)
            .on_press(Message::Zero);

        let negate_btn = button("+/-")
            .style(theme::Button::Text)
            .on_press(Message::Negate);

        let decimal_btn = button(".")
            .style(theme::Button::Text)
            .on_press(Message::Decimal);

    
        // let downloads = Column::with_children(
        //     self.downloads.iter().map(Download::view).collect(),
        // )
        // .push(
        //     button("Start Calculation!")
        //     .style(theme::Button::Primary)
        //     .on_press(Message::StartCalculating)
            
        // )
        // .push(
        //     text(format!("Output String: {}", self.output_string))
        // )
        // .push(
        //     button("1")
        //     .style(theme::Button::Text)
        //     .on_press(Message::One)
        // )
        // .push(
        //     button("2")
        //     .style(theme::Button::Text)
        //     .on_press(Message::Two)
        // )
        // .push(
        //     button("3")
        //     .style(theme::Button::Text)
        //     .on_press(Message::Three)
        // )
        // .push(
        //     button("+")
        //     .style(theme::Button::Text)
        //     .on_press(Message::Add)
        // )
        // .push(
        //     button("=")
        //     .style(theme::Button::Text)
        //     .on_press(Message::Equals)
        // )
        // .spacing(20)
        // .align_items(Alignment::End);

        // let controls = row![toggle_button, reset_button].spacing(20);

        let first_row = row![input_text];
        let second_row = row![output_text];
        let third_row = row![left_paren_btn, right_paren_btn, factorial_btn, divide_btn].spacing(20);
        let fourth_row = row![seven_btn, eight_btn, nine_btn, multiply_btn].spacing(20);
        let fifth_row = row![four_btn, five_btn, six_btn, subtract_btn].spacing(20);
        let sixth_row = row![one_btn, two_btn, three_btn, add_btn].spacing(20);
        let seventh_row = row![negate_btn, zero_btn, decimal_btn, equals_btn].spacing(20);

        let content = column![first_row, second_row, third_row, fourth_row, fifth_row, sixth_row, seventh_row]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content)    
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

