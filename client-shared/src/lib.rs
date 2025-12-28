use iced::widget::{button, column, container, row, space, text, text_input};
use iced::{Alignment, Element, Length, Task, Theme};

use shared::Data;

#[derive(Clone)]
pub enum Message {
    ChangeA(String),
    ChangeB(String),
    RequestSum,
    UpdateResult(i32),
    ResponseError(String),
}

pub struct App {
    data: Data,
    result: i32,
}

impl App {
    pub fn new() -> App {
        let data = Data { a: 0, b: 0 };
        App { data, result: 0 }
    }

    pub fn title(&self) -> String {
        "Fullstack Rust with iced and axum".to_owned()
    }

    pub fn theme(&self) -> Theme {
        Theme::Ferra
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeA(input) => {
                if let Ok(value) = input.parse() {
                    self.data.a = value;
                }
            }
            Message::ChangeB(input) => {
                if let Ok(value) = input.parse() {
                    self.data.b = value;
                }
            }
            Message::RequestSum => {
                let data = serde_json::to_vec(&self.data).expect("Should encode the data");
                return Task::perform(request_sum(data), identity);
            }
            Message::UpdateResult(value) => {
                self.result = value;
            }
            Message::ResponseError(_e) => {
                // handle error
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let contents = column![
            row![
                text("a"),
                space::horizontal().width(Length::Fixed(20.0)),
                text_input("value a", &self.data.a.to_string())
                    .on_input(Message::ChangeA)
                    .width(Length::Fixed(80.0)),
            ]
            .align_y(Alignment::Center),
            row![
                text("b"),
                space::horizontal().width(Length::Fixed(20.0)),
                text_input("value b", &self.data.b.to_string())
                    .on_input(Message::ChangeB)
                    .width(Length::Fixed(80.0)),
            ]
            .align_y(Alignment::Center),
            row![button("Request sum").on_press(Message::RequestSum)].align_y(Alignment::Center),
            text(format!("a + b = {}", self.result)),
        ]
        .width(Length::Fixed(200.0))
        .spacing(10)
        .align_x(Alignment::Center);
        container(contents).center(Length::Fill).into()
    }
}

fn identity(message: Message) -> Message {
    message
}

async fn request_sum(data: Vec<u8>) -> Message {
    let client = reqwest::Client::new();
    match client
        .post("http://127.0.0.1:8080/sum")
        .body(data)
        .header("content-type", "application/json")
        .send()
        .await
    {
        Ok(response) => {
            if response.status() != 200 {
                return Message::ResponseError(format!("Wrong status: {}", response.status()));
            }
            match response.json().await {
                Ok(result) => {
                    println!("Got response: {result}");
                    Message::UpdateResult(result)
                }
                Err(e) => Message::ResponseError(format!("Error decoding response: {e}")),
            }
        }
        Err(e) => Message::ResponseError(format!("Network error: {e}")),
    }
}
