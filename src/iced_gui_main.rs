use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
};

use std::time::{Duration, Instant};
use std::cell::Cell;

pub fn main() -> iced::Result {
    Stopwatch::run(Settings::default())
}

struct Stopwatch {
    duration: Duration,
    state: State,
    input_string: Cell<String>,
}

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
                input_string: Cell::new("".to_string()),
            },
            Command::none(),
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
                *self.input_string.get_mut() += "1";
            }
            Message::Two => {
                *self.input_string.get_mut() += "2";
            }
            Message::Three => {
                *self.input_string.get_mut() += "3";
            }
            Message::Add => {
                *self.input_string.get_mut() += "+";
            }
            Message::Subtract => {
                *self.input_string.get_mut() += "-";
            }
            _ => self.duration = Duration::default()
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        println!("Input String: {:?}", self.input_string.get_mut());
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
        let controls2 = row![add_button, subtract_button].spacing(20);
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

}
