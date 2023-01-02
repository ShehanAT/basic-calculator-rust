use std::collections::HashMap;
use iced::window::{Position, Icon};
use iced::{executor, theme, Point, Size};
use iced::widget::{button, column, container, text, row};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Theme, window
}; 
use iced_graphics::widget::canvas::{
    Canvas, Cursor, Frame, Geometry, Path, Stroke,
};


mod parser;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (200, 400),
            resizable: true,
            decorations: true,
            position: Position::Default,
            min_size: Some((300, 400)),
            max_size: Some((200, 400)),
            visible: true,
            transparent: false,
            always_on_top: true,
            icon: Some(Icon::from_rgba(vec![0,0,0,0], 1, 1).unwrap()),
        },
        ..Default::default()
    };
    CalculatorGUI::run(settings)
}

#[derive(Debug)]
struct CalculatorGUI {
    display_text: String,
    done_calculation: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartCalculating,
    DoneCalculating(String),
    CE,
    Del,
    Caret,
    Mod,
    Sin,
    Cos,
    Tan,
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

impl Application for CalculatorGUI {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (CalculatorGUI, Command<Message>) {
        (
            CalculatorGUI {
                display_text: "".to_string(),
                done_calculation: true,
            },
            Command::perform(Calculator::calculate("".to_string()), Message::DoneCalculating),
        )
    }

    fn title(&self) -> String {
        String::from("Calculator Application")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::One => {
                if self.done_calculation {
                    self.display_text = "1".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "1";
                }
                
                Command::none()
            },
            Message::Two => {
                if self.done_calculation {
                    self.display_text = "2".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "2";
                }
                Command::none()
            },
            Message::Three => {
                if self.done_calculation {
                    self.display_text = "3".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "3";
                }
                Command::none()
            },
            Message::Four => {
                if self.done_calculation {
                    self.display_text = "4".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "4";
                }
                Command::none()
            },
            Message::Five => {
                if self.done_calculation {
                    self.display_text = "5".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "5";
                }
                Command::none()
            },
            Message::Six => {
                if self.done_calculation {
                    self.display_text = "6".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "6";
                }
                Command::none()
            },
            Message::Seven => {
                if self.done_calculation {
                    self.display_text = "7".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "7";
                }
                Command::none()
            },
            Message::Eight => {
                if self.done_calculation {
                    self.display_text = "8".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "8";
                }
                Command::none()
            },
            Message::Nine => {
                if self.done_calculation {
                    self.display_text = "9".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "9";
                }
                Command::none()
            },
            Message::Zero => { 
                if self.done_calculation {
                    self.display_text = "0".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "0";
                }
                Command::none()
            },
            Message::Decimal => {
                if self.done_calculation {
                    self.display_text = ".".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += ".";
                }
                Command::none()
            },
            Message::Add => {
                self.display_text += " + ";
                if self.done_calculation {
                    self.done_calculation = false;
                }
                Command::none()
            },
            Message::Subtract => {
                self.display_text += " - ";
                if self.done_calculation {
                    self.done_calculation = false;
                }
                Command::none()
            },
            Message::Multiply => {
                self.display_text += " * ";
                if self.done_calculation {
                    self.done_calculation = false;
                }
                Command::none()
            },
            Message::Divide => {
                self.display_text += " / ";
                if self.done_calculation {
                    self.done_calculation = false;
                }
                Command::none()
            },
            Message::Mod => {
                self.display_text += " % ";
                if self.done_calculation {
                    self.done_calculation = false;
                }
                Command::none()
            },
            Message::CE => {
                self.display_text = "".to_string();
                Command::none()
            },
            Message::Del => {
                let mut temp_display_text = self.display_text.to_string();
                temp_display_text.pop();
                self.display_text = temp_display_text;
                Command::none()
            },
            Message::Caret => {
                self.display_text += " ^ ";
                if self.done_calculation {
                    self.done_calculation = false;
                }
                Command::none()
            },
            Message::Sin => {
                self.display_text = format!("sin({})", self.display_text);
                Command::none()
            },
            Message::Cos => {
                self.display_text = format!("cos({})", self.display_text);
                Command::none()
            },
            Message::Tan => {
                self.display_text = format!("tan({})", self.display_text);
                Command::none()
            },
            Message::LeftParen => {
                if self.done_calculation {
                    self.display_text = "(".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += "(";
                }
                Command::none()
            },
            Message::RightParen => {
                if self.done_calculation {
                    self.display_text = ")".to_string();
                    self.done_calculation = false;
                } else {
                    self.display_text += ")";
                }
                Command::none()
            },
            Message::Negate => {
                self.display_text = format!("(-{})", self.display_text);
                Command::none()
            },
            Message::Factorial => {
                self.display_text = format!("fact({})", self.display_text);
                Command::none()
            },
            Message::Equals => {
                let dup_str = self.display_text.clone();
                self.display_text = "".to_string();
                Command::perform(Calculator::calculate(dup_str), Message::DoneCalculating)
            },
            Message::StartCalculating => {
                Command::perform(Calculator::calculate("4 + 4".to_string()), Message::DoneCalculating)
            },
            Message::DoneCalculating(result) => {
                self.display_text = result;
                self.done_calculation = true;
                Command::none()
            },
            _ => {
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let display_text = text(format!("{}", self.display_text));

        let ce_btn = button("CE")
        .style(theme::Button::Text)
        .on_press(Message::CE);

        let del_btn = button("DEL")
        .style(theme::Button::Text)
        .on_press(Message::Del);

        let caret_btn = button("^")
        .style(theme::Button::Text)
        .on_press(Message::Caret);

        let mod_btn = button("%")
        .style(theme::Button::Text)
        .on_press(Message::Mod);

        let sin_btn = button("Sin")
        .style(theme::Button::Text)
        .on_press(Message::Sin);

        let cos_btn = button("Cos")
        .style(theme::Button::Text)
        .on_press(Message::Cos);

        let tan_btn = button("Tan")
        .style(theme::Button::Text)
        .on_press(Message::Tan);

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

        let display_frame = Frame::new(Size { width: 500.0, height: 500.0 });
        display_frame.stroke(
            &Path::rectangle(Point::ORIGIN, display_frame.size()),
            Stroke::default().with_width(2.0)
        );
        display_frame.fill_text(display_text.into());
        // display_frame
        //  {
        //         Curve::draw_all(self.curves, frame);

        //         frame.stroke(
        //             &Path::rectangle(Point::ORIGIN, frame.size()),
        //             Stroke::default().with_width(2.0),
        // );
        // };
        let first_row = row![display_frame].spacing(20).padding(10);
        // let second_row = row![output_text];
        // let third_row = row![].spacing(20);
        let fourth_row = row![sin_btn, cos_btn, tan_btn, del_btn, ce_btn].spacing(5);
        let fifth_row = row![factorial_btn, mod_btn, negate_btn, caret_btn, left_paren_btn, right_paren_btn].spacing(13);
        let sixth_row = row![seven_btn, eight_btn, nine_btn, divide_btn].spacing(20);
        let seventh_row = row![four_btn, five_btn, six_btn, multiply_btn].spacing(20);
        let eighth_row = row![one_btn, two_btn, three_btn, subtract_btn].spacing(20);
        let ninth_row = row![equals_btn, zero_btn, decimal_btn, add_btn].spacing(20);

        let content = column![first_row, fourth_row, fifth_row, sixth_row, seventh_row, eighth_row, ninth_row]
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

pub struct Calculator {
    pub input_string: String,
    pub output_string: String,
}

impl Calculator {


    
    pub async fn calculate(
        input_string: String,
    ) -> String {
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

