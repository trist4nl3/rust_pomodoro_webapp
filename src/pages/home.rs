use yew::prelude::*;
use crate::components::pomodoro::Pomodoro; // Import the App component from timer.rs

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            
            <Pomodoro />
        </div>
    }
}