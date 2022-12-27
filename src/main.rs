use iced::Renderer;
use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
};
use iced_native;
use std::time::{Duration, Instant};
use std::cell::Cell;


use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
pub mod parser;


pub fn main() -> iced::Result {
    Stopwatch::run(Settings::default())
}

// #[derive(Debug)]
struct Stopwatch {
    duration: Duration,
    state: State,
    // input_string: Cell<String>,
    input_string: InputString,
    // input_string: str,

}

#[derive(Debug, Clone)]
struct CalculatorLogic {
    input_string: InputString,
}

impl CalculatorLogic {

    async fn evaluate(input: &str, env: &mut HashMap<String, f64>) -> Result<f64, String> {
        let mut p = parser::Parser::new(input);
        let ast = p.parse()?;
        match ast.eval(env) {
            Some(result) => Ok(result),
            None => Err("No value for that expression!".to_string())
        }
    }

    async fn evaluate_expr(input_string: str) {
        use std::f64;
        let mut env = HashMap::new();
        env.insert("wow".to_string(), 35.0f64);
        env.insert("pi".to_string(), f64::consts::PI);
    
        let stdin = io::stdin();
    
        let mut input = input_string;
    
        // stdin.read_line(&mut print_str);
    
        match stdin.read_line(&mut input) {
            // println!("Input: {}", print_str);
            Ok(_) => {
    
                if input.len() == 0 {
                    println!("");
                    return;
                }
    
                let expression_text = input.trim_right();
    
                let result = Self::evaluate(expression_text, &mut env);
                match result {
                    Ok(value) => {
                        println!("=> {}", value);
                    }
                    Err(s) => {
                        println!("Error: {}", s);
                    }
                }
                io::stdout().flush().ok();
            }
            Err(_) => {
                println!("");
                return;
            }
        }
    }
}

#[derive(Debug)]
struct InputString {
    input_string: String,
}

// impl Copy for Cell<String> {}

// #[derive(Debug, Clone)]
enum State {
    Idle,
    Ticking { last_tick: Instant },
    
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Reset,
    Tick(Instant),
    Add,
    Subtract,
    Divide,
    Multiply,
    Number,
    One,
    Two,
    Three,
    Evaluate,
    Done,
}

impl Application for Stopwatch {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Stopwatch, Command<Message>) {
        (
            Stopwatch {
                duration: Duration::default(),
                state: State::Idle,
                input_string: InputString { input_string: 
                    // Cell::new("".to_string()) 
                    String::from("".to_string())
                },
                // input_string: String::new().as_str(),
            },
            // Command::none(),
            Command::perform(CalculatorLogic::evaluate_expr(""), Message::Done),
        )
    }

    fn title(&self) -> String {
        String::from("Stopwatch - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                }
            },
            Message::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
            }
            Message::Reset => {
                self.duration = Duration::default();
            }
            Message::One => {
                // *self.input_string.input_string.get_mut() += "1";
                self.input_string.input_string += "1";
                // self.input_string.input_string += "1";
            }
            Message::Two => {
                // *self.input_string.input_string.get_mut() += "2";
                self.input_string.input_string += "2";
            }
            Message::Three => {
                // *self.input_string.input_string.get_mut() += "3";
                self.input_string.input_string += "3";
            }
            Message::Add => {
                // *self.input_string.input_string.get_mut() += "+";
                self.input_string.input_string += "+";
            }
            Message::Subtract => {
                // *self.input_string.input_string.get_mut() += "-";
                self.input_string.input_string += "-";
            }
            Message::Evaluate => {
                Command::perform(CalculatorLogic::evaluate_expr(self.input_string.input_string), Message::Done)
            }
            _ => self.duration = Duration::default()
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        // let input_string = self.input_string.input_string
        println!("Input String: {}", self.input_string.input_string);
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        }
    }

   

    fn view(&self) -> Element<Message> {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let seconds = self.duration.as_secs();

        let duration = text(format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.duration.subsec_millis() / 10,
        ))
        .size(40);

        let button = |label| {
            button(
                text(label).horizontal_alignment(alignment::Horizontal::Center),
            )
            .padding(10)
            .width(Length::Units(80))
        };

        let toggle_button = {
            let label = match self.state {
                State::Idle => "Start",
                State::Ticking { .. } => "Stop"
            };

            button(label).on_press(Message::Toggle)
        };

        let add_button = button("+")
            .style(theme::Button::Primary)
            .on_press(Message::Add);

        let subtract_button = button("-")
            .style(theme::Button::Primary)
            .on_press(Message::Subtract);

        let equal_button = button("=")
            .style(theme::Button::Primary)
            .on_press(Message::Evaluate);

        let one_button = button("1")
            .style(theme::Button::Secondary)
            .on_press(Message::One);

        let two_button = button("2")
            .style(theme::Button::Secondary)
            .on_press(Message::Two);
        
        let three_button = button("3")
            .style(theme::Button::Secondary)
            .on_press(Message::Three);

        let reset_button = button("Reset")
            .style(theme::Button::Destructive)
            .on_press(Message::Reset);

        let controls = row![toggle_button, reset_button].spacing(20);
        let controls2 = row![add_button, subtract_button, equal_button].spacing(20);
        let controls3 = row![one_button, two_button, three_button].spacing(20);

        let content = column![duration, controls, controls2, controls3]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }
  

}



