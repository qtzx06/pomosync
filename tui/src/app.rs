use chrono::{DateTime, Local, Timelike};
use std::time::Duration;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Work,
    ShortBreak,
    LongBreak,
}

impl State {
    fn next_state_name(&self) -> &'static str {
        match self {
            State::Work => "Short Break",
            State::ShortBreak => "Work",
            State::LongBreak => "Work",
        }
    }
}

pub struct Cycle {
    pub state: State,
    pub state_name: String,
    pub next_state_name: String,
    pub remaining_duration: Duration,
    pub paused: bool,
    pub show_menu: bool,
}

impl Cycle {
    pub fn new() -> Self {
        let mut cycle = Self {
            state: State::Work,
            state_name: String::new(),
            next_state_name: String::new(),
            remaining_duration: Duration::from_secs(0),
            paused: false,
            show_menu: false,
        };
        cycle.update();
        cycle
    }

    pub fn update(&mut self) {
        if self.paused {
            return;
        }

        let now: DateTime<Local> = Local::now();
        let minutes_into_2h_cycle = (now.minute() + (now.hour() % 2) * 60) as u64;
        let seconds_into_minute = now.second() as u64;
        let total_seconds_into_cycle = minutes_into_2h_cycle * 60 + seconds_into_minute;

        let (new_state, state_name, state_start_minute, state_duration_minutes) =
            match minutes_into_2h_cycle {
                0..=24 => (State::Work, "Work", 0, 25),
                25..=29 => (State::ShortBreak, "Short Break", 25, 5),
                30..=54 => (State::Work, "Work", 30, 25),
                55..=59 => (State::ShortBreak, "Short Break", 55, 5),
                60..=84 => (State::Work, "Work", 60, 25),
                85..=89 => (State::ShortBreak, "Short Break", 85, 5),
                90..=119 => (State::LongBreak, "Long Break", 90, 30),
                _ => unreachable!(),
            };

        let state_total_seconds = state_duration_minutes * 60;
        let elapsed_seconds_in_state = total_seconds_into_cycle - (state_start_minute * 60);
        let remaining_seconds = state_total_seconds - elapsed_seconds_in_state;

        self.state = new_state;
        self.state_name = state_name.to_string();
        self.next_state_name = self.state.next_state_name().to_string();
        self.remaining_duration = Duration::from_secs(remaining_seconds);
    }

    pub fn remaining_time(&self) -> String {
        let secs = self.remaining_duration.as_secs();
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_menu(&mut self) {
        self.show_menu = !self.show_menu;
    }
}