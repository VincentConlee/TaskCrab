use iced::Settings;
use iced::{
    executor,
    widget::{
        button, column, container, scrollable, text, Button, Container, Row, Text, TextInput,
    },
    Alignment, Application, Command, Length, Theme,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

fn main() -> iced::Result {
    TaskCrab::run(Settings::default())
}

pub struct TaskCrab {
    input: String,
    tasks: Vec<Task>,
    priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: u64,
    name: String,
    priority: u8,
    _due_date: (u8, u8, u16),
}

impl Application for TaskCrab {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            TaskCrab {
                input: String::new(),
                tasks: load_tasks_from_file().unwrap_or_else(|_| Vec::new()),
                priority: 3,
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
            Message::PrioritySelected(priority) => {
                self.priority = priority;
            }
            Message::Submit => {
                if self.input.is_empty() {
                    return Command::none();
                }
                let _ = clear_tasks_file();
                self.tasks.push(Task {
                    id: self.tasks.len() as u64,
                    name: self.input.clone(),
                    priority: self.priority,
                    _due_date: (0, 0, 0),
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

        let priority = priority_selector(self.priority);

        let tasks = self
            .tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let task_button = button(text(task.name.clone()).size(18))
                    .padding(5)
                    .style(iced::theme::Button::Text)
                    .on_press(Message::Delete(i));

                //color based on priority
                let color = match task.priority {
                    1 => iced::Color::from_rgb(0.2, 0.8, 0.2),
                    2 => iced::Color::from_rgb(0.5, 0.8, 0.2),
                    3 => iced::Color::from_rgb(1.0, 0.8, 0.0),
                    4 => iced::Color::from_rgb(1.0, 0.5, 0.0),
                    _ => iced::Color::from_rgb(1.0, 0.2, 0.2),
                };

                let priority_indicator = Container::new(Text::new(format!("●")))
                    .width(30)
                    .height(30)
                    .style(iced::theme::Container::Custom(Box::new(
                        move |_: &Theme| container::Appearance {
                            text_color: Some(color),
                            background: None,
                            ..Default::default()
                        },
                    )))
                    .align_y(iced::alignment::Vertical::Bottom)
                    .padding(1.5)
                    .align_x(iced::alignment::Horizontal::Left);

                Row::new()
                    .spacing(10)
                    .push(task_button)
                    .push(priority_indicator)
                    .into()
            })
            .collect();

        //implement task completion graphic
        //implement task due date
        //implement task organization/sorting
        //make perty
        //add to path so it can be run from anywhere
        //haha todo list in todo list

        column![
            input,
            priority,
            text("Tasks:")
                .horizontal_alignment(iced::alignment::Horizontal::Left)
                .size(30),
            scrollable(
                container(column(tasks).spacing(5).padding(5))
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .align_x(iced::alignment::Horizontal::Left)
            ),
            button(text("Clear Tasks").size(20))
                .on_press(Message::Clear)
                .padding(10)
                .style(iced::theme::Button::Text),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

fn priority_selector(selected: u8) -> Row<'static, Message> {
    let mut row = Row::new().spacing(5);

    for i in 1..=5 {
        let symbol = if i <= selected { "●" } else { "○" };
        let button = Button::new(Text::new(symbol))
            .on_press(Message::PrioritySelected(i))
            .padding(0)
            .style(if i <= selected {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            });

        row = row.push(button);
    }
    row
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    PrioritySelected(u8),
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
