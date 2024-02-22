use std::rc::Rc;
use web_sys::HtmlInputElement;
use gloo::timers::callback::{Interval, Timeout};
use yew::prelude::*;
use wasm_bindgen::JsValue;
use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::components::inputfield::FieldInput;


pub fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}

pub enum TimerAction {
    Add(&'static str),
    Cancel,
    SetInterval(Interval),
    SetTimeout(Timeout),
    TimeoutDone,
    UpdateCountdown,
    SetTime(u32),
    Pause,
}

#[derive(Clone, Debug)]
pub struct TimerState {
    messages: Vec<&'static str>,
    interval_handle: Option<Rc<Interval>>,
    timeout_handle: Option<Rc<Timeout>>,
    time_remaining: u32,
}

impl PartialEq for TimerState {
    fn eq(&self, other: &Self) -> bool {
        self.messages == other.messages
            && self.interval_handle.is_some() == other.interval_handle.is_some()
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
                })
            }
            TimerAction::SetInterval(t) => Rc::new(TimerState {
                messages: vec!["Interval started!"],
                interval_handle: Some(Rc::from(t)),
                timeout_handle: self.timeout_handle.clone(),
                time_remaining: self.time_remaining,
            }),
            TimerAction::SetTimeout(t) => Rc::new(TimerState {
                messages: vec!["Timer started!!"],
                interval_handle: self.interval_handle.clone(),
                timeout_handle: Some(Rc::from(t)),
                time_remaining: self.time_remaining,
            }),
            TimerAction::TimeoutDone => {
                let mut messages = self.messages.clone();
                messages.push("Done!");
                Rc::new(TimerState {
                    messages,
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: None,
                    time_remaining: self.time_remaining,
                })
            }
            TimerAction::Cancel => {
                let mut messages = self.messages.clone();
                messages.push("Canceled!");
                Rc::new(TimerState {
                    messages,
                    interval_handle: None,
                    timeout_handle: None,
                    time_remaining: 0,
                })
            }
            TimerAction::UpdateCountdown => {
                let mut time_remaining = self.time_remaining;
                if time_remaining > 0 {
                    time_remaining -= 1;
                }
                Rc::new(TimerState {
                    messages: self.messages.clone(),
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: self.timeout_handle.clone(),
                    time_remaining: self.time_remaining - 1,
                })
            }
            TimerAction::SetTime(time) => {
                Rc::new(TimerState {
                    messages: self.messages.clone(),
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: self.timeout_handle.clone(),
                    time_remaining: time,
                })
            }
            TimerAction::Pause => {
                let mut messages = self.messages.clone();
                messages.push("Paused!");
                Rc::new(TimerState {
                    messages,
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: self.timeout_handle.clone(),
                    time_remaining: self.time_remaining,
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
    let state = use_reducer(|| TimerState {
        messages: Vec::new(),
        interval_handle: None,
        timeout_handle: None,
        time_remaining: 0,
    });

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
    
    let display_countdown: Html = if state.clone().time_remaining > 0 {
        state.time_remaining.to_string().into()
    } else {
        html! {}
    };

    let has_job = state.interval_handle.is_some() || state.timeout_handle.is_some();

    let on_add_timeout = {
        let state = state.clone();

        Callback::from(move |_: MouseEvent| {
            let timeout_state = state.clone();
            let message_state = state.clone();
            let ping_state = state.clone();
            let interval_state = state.clone();
            // Setting time
            let time = state.clone().time_remaining;
            state.dispatch(TimerAction::SetTime(time));


            let t = Timeout::new(time * 1000, move || {
                message_state.dispatch(TimerAction::TimeoutDone);
            });
            let i = Interval::new(1000, move || {
                ping_state.dispatch(TimerAction::UpdateCountdown);
            });


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
            state.dispatch(TimerAction::SetTime(time))
        })

    };
    

    let on_cancel = {
        Callback::from(move |_: MouseEvent| {
            state.dispatch(TimerAction::Cancel);
        })
    };
    // let on_pause = {
    //     Callback::from(move |_: MouseEvent| {
    //         state.dispatch(TimerAction::Pause);
    //     })
    // };
    

    html!(
        <>
            <div id="buttons">
                <button disabled={has_job} onclick={on_add_timeout}>{ "Start" }</button>
                //<button disabled={!has_job} onclick={on_pause}>{ "Pause" }</button>
                <button disabled={!has_job} onclick={on_cancel}>{ "Cancel"}</button>
            </div>
            <div id="user_input">
                // setting time
                <form {onsubmit} class="settings">
                    <FieldInput
                        label="Pomodoro Time"
                        input_type="number"
                        name="time"
                        node_ref={time_ref}
                    />
                    <button type="submit">{"Save"}</button>
                </form>
                

                
            </div>
            <div id="wrapper">
                <Clock />
                <div id="time_remaining">{ display_countdown }</div>
                <div id="messages">
                    { messages }
                </div>
            </div>
        </>
    )
}

