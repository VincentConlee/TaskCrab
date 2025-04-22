use iced::Settings;
use iced::{
    executor, Application, Command, Theme, Length,
    widget::{column, text, TextInput, button, container, scrollable},
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

fn main() -> iced::Result{
    TaskCrab::run(Settings::default())
}

pub struct TaskCrab{
    input: String,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task{
    id: u64,
    name: String,
    _description: String,
    _time: u64,
    _priority: u8,
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
                tasks: load_tasks_from_file().unwrap_or_else(|_| Vec::new()),
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
                let _ = clear_tasks_file();
                self.tasks.push(Task{
                    id: self.tasks.len() as u64,
                    name: self.input.clone(),
                    _description: String::new(),
                    _time: 0,
                    _priority: 0,
                });
                let _ = save_tasks_to_file(&self.tasks);
                self.input.clear();
            }
            Message::Delete(i) => {
                if i < self.tasks.len() {
                    self.tasks.remove(i);
                }
                let _ = clear_tasks_file();
                let _ = save_tasks_to_file(&self.tasks);
            }
            Message::Clear => {
                self.tasks.clear();
                let _ = clear_tasks_file();
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
        .map(|(i, task)| { 
            button(text(task.name.clone()).size(18)).padding(5).style(iced::theme::Button::Text)
            .on_press(Message::Delete(i)).into()}).collect();

        //implement highligh after task creation
        //implement wait (so tasks don't get spammed)
        //implement task completion graphic
        //implement task parameters
        //implement task organization/sorting
        //add to path so it can be run from anywhere
        //haha todo list in todo list
        //make perty

        column![
            input,
            text("Tasks:").horizontal_alignment(iced::alignment::Horizontal::Center).size(30),
            scrollable(
            container(
                column(tasks)
                .spacing(5)
                .padding(5)
            ).width(Length::Fill).height(Length::Shrink)),
            button(text("Clear Tasks").size(20))
                .on_press(Message::Clear)
                .padding(5)
                .style(iced::theme::Button::Text),
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
    Clear,
}

fn load_tasks_from_file() -> Result<Vec<Task>, std::io::Error> {
    let file = std::fs::File::open("tasks.json")?;
    let tasks: Vec<Task> = serde_json::from_reader(file)?;
    Ok(tasks)
}

fn save_tasks_to_file(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    let file = std::fs::File::create("tasks.json")?;
    serde_json::to_writer_pretty(file, tasks)?;
    Ok(())
}

fn clear_tasks_file() -> Result<(), std::io::Error> {
    std::fs::write("tasks.json", "")?;
    Ok(())
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}