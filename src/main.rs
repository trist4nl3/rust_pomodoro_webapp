mod pomodoro;

use yew::prelude::*;

use pomodoro::Pomodoro;

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <h1 class="text-3xl font-bold text-center mt-4">{"Pomodoro"}</h1>
            <Pomodoro />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
