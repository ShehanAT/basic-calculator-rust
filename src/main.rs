use iced::{executor, theme};
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
    Add,
    Download(usize),
    DownloadProgressed((usize, download::Progress)),
    StartCalculating,
    DoneCalculating((usize, download::Progress)),
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
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Download progress - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Add => {
                self.last_id += 1;

                self.downloads.push(Download::new(self.last_id));
            }
            Message::Download(index) => {
                if let Some(download) = self.downloads.get_mut(index) {
                    download.start();
                }
            }
            Message::DownloadProgressed((id, progress)) => {
                if let Some(download) =
                    self.downloads.iter_mut().find(|download| download.id == id)
                {
                    download.progress(progress);
                }
            }
            Message::StartCalculating => {
                println!("Message::StartCalculating");
                self.single_download.start_calculating();
            }
            Message::DoneCalculating((id, progress)) => {
                println!("Message::DoneCalculating");
                println!("{:?}", progress);
            },
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        println!("Passing subscription()");
        println!("{:?}", self.single_download);
        Subscription::batch(self.downloads.iter().map(Download::subscription))
    }

    fn view(&self) -> Element<Message> {
        let downloads = Column::with_children(
            self.downloads.iter().map(Download::view).collect(),
        )
        .push(
            button("Add another download")
                .on_press(Message::Add)
                .padding(10),
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

    pub fn start_calculating(&mut self) {
        match self.state {
            State::Idle { .. }
            | State::Finished { .. }
            | State::Errored { .. } => {
                self.state = State::CalculatorEvaluating { done: false };
            }
            _ => {}
        }

        if let State::CalculatorEvaluating { done } = &mut self.state {
            println!("State::CalculatorEvaluating");
            download::download_start_calculating(10, "4 + 4")
            .map(Message::DoneCalculating);
            // if *done {
            //     match new_progress {
            //         download::Progress::CalculationFinished(result_output) => {
            //             println!("CalculationFinished. Output: {}", result_output);
            //             self.state = State::CalculatorResult { input_string: result_output };
            //         },
            //         _ => todo!()
            //     }
            // }

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
        match self.state {
            State::Downloading { .. } => {
                download::file(self.id, "https://speed.hetzner.de/100MB.bin?")
                    .map(Message::DownloadProgressed)
            },
            State::CalculatorEvaluating { .. } => {
                todo!();
                // download::download_start_calculating(0, "4 + 4")
                //     .map(Message::DoneCalculating)
            }
            _ => Subscription::none(),
        }
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

        let control2: Element<_> = match &self.state {
            State::Idle => button("Start Calculation!")
            .style(theme::Button::Primary)
            .on_press(Message::StartCalculating)
            .into(),
            State::Downloading { progress } => todo!(),
            State::Finished => todo!(),
            State::Errored => todo!(),
            State::CalculatorEvaluating { done } => todo!(),
            State::CalculatorResult { input_string } => todo!(),

        };
        let control: Element<_> = match &self.state {
            State::Idle => button("Start the download!")
                .on_press(Message::Download(self.id))
                .into(),
            State::Finished => {
                column!["Download finished!", button("Start again")]
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .into()
            }
            State::Downloading { .. } => {
                text(format!("Downloading... {:.2}%", current_progress)).into()
            }
            State::Errored => column![
                "Something went wrong :(",
                button("Try again").on_press(Message::Download(self.id)),
            ]
            .spacing(10)
            .align_items(Alignment::Center)
            .into(),
            State::CalculatorEvaluating { done } => todo!(),
            State::CalculatorResult { input_string } => todo!(),
        };

        Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Alignment::Center)
            .push(progress_bar)
            .push(control)
            .push(control2)
            .into()
    }
}
