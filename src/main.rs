use iced::Settings;
use iced::{
    executor, Application, Command, Theme, Element,
    widget::{column, text, text_input, TextInput, button},
};

fn main() -> iced::Result{
    TaskCrab::run(Settings::default())
}

pub struct TaskCrab{
    input: String,
    tasks: Vec<String>,
}

impl Application for TaskCrab{
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            TaskCrab {
                input: String::new(),
                tasks: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("TaskCrab go Brrrr")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::InputChanged(new_input) => {
                self.input = new_input;
            }
            Message::Submit => {
                self.tasks.push(self.input.clone());
            }
            Message::Delete(i) => {
                if i < self.tasks.len() {
                    self.tasks.remove(i);
                }
            }
        }
        Command::none()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let input = TextInput::new("Enter task", &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::Submit);

        let tasks = self.tasks.iter().enumerate()
        .map(|(i, task)| { button(text(task)).on_press(Message::Delete(i)).into()}).collect();

        column![
            input,
            text("Tasks:"),
            column(tasks)
                .spacing(5)
                .padding(5)
        ]
        .padding(20)
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Submit,
    Delete(usize),
    Trial,
}