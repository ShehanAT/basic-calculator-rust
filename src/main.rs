use std::collections::HashMap;

use iced::window::{Position, Icon};
use iced::{executor, alignment};
use iced::widget::{button, column, container, text, row};
use iced::{
    Alignment, Application, Command, Length, Settings, window
}; 
use self::theme::Theme;
use self::widget::Element;


mod parser;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (300, 500),
            resizable: true,
            decorations: true,
            position: Position::Default,
            min_size: Some((300, 500)),
            max_size: Some((300, 500)),
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
    Typing,
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
        let title = text("Calculator")
        .width(Length::Fill)
        .size(20)
        .horizontal_alignment(alignment::Horizontal::Center);
        
        let display_text = button(text(format!("{}", self.display_text)))
            .style(theme::Button::Primary)
            .width(Length::Fill);

        let ce_btn = button("CE")
        .style(theme::Button::Secondary)
        .on_press(Message::CE);

        let del_btn = button("DEL")
        .style(theme::Button::Secondary)
        .on_press(Message::Del);

        let caret_btn = button("^")
        .style(theme::Button::Secondary)
        .on_press(Message::Caret);

        let mod_btn = button("%")
        .style(theme::Button::Secondary)
        .on_press(Message::Mod);

        let sin_btn = button("Sin")
        .style(theme::Button::Secondary)
        .on_press(Message::Sin);

        let cos_btn = button("Cos")
        .style(theme::Button::Secondary)
        .on_press(Message::Cos);

        let tan_btn = button("Tan")
        .style(theme::Button::Secondary)
        .on_press(Message::Tan);

        let left_paren_btn = 
            button("(")
            .style(theme::Button::Secondary)
            .on_press(Message::LeftParen);

        let right_paren_btn = 
            button(")")
            .style(theme::Button::Secondary)
            .on_press(Message::RightParen);

        let factorial_btn = 
            button("n!")
            .style(theme::Button::Secondary)
            .on_press(Message::Factorial);

        let add_btn = 
            button("+")
            .style(theme::Button::Secondary)
            .on_press(Message::Add);

        let subtract_btn = 
            button("-")
            .style(theme::Button::Secondary)
            .on_press(Message::Subtract);

        let multiply_btn = 
            button("x")
            .style(theme::Button::Secondary)
            .on_press(Message::Multiply);
        
        let divide_btn = 
            button("÷")
            .style(theme::Button::Secondary)
            .on_press(Message::Divide);
        
        let equals_btn = button("=")
            .style(theme::Button::Secondary)
            .on_press(Message::Equals);

        let one_btn = button("1")
            .style(theme::Button::Secondary)
            .on_press(Message::One);

        let two_btn = button("2")
            .style(theme::Button::Secondary)
            .on_press(Message::Two);

        let three_btn = button("3")
            .style(theme::Button::Secondary)
            .on_press(Message::Three);

        let four_btn = button("4")
            .style(theme::Button::Secondary)
            .on_press(Message::Four);;

        let five_btn = button("5")
            .style(theme::Button::Secondary)
            .on_press(Message::Five);

        let six_btn = button("6")
            .style(theme::Button::Secondary)
            .on_press(Message::Six);
        
        let seven_btn = button("7")
            .style(theme::Button::Secondary)
            .on_press(Message::Seven);

        let eight_btn = button("8")
            .style(theme::Button::Secondary)
            .on_press(Message::Eight);

        let nine_btn = button("9")
            .style(theme::Button::Secondary)
            .on_press(Message::Nine);
    
        let zero_btn = button("0")
            .style(theme::Button::Secondary)
            .on_press(Message::Zero);

        let negate_btn = button("+/-")
            .style(theme::Button::Secondary)
            .on_press(Message::Negate);

        let decimal_btn = button(".")
            .style(theme::Button::Secondary)
            .on_press(Message::Decimal);
            
        let first_row = row![title].spacing(20).padding(10).align_items(Alignment::Start);
        let second_row = row![display_text].padding(32).align_items(Alignment::Start);
        let fourth_row = row![sin_btn, cos_btn, tan_btn, del_btn, ce_btn].spacing(5);
        let fifth_row = row![factorial_btn, mod_btn, negate_btn, caret_btn, left_paren_btn, right_paren_btn].spacing(13);
        let sixth_row = row![seven_btn, eight_btn, nine_btn, divide_btn].spacing(20);
        let seventh_row = row![four_btn, five_btn, six_btn, multiply_btn].spacing(20);
        let eighth_row = row![one_btn, two_btn, three_btn, subtract_btn].spacing(20);
        let ninth_row = row![equals_btn, zero_btn, decimal_btn, add_btn].spacing(20);

        let content = column![first_row, second_row, fourth_row, fifth_row, sixth_row, seventh_row, eighth_row, ninth_row]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .style(theme::Container::Bordered)
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


mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
}

mod theme {
    use iced::widget::{button, container, text, row};
    use iced::{application, color, Color};

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Theme;

    impl application::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: &Self::Style) -> application::Appearance {
            application::Appearance {
                background_color: color!(0x28, 0x28, 0x28),
                text_color: color!(0xeb, 0xdb, 0xb2),
            }
        }
    }

    impl text::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: Self::Style) -> text::Appearance {
            text::Appearance {
                color: color!(0xeb, 0xdb, 0xb2).into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Container {
        #[default]
        Default,
        Bordered,
    }

    impl container::StyleSheet for Theme {
        type Style = Container;

        fn appearance(&self, style: &Self::Style) -> container::Appearance {
            match style {
                Container::Default => container::Appearance::default(),
                Container::Bordered => container::Appearance {
                    border_color: color!(0x45, 0x85, 0x88),
                    border_width: 1.0,
                    border_radius: 4.0,
                    ..Default::default()
                },
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Button {
        #[default]
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Theme {
        type Style = Button;

        fn active(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Button::Primary => button::Appearance {
                    background: color!(0x28, 0x28, 0x28).into(),
                    border_radius: 4.0,
                    border_width: 1.0,
                    border_color: color!(0x45, 0x85, 0x88),
                    ..Default::default()
                },
                Button::Secondary => button::Appearance {
                    background: color!(0x3c, 0x38, 0x36).into(),
                    ..Default::default()
                },
            }
        }
    }
}

