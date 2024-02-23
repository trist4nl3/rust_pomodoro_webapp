use std::rc::Rc;
use web_sys::HtmlInputElement;
use gloo::timers::callback::{Interval, Timeout};
use yew::prelude::*;
use crate::components::inputfield::FieldInput;


use std::cell::RefCell;

pub fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}

pub enum TimerAction {
    Add(&'static str),
    Cancel,
    SetInterval(Rc<Interval>),
    SetTimeout(Rc<Timeout>),
    TimeoutDone,
    UpdateCountdown,
    SetTime(u32),
    SetCountdown(u32),
    Pause,
}

#[derive(Clone, Debug)]
pub struct TimerState {
    messages: Vec<&'static str>,
    interval_handle: Option<Rc<Interval>>,
    timeout_handle: Option<Rc<Timeout>>,
    time_remaining: u32,
    time_amount: u32,
    on_break: bool,
    time_default: u32,
}

impl PartialEq for TimerState {
    fn eq(&self, other: &Self) -> bool {
        self.messages == other.messages
            && self.interval_handle.is_some() == other.interval_handle.is_some()
    }
}

impl TimerState {
    fn new() -> Self {
        TimerState {
            messages: Vec::new(),
            interval_handle: None,
            timeout_handle: None,
            time_remaining: 25 * 60,
            time_amount: 25,
            on_break: false,
            time_default: 25,
        }
    }
}

impl Reducible for TimerState {
    type Action = TimerAction;

    fn reduce(self: Rc<Self>, action: TimerAction) -> Rc<Self> {
        match action {
            TimerAction::Add(message) => {
                let mut messages = self.messages.clone();
                messages.push(message);
                Rc::new(TimerState {
                    messages,
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: self.timeout_handle.clone(),
                    time_remaining: self.time_remaining,
                    time_amount: self.time_amount,
                    on_break: self.on_break,
                    time_default: self.time_default,
                })
            }
            TimerAction::SetInterval(t) => Rc::new(TimerState {
                messages: vec!["Interval started!"],
                interval_handle: Some(t),
                timeout_handle: self.timeout_handle.clone(),
                time_remaining: self.time_remaining,
                time_amount: self.time_amount,
                on_break: self.on_break,
                time_default: self.time_default,
            }),
            TimerAction::SetTimeout(t) => Rc::new(TimerState {
                messages: vec!["Timer started!!"],
                interval_handle: self.interval_handle.clone(),
                timeout_handle: Some(t),
                time_remaining: self.time_remaining,
                time_amount: self.time_amount,
                on_break: self.on_break,
                time_default: self.time_default,
            }),
            TimerAction::TimeoutDone => {
                let mut messages = self.messages.clone();
                messages.push("Done!");
                Rc::new(TimerState {
                    messages,
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: None,
                    time_remaining: self.time_amount * 60,
                    time_amount: self.time_amount,
                    on_break: self.on_break,
                    time_default: self.time_default,
                })
            }
            TimerAction::Cancel => {
                let mut messages = self.messages.clone();
                messages.push("Canceled!");
                Rc::new(TimerState {
                    messages,
                    interval_handle: None,
                    timeout_handle: None,
                    time_remaining: self.time_default * 60,
                    time_amount: self.time_amount,
                    on_break: self.on_break,
                    time_default: self.time_default,
                })
            }
            TimerAction::UpdateCountdown => {
                if self.time_remaining > 0 {
                    Rc::new(TimerState {
                        messages: self.messages.clone(),
                        interval_handle: self.interval_handle.clone(),
                        timeout_handle: self.timeout_handle.clone(),
                        time_remaining: self.time_remaining - 1,
                        time_amount: self.time_amount,
                        on_break: self.on_break,
                        time_default: self.time_default,
                    })
                } else {
                    self.clone()
                }
            }
            TimerAction::SetTime(time) => {
                Rc::new(TimerState {
                    messages: self.messages.clone(),
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: self.timeout_handle.clone(),
                    time_remaining: time * 60,
                    time_amount: time,
                    on_break: self.on_break,
                    time_default: time,
                })
            }
            TimerAction::SetCountdown(time) => {
                Rc::new(TimerState {
                    messages: self.messages.clone(),
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: self.timeout_handle.clone(),
                    time_remaining: time,
                    time_amount: self.time_amount,
                    on_break: self.on_break,
                    time_default: self.time_default,
                })
            }
            TimerAction::Pause => {
                let mut messages = self.messages.clone();
                messages.push("Paused!");
                Rc::new(TimerState {
                    messages,
                    interval_handle: None,
                    timeout_handle: None,
                    time_remaining: self.time_remaining,
                    time_amount: self.time_amount,
                    on_break: self.on_break,
                    time_default: self.time_default,
                })
            }
        }
    }
}

#[function_component(Clock)]
pub fn clock() -> Html {
    let time = use_state(get_current_time);
    
    {
        let time = time.clone();
        use_effect_with((), |_| {
            Interval::new(1000, move || time.set(get_current_time())).forget();
        });
    }
    html!(
        <div id="time">{ time.as_str() }</div>
    )
}

#[function_component]
pub fn Pomodoro() -> Html {
    let state = use_reducer(TimerState::new);

    let time_ref = use_node_ref();

    let mut key = 0;
    let messages: Html = state
        .messages
        .iter()
        .map(|message| {
            key += 1;
            html! { <p key={ key }>{ *message }</p> }
        })
        .collect(); 
    let minutes = state.clone().time_remaining / 60;
    let seconds = state.clone().time_remaining % 60;
    let time_str = format!("{:02}:{:02}", minutes, seconds);
    
    let _minutes_2 = state.clone().time_amount / 60;
    let _seconds_2 = state.clone().time_amount % 60;
    let _time_str_2 = format!("{:02}:{:02}", minutes, seconds);
    
    let display_countdown: Html = if state.clone().time_remaining > 0 {
        // Display time in 00:00 format
        html! { <div id="time"> {  time_str } </div> }
    } else {
        html! { <div id="time"> {  _time_str_2 } </div> }
    };
   
    let has_job = state.timeout_handle.is_some();

    let setting_pressed = Rc::new(RefCell::new(false));
    let setting_pressed_clone = setting_pressed.clone();

    let on_settings = {
        Callback::from(move |_: MouseEvent| {
            let mut setting_pressed_ref = setting_pressed_clone.borrow_mut();
            if *setting_pressed_ref || has_job {
                let user_input = web_sys::window().unwrap().document().unwrap().get_element_by_id("user_input").unwrap();
                user_input.set_attribute("style", "display: none;").unwrap();
                *setting_pressed_ref = false;
            } else {
                let user_input = web_sys::window().unwrap().document().unwrap().get_element_by_id("user_input").unwrap();
                user_input.set_attribute("style", "display: block;").unwrap();
                *setting_pressed_ref = true;
            }
        })
    };

    let on_exit_settings = {
        Callback::from(move |_: MouseEvent| {
            let mut setting_pressed_ref = setting_pressed.borrow_mut();
            let user_input = web_sys::window().unwrap().document().unwrap().get_element_by_id("user_input").unwrap();
            user_input.set_attribute("style", "display: none;").unwrap();
            *setting_pressed_ref = false;
        })
    };

    let on_add_timeout = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let timeout_state = state.clone();
            let message_state = state.clone();
            let ping_state = state.clone();
            let interval_state = state.clone();
            // Setting time
            let time = state.clone().time_remaining;
            let time_state = state.clone();
            time_state.dispatch(TimerAction::SetCountdown(time));

            let t = Rc::new(Timeout::new(time * 1000, move || {
                message_state.dispatch(TimerAction::TimeoutDone);
            }));
            let i = Rc::new(Interval::new(1000, move || {
                ping_state.dispatch(TimerAction::UpdateCountdown);
            }));

            interval_state.dispatch(TimerAction::SetInterval(i));
            timeout_state.dispatch(TimerAction::SetTimeout(t));
        })
    };

    let onsubmit = {
        let state = state.clone();
        let time_ref = time_ref.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let time = time_ref.cast::<HtmlInputElement>().unwrap().value().parse().unwrap();
            state.dispatch(TimerAction::SetTime(time));
        })
    };
    
    let time_amount = state.clone().time_amount.to_string();

    let timer_start = {
        let on_add_timeout = on_add_timeout.clone();
        let on_exit_settings = on_exit_settings.clone();
        Callback::from(move |event: MouseEvent| {
            on_add_timeout.emit(event.clone());
            on_exit_settings.emit(event.clone());
        })
    };

    let on_pause = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(TimerAction::Pause);
        })
    };

    let on_cancel = {
        Callback::from(move |_: MouseEvent| {
            state.dispatch(TimerAction::Cancel);
        })
    };

    html!(
        <>
        <div id="background">
        <div id="content">
            <div id="title-area">
                <h1 class="title">{ "Pomodoro Timer" }</h1>
                <h2 class="subtitle">{ "By Neblume"}</h2>
            </div>

            <div id="timer_wrapper">
                <div id="time_remaining">{ display_countdown }</div>
            </div>
            <div id="buttons">
                <button class="button" disabled={has_job} onclick={timer_start}>{ "Start" }</button>
                <button class="button" disabled={!has_job} onclick={on_pause}>{ "Pause" }</button>
                <button class="button" disabled={!has_job} onclick={on_cancel}><i class="fas fa-redo-alt"></i></button>
                <div id="settings-button">
                    <button class="button" disabled={has_job} onclick={on_settings}><i class="fas fa-cog"></i></button>
                    <div id="settings-menu">
                        <div id="user_input" class="settings" style="display: none;">
                            <button class="exit-button" onclick={on_exit_settings}><i class="fas fa-times"></i></button>
                            // setting time
                            <form {onsubmit}>
                                <FieldInput
                                    label="Minutes:"
                                    input_type="number"
                                    name=""
                                    placeholder={time_amount}
                                    node_ref={time_ref}
                                />
                                <button class="button" type="submit">{"Save"}</button>
                            </form>
                        </div>
                        // Other settings content here
                    </div>
                </div>
            </div>
        </div>
    </div>
    
        </>
    )
}
