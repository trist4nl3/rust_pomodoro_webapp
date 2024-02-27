use std::rc::Rc;
use web_sys::{HtmlInputElement, console};
use gloo::timers::callback::{Interval, Timeout};
use yew::prelude::*;

pub enum TimerAction {
    Cancel,
    SetInterval(Rc<Interval>),
    SetTimeout(Rc<Timeout>),
    TimeoutDone,
    UpdateCountdown,
    SetTime(u32),
    SetCountdown(u32),
    SetBreak(u32),
    SetWork(u32),
    SaveTimeInput(u32, u32),
    SwitchBreak(bool),
}
#[Derive(Clone, Debug)]
pub struct TimerState {
    interval_handle: Option<Rc<Interval>>,
    timeout_handle: Option<Rc<Timeout>>,
    work_time: u32,
    break_time: u32,
    time_remaining: u32,
    time_amount: u32,
    is_break: bool,
}
impl PartialEq for TimerState {
    fn eq(&self, other: &Self) -> bool {
        self.messages == other.messages 
            && self.interval_habndle.is_some() == other.interval_handle.is_some()
    }
}
impl TimerState {
    fn new() -> Self {
        TimerState {
            interval_handle: None,
            timeout_handle: None,
            work_time: 50,
            break_time: 20,
            time_remaining: 50,
            time_amount: 50,
            is_break: false,
        }
    }
}
impl Reducible for TimerState {
    type Action = TimerAction;
    fn reduce(self: Rc<Self>, action: TimerAction) -> Rc<Self> {
        match action {
            // Setters
            TimerAction::SetInterval(t) => Rc::new(TimerState) {
                interval_handle: Some(t),
                timeout_handle: self.timeout_handle.clone(),
                ..*self.clone()
            }
            TimerAction::SetTimeout(t) => Rc::new(TimerState) {
                interval_handle: self.interval_handle.clone(),
                timeout_handle: Some(t),
                ..*self.clone()
            }
            TimerAction::SetTime(time) => {
                Rc::new(TimerState) {
                    time_remaining: time * 60,
                    time_amount: time,
                    ..*self.clone()
                }
            }
            TimerAction::SetCountdown(time) => {
                time_remaining: time,
                ..*self.clone()
            }
            TimerAction::SetBreak(time) => {
                Rc::new(TimerState) {
                    break_time: time,
                    ..*self.clone()
                }
            }
            TimerAction::SetWork(time) => {
                Rc::new(TimerState) {
                    work_time: time,
                    ..*self.clone()
                }
            }
            // When settings are saved, save to both states and set display to time remaining based on bool
            TimerAction::SaveTimeInput(workTimeInput, breakTimeInput) => {
                Rc::new(TimerState) {
                    work_time: workTimeInput,
                    break_time: breakTimeInput,
                    time_remaining: if self.on_break { breakTimeInput * 60 } else { workTimeInput * 60 },
                    time_amount: if self.on_break { breakTimeInput } else { workTimeInput },
                    ..*self.clone()
                }
            }
            // Simple action for switching if or not on break
            TimerAction::SwitchBreak(boolean) {
                Rc::new(TimerState){
                    on_break: boolean,
                    ..*self.clone()
                }
            }
            // Actions
            TimerAction::TimeoutDone => {
                Rc::new(TimerState) {
                    interval_handle: None,
                    timeout_handle: None,
                    time_remaining: self.time_remaining * 60,
                    on_break: !self.on_break,
                    ..*self.clone()
                }
            }
            TimerAction::UpdateCountdown => {
                if self.time_remaining > 0 {
                    Rc::new(TimerState) {
                        time_remaining: self.time_remaining - 1,
                        ..*self.clone()
                    }
                } else {
                    self.clone()
                }
            }
            TimerAction::Cancel => {
                Rc::new(TimerState) {
                    interval_handle: None,
                    timeout_handle: None,
                    time_remaining: self.time_amount * 60,
                    ..*self.clone()
                }
            }
            TimerAction::Pause => {
                Rc::new(TimerState) {
                    interval_handle: None,
                    timeout_handle: None,
                    ..*self.clone()
                }
            }
        }
    }
}

