use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = format!("{}{}", self.content, "\x1b[0m"); // Reset color to default after printing /x1b[0m is the escape sequence for reseting the color
        let colored_content = content.truecolor(self.color.0, self.color.1, self.color.2);
        // truecolor is a method from the colored crate that takes 3 arguments: red, green and blue
        let formatted_notification = match self.position {
            Position::Top => format!("(Top, {}, {})", self.size, colored_content),
            Position::Bottom => format!("(Bottom, {}, {})", self.size, colored_content),
            Position::Center => format!("(Center, {}, {})", self.size, colored_content),
        };
        
        write!(f, "{}", formatted_notification)
    }
}

impl Event<'_> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Event::Registration(duration) => {
                let remaining_time = duration.num_seconds();
                let hours = remaining_time / 3600;
                let minutes = (remaining_time % 3600) / 60;
                let seconds = remaining_time % 60;
                let formatted_duration = format!("You have {}H:{}M:{}S left before the registration ends", hours, minutes, seconds);

                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: formatted_duration,
                }
            }
            Event::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}