use gloo_timers::callback::Interval;
use js_sys::Date;
use yew::{prelude::*, services::RenderService};

enum Message {
    Tick,
}

struct Model {
    link: ComponentLink<Self>,
    date: Date,
    interval: Interval,
}

impl Component for Model {
    type Message = Message;

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Message::Tick);
        let interval = Interval::new(1000, move || callback.emit(()));

        Self {
            link,
            date: Date::new_0(),
            interval,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Tick => self.date = Date::new_0(),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let hours_angle = (self.date.get_hours() as f32 % 12.0) / 12.0 * 360.0;
        let minutes_angle = (self.date.get_minutes() as f32 % 60.0) / 60.0 * 360.0;
        let seconds_angle = (self.date.get_seconds() as f32 % 60.0) / 60.0 * 360.0;

        html! {
            <svg style="display: block; height: 100%; width: 100%;" viewbox="0 0 100% 100%">
                <circle cx="50%" cy="50%" r="30%" fill="none" stroke="#1a202c"/>

                // hours
                <line x1="50%" y1="50%" x2="50%" y2="40%" stroke="#2d3748" stroke-width=6 stroke-linecap="round" transform=format!("rotate({})", hours_angle) transform-origin="50% 50%"/>

                // minutes
                <line x1="50%" y1="50%" x2="50%" y2="28%" stroke="#2d3748" stroke-width=3 stroke-linecap="round" transform=format!("rotate({})", minutes_angle) transform-origin="50% 50%"/>

                // seconds
                <line x1="50%" y1="50%" x2="50%" y2="24%" stroke="#e53e3e" stroke-width=2 stroke-linecap="round" transform=format!("rotate({})", seconds_angle) transform-origin="50% 50%"/>
            </svg>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
