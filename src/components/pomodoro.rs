use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{HtmlInputElement, HtmlAudioElement};
use gloo::timers::callback::{Interval, Timeout};
use yew::prelude::*;
use wasm_bindgen::JsValue;
use crate::services::timer::{TimerAction, TimerState};
use crate::components::inputfield::FieldInput;




#[function_component]
pub fn Pomodoro() -> Html {
    // Declaring state for utils
    let state = use_reducer(TimerState::new);
    // Node references for input fields
    let work_time_ref = use_node_ref();
    let break_time_ref = use_node_ref();
    // Displaying the time remaining
    let time_str = format!("{:02}:{:02}", state.time_remaining / 60, state.time_remaining % 60);
    let display_countdown: Html = html! {
        <div id="time">
            { time_str.clone() }
        </div>
    };
    // Loading sound files
    let button_sound_ref = use_node_ref();
    let alarm_sound_ref = use_node_ref();
    let button_sound_url= "/src/assets/button.wav"; // Adjust the path to your first sound file
    let alarm_sound_url = "/src/assets/alarm.mp3";

    // Function to play sound
    let play_sound = |sound_ref: &NodeRef| {
        if let Some(audio) = sound_ref.cast::<HtmlAudioElement>() {
            // Attempt to play the audio
            let play_promise = audio.play();
    
            // Handle the result of the promise
            if let Ok(play_promise) = play_promise {
                // If the promise is successful, do nothing (audio will play)
                play_promise.then(&wasm_bindgen::closure::Closure::wrap(Box::new(move |_result: JsValue| {
                    // Do nothing, audio will play
                }) as Box<dyn FnMut(JsValue)>));
    
            } else {
                // If there's an error, log it
                log::error!("Failed to play sound: {:?}", play_promise);
            }
        } else {
            log::error!("Failed to cast NodeRef to HtmlAudioElement");
        }
    };

    // Checking if theres a job going on to disable buttons
    let has_job = state.timeout_handle.is_some();

    // Declaration for whehter button is being pressed
    let setting_pressed = Rc::new(RefCell::new(false));
    // Make a clone to pass to the button
    let setting_pressed_clone = Rc::clone(&setting_pressed);

    // Methods for settings popup
    // @To Do Move this to a seperate file
    let on_settings = {
        let setting_pressed_clone = setting_pressed_clone.clone(); // Clone again for the closure
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
        let setting_pressed_clone = setting_pressed.clone(); // Clone again for the closure
        Callback::from(move |_: MouseEvent| {
            let mut setting_pressed_ref = setting_pressed_clone.borrow_mut();
            let user_input = web_sys::window().unwrap().document().unwrap().get_element_by_id("user_input").unwrap();
            
    
            user_input.set_attribute("style", "display: none;").unwrap();
            *setting_pressed_ref = false;
        })
    };

    // Declaring variables to be retrieved for display purposes
    let work_time = state.clone().work_time.to_string();
    let break_time = state.clone().break_time.to_string();
    let time_amount = state.clone().time_amount.to_string();
    let get_countdown = state.clone().time_remaining.to_string();
    let get_current_state = if !state.on_break { "Work" } else { "Break" };

    // Method for making the title of the page
    let getTitle = if !state.clone().running {
        html! {<title> {"Pomodoro Timer | Neblume "}</title>}
    } else {
        html! {<title>{get_current_state}{" : "}{time_str.clone()} </title>}
    };

    // Methods for whether or not the work/break button is highlighted
    let work_button_class = if !state.on_break { "highlighted" } else { "" };
    let break_button_class = if state.on_break { "highlighted" } else { "" };

    // Method for starting the timer
    let start_timer = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent|{
            let timer_state = state.clone();
            let interval_state = state.clone();
            let tick_state = state.clone();
            let complete_state = state.clone();
            // Getting overall time saved
            let time = state.clone().time_remaining;
            // Starting the countdown and ticking every second
            let countdown_state = state.clone();
            countdown_state.dispatch(TimerAction::SetCountdown(time));
            let i = Rc::new(Interval::new(1000, move || {
                tick_state.dispatch(TimerAction::UpdateCountdown);
            }));
            // For when the timer is completed
            let t = Rc::new(Timeout::new(time * 1000, move || {
                complete_state.dispatch(TimerAction::TimeoutDone);
                complete_state.dispatch(TimerAction::SaveTimeInput(
                    complete_state.clone().work_time,
                    complete_state.clone().break_time,
                ));
            }));
            // Starting the timer and interval
            timer_state.dispatch(TimerAction::SetInterval(i));
            timer_state.dispatch(TimerAction::SetTimeout(t));
        })
    };

    // Method to parallel tasks when the start button is pressed calls both start_timer and exit_settings
    let start_timer_parallel = {
        let start_timer = start_timer.clone();
        let on_exit_settings = on_exit_settings.clone();
        Callback::from(move |event: MouseEvent| {
            start_timer.emit(event.clone());
            on_exit_settings.emit(event.clone());
        })
    };

    // Method for resetting the timer
    let on_cancel = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(TimerAction::Cancel);
        })
    };

    // Method for pausing the timer
    let on_pause = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(TimerAction::Pause);
        })
    };

    // Method for when the work button is pressed
    let on_work = {
        let state = state.clone();
        let button_ref = button_sound_ref.clone();
        Callback::from(move |_: MouseEvent| {
            play_sound(&button_ref);
            state.dispatch(TimerAction::SetTime(state.clone().work_time));
            state.dispatch(TimerAction::OnBreak(false));
            state.dispatch(TimerAction::Cancel);
        })
    };
    
    // Method for when the break button is pressed
    let on_break = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(TimerAction::SetTime(state.clone().break_time));
            state.dispatch(TimerAction::OnBreak(true));
            state.dispatch(TimerAction::Cancel);
        })
    };

    

    // Method for saving the settings
    let onsubmit = {
        let state = state.clone();
        let work_state = state.clone();
        let break_state = state.clone();
        let work_time_ref = work_time_ref.clone();
        let break_time_ref = break_time_ref.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let work_time = work_time_ref.cast::<HtmlInputElement>().unwrap().value().parse().unwrap();
            let break_time = break_time_ref.cast::<HtmlInputElement>().unwrap().value().parse().unwrap();
            work_state.dispatch(TimerAction::SetWork(work_time));
            break_state.dispatch(TimerAction::SetBreak(break_time));
            state.dispatch(TimerAction::SaveTimeInput(work_time, break_time));
            state.dispatch(TimerAction::Cancel);
        })
    };

    html!(
        <>
        { getTitle }
        <audio ref={button_sound_ref} src={"audio.mp3"} preload="auto" style="display: none;"></audio>
        <audio ref={alarm_sound_ref} src={alarm_sound_url} preload="auto" style="display: none;"></audio>
        <div id="background">
        <div id="content">
            <div id="title-area">
                <h1 class="title">{ "Pomodoro Timer" }</h1>
                <h2 class="subtitle">{ "By Neblume"}</h2>
            </div>
            <div id="switch_states">
                <button class={"button ".to_owned() + work_button_class} onclick={on_work}>{"Work"}</button>
                <button class={"button ".to_owned() + break_button_class} onclick={on_break}>{"Break"}</button>
            </div>
            <div id="timer_wrapper">
                <div id="time_remaining">{ display_countdown }</div>
            </div>
            <div id="buttons">
                {if !has_job {
                html!(
                    <button class="button" disabled={has_job} onclick={start_timer_parallel}>{ "Start" }</button>
                )
                } else {
                html!(
                    <button class="button" disabled={!has_job} onclick={on_pause}>{ "Pause" }</button>
                )
                }}
                <button class="button" onclick={on_cancel}><i class="fas fa-redo-alt"></i></button>
                <div id="settings-button">
                    <button class="button" disabled={has_job} onclick={on_settings}><i class="fas fa-cog"></i></button>
                    <div id="settings-menu">
                        <div id="user_input" class="settings" style="display: none;">
                            <button class="exit-button" onclick={on_exit_settings.clone()}><i class="fas fa-times"></i></button>
                            // setting time
                            <form {onsubmit}>
                                <FieldInput
                                    label="Work time:"
                                    input_type="number"
                                    name=""
                                    value={work_time}
                                    node_ref={work_time_ref}
                                />
                                <FieldInput
                                    label="Break time:"
                                    input_type="number"
                                    name=""
                                    value={break_time}
                                    node_ref={break_time_ref}

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