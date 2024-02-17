use gloo::console::{self, Timer};
use gloo::timers::callback::{Interval, Timeout};
use yew::{html, Component, Context, Html};

pub enum Msg {
    StartTimeout,
    Cancel,
    Done,
    UpdateTime,
    UpdateCountdown,

}

pub struct App {
    time: u32,
    countdown: u32,
    messages: Vec<&'static str>,
    _standalone: (Interval, Interval),
    interval: Option<Interval>,
    timeout: Option<Timeout>,
    console_timer: Option<Timer<'static>>,
}

impl App {
    fn cancel(&mut self) {
        self.timeout = None;
        self.interval = None;
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let standalone_handle =
            Interval::new(10, || console::debug!("Example of a standalone callback."));

        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            time: 10, // seconds
            countdown: 0,
            messages: Vec::new(),
            _standalone: (standalone_handle, clock_handle),
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
            Msg::UpdateTime => {
                
                true
            }
            Msg::UpdateCountdown => {
                if self.countdown > 0 {
                    self.countdown -= 1;
                }
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
                        { "Start Timeout" }
                    </button>
                    <button disabled={!has_job} onclick={ctx.link().callback(|_| Msg::Cancel)}>
                        { "Cancel!" }
                    </button>
                    
                </div>
                <div id="wrapper">
                    <div id="timer">
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

fn main() {
    yew::Renderer::<App>::new().render();
}