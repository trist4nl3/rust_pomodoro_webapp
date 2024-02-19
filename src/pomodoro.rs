use gloo::console::{self, Timer};
use gloo::timers::callback::{Interval, Timeout};
use yew::{html, Component, Context, Html};
use web_sys::HtmlInputElement;



pub enum Msg {
    StartTimeout,
    Cancel,
    Done,
    UpdateCountdown,
    SetTime(u32),

}

pub struct Pomodoro {
    time: String,
    countdown: u32,
    messages: Vec<&'static str>,
    interval: Option<Interval>,
    timeout: Option<Timeout>,
    console_timer: Option<Timer<'static>>,
}

impl Pomodoro {
    fn cancel(&mut self) {
        self.timeout = None;
        self.interval = None;
    }
}

impl Component for Pomodoro {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            time: 1500, // 25 minutes worth of seconds
            countdown: 0,
            messages: Vec::new(),
            interval: None,
            timeout: None,
            console_timer: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartTimeout => {
                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(self.time * 1000, move || link.send_message(Msg::Done))
                };
                let counter = {
                    let link = ctx.link().clone();
                    Interval::new(1000, move || {
                        link.send_message(Msg::UpdateCountdown);
                    })
                };
                       
                self.countdown = self.time;
                self.messages.clear();
                console::clear!();
                self.messages.push("Timer started!");
                self.console_timer = Some(Timer::new("Timer"));
                self.timeout = Some(handle);
                self.interval = Some(counter);
                true
            }
            Msg::Cancel => {
                self.cancel();
                self.messages.push("Canceled!");
                console::warn!("Canceled!");
                true
            }
            Msg::Done => {
                self.cancel();
                self.messages.clear();
                self.messages.push("Done!");
                console::info!("Done!");
                if let Some(timer) = self.console_timer.take() {
                    drop(timer);
                }
                true
            }

            Msg::UpdateCountdown => {
                if self.countdown > 0 {
                    self.countdown -= 1;
                }
                true
            }
            Msg::SetTime(time) => {
                self.time = time;
                true
            }

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let has_job = self.timeout.is_some() || self.interval.is_some();
        let show_countdown = self.timeout.is_some(); // Only show countdown if timeout is active
    
        html! {
            <>
                <div id="buttons">
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StartTimeout)}>
                        { "Start" }
                    </button>
                    <button disabled={!has_job} onclick={ctx.link().callback(|_| Msg::Cancel)}>
                        { "Cancel!" }
                    </button>
                    
                </div>
                <div id="time-input">
                <input
                    type="number"
                    value={self.time.to_string()} // Convert u32 to string
                    oninput={ctx.link().callback(|e: InputData| Msg::SetTime(e.value.parse().unwrap_or_default()))}
                />
                <button onclick={ctx.link().callback(|_| Msg::SetTime(1500))}>
                    { "Default" }
                </button>
                </div>
                <div id="wrapper">
                    <div id="timer">
                    // Format for minutes and seconds
                        <p>{ format!("{}:{:02}", self.countdown / 60, self.countdown % 60) }</p>
                        { if show_countdown { html! { <p>{ format!("Countdown: {} seconds", self.countdown) }</p> } } else { html! {} } }
                    </div>
                    <div id="messages">
                        { for self.messages.iter().map(|message| html! { <p>{ *message }</p> }) }
                    </div>
                </div>
            </>
        }
    }
}
