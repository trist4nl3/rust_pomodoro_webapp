use yew::prelude::*;
use crate::components::timer::Pomodoro; // Import the App component from timer.rs

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
            <Pomodoro />
        </div>
    }
}