use chrono::{Datelike, Local};
/// TaskCrab
/// A simple task manager application built with Iced and Rust.
/// It allows users to add, delete, and view tasks with different priorities and due dates.
/// The application uses JSON files to store tasks and provides a simple UI for task management.
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
    day: String,
    month: String,
    year: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: u64,
    name: String,
    priority: u8,
    due_date: (u8, u8, u16),
}

impl Application for TaskCrab {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    //-------------Application Functions--------------//

    fn title(&self) -> String {
        String::from("TaskCrab")
    }
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            TaskCrab {
                input: String::new(),
                tasks: load_tasks_from_file().unwrap_or_else(|_| Vec::new()),
                priority: 3,
                day: String::new(),
                month: String::new(),
                year: String::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            //update input fields
            Message::InputChanged(new_input) => {
                self.input = new_input;
            }
            Message::PrioritySelected(priority) => {
                self.priority = priority;
            }
            Message::DayChanged(day) => {
                self.day = day;
            }
            Message::MonthChanged(month) => {
                self.month = month;
            }
            Message::YearChanged(year) => {
                self.year = year;
            }

            //submit task
            Message::Submit => {
                if self.input.is_empty() {
                    return Command::none();
                }
                let _ = clear_tasks_file();
                self.tasks.push(Task {
                    id: self.tasks.len() as u64,
                    name: self.input.clone(),
                    priority: self.priority,
                    due_date: (
                        self.month.parse().unwrap_or(0),
                        self.day.parse().unwrap_or(0),
                        self.year.parse().unwrap_or(0),
                    ),
                });
                sort_tasks(&mut self.tasks);
                let _ = save_tasks_to_file(&self.tasks);
                self.input.clear();
                self.day.clear();
                self.month.clear();
                self.year.clear();
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

    //-------------View Function--------------//

    fn view(&self) -> iced::Element<Self::Message> {
        //bottom row with input, priority, date, and clear button
        let bottom_row: Row<'_, Message> = Row::new()
            .spacing(5)
            .align_items(Alignment::Center)
            .push(
                TextInput::new("MM", &self.month)
                    .on_input(Message::MonthChanged)
                    .on_submit(Message::Submit)
                    .width(Length::Fixed(40.0)),
            )
            .push(
                TextInput::new("DD", &self.day)
                    .on_input(Message::DayChanged)
                    .on_submit(Message::Submit)
                    .width(Length::Fixed(40.0)),
            )
            .push(
                TextInput::new("YYYY", &self.year)
                    .on_input(Message::YearChanged)
                    .on_submit(Message::Submit)
                    .width(Length::Fixed(60.0)),
            )
            .push(
                TextInput::new("Enter tasks to pinch away", &self.input)
                    .on_input(Message::InputChanged)
                    .on_submit(Message::Submit),
            )
            .push(
                priority_selector(self.priority)
                    .align_items(Alignment::Center)
                    .height(30),
            )
            .push(
                button(text("Clear").size(22))
                    .on_press(Message::Clear)
                    .style(iced::theme::Button::Text),
            );

        //task rows with name, priority, and date
        let tasks = self
            .tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let task_button = button(text(task.name.clone()).size(25))
                    .padding(5)
                    .style(iced::theme::Button::Text)
                    .on_press(Message::Delete(i))
                    .height(Length::Fixed(40.0))
                    .height(Length::Fixed(40.0));

                //color based on priority
                let color = match task.priority {
                    1 => iced::Color::from_rgb(0.2, 0.8, 0.2),
                    2 => iced::Color::from_rgb(0.5, 0.8, 0.2),
                    3 => iced::Color::from_rgb(1.0, 0.8, 0.0),
                    4 => iced::Color::from_rgb(1.0, 0.5, 0.0),
                    _ => iced::Color::from_rgb(1.0, 0.2, 0.2),
                };

                let priority_indicator = Container::new(Text::new(format!("●")))
                    .style(iced::theme::Container::Custom(Box::new(
                        move |_: &Theme| container::Appearance {
                            text_color: Some(color),
                            background: None,
                            ..Default::default()
                        },
                    )));

                let due_date: Text<'_> = Text::new(match task.due_date {
                    (0, 0, 0) => format!(""),
                    (month, 0, 0) => format!("{}", month),
                    (0, day, 0) => format!("{}", day),
                    (month, day, 0) => format!("{}/{}", month, day),
                    (month, 0, year) if year != 0 => format!("{}/{}", month, year),
                    (month, day, year) if year == 0 => format!("{}/{}", month, day),
                    (month, day, year) => format!("{}/{}/{}", month, day, year),
                })
                .height(Length::Fixed(40.0));

                Row::new()
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .height(Length::Fixed(40.0))
                    .push(
                        Container::new(priority_indicator)
                            .width(Length::Fixed(30.0))
                            .center_y(),
                    )
                    .push(
                        Container::new(task_button)
                            .width(Length::FillPortion(3)) // Task name takes most space
                            .center_y(),
                    )
                    .push(
                        Container::new(due_date)
                            .width(Length::Fixed(80.0)) // Fixed-width due date
                            .center_y()
                            .align_x(iced::alignment::Horizontal::Right),
                    )
                    .into()
            })
            .collect();

        //main container with input, priority, and tasks rows
        column![
            scrollable(container(column(tasks).spacing(5).padding(5)))
                .width(Length::Fill)
                .height(Length::FillPortion(1)),
            container(bottom_row)
                .align_x(iced::alignment::Horizontal::Left)
                .width(Length::Fill)
                .height(Length::Fixed(50.0)),
        ]
        .padding(20)
        .spacing(10)
        .into()
    }
}

//-------------Message Enum--------------//

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    PrioritySelected(u8),
    DayChanged(String),
    MonthChanged(String),
    YearChanged(String),
    Submit,
    Delete(usize),
    Clear,
}

//-------------UI Functions--------------//

fn priority_selector(selected: u8) -> Row<'static, Message> {
    let mut row = Row::new().spacing(5);

    for i in 1..=5 {
        let symbol = if i <= selected { "●" } else { "○" };
        let button = Button::new(Text::new(symbol))
            .on_press(Message::PrioritySelected(i))
            .padding(0)
            .style(if i <= selected {
                iced::theme::Button::Text
            } else {
                iced::theme::Button::Text
            });

        row = row.push(button);
    }
    row
}

//-------------File Functions--------------//

fn load_tasks_from_file() -> Result<Vec<Task>, std::io::Error> {
    let file = std::fs::File::open("tasks.json")?;
    let mut tasks: Vec<Task> = serde_json::from_reader(file)?;
    sort_tasks(&mut tasks);
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

//-------------Date Functions--------------//

fn _get_current_date() -> (u8, u8, u16) {
    let now = Local::now();
    (now.day() as u8, now.month() as u8, now.year() as u16)
}

//-------------Display Functions--------------//

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

//-------------Organization Functions--------------//
//implement task organization/sorting

fn sort_tasks(tasks: &mut Vec<Task>) {
    tasks.sort_by(|a, b| {
        //let a_date = (a.due_date.0, a.due_date.1, a.due_date.2);
        //let b_date = (b.due_date.0, b.due_date.1, b.due_date.2);
        let a_priority = a.priority;
        let b_priority = b.priority;
        b_priority.cmp(&a_priority)
    });
}

//-------------Brainstorm for future--------------//
// - Add a feature to set reminders for tasks.
// - Description field for tasks??
// - Add a feature to categorize tasks (e.g., work, personal, etc.).
// - task history
// - Completion graphic
// - haha todo list in todo list