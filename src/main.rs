use gloo_timers::callback::Interval;
use js_sys::Date;
use yew::{prelude::*, services::RenderService};

// #[derive(Clone)]
// struct TickProperties {
//     angle: f64,
//     stroke: u32,
//     width: u32,
//     height: u32,
// }

// impl Properties for TickProperties {}

// struct Tick {
//     link: ComponentLink<Self>,
//     props: TickProperties,
// }

// impl Component for Tick {
//     type Message = ();

//     type Properties = TickProperties;

//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self { link, props }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         todo!()
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         todo!()
//     }

//     fn view(&self) -> Html {
//         html! {
//             <line x1=100 y1=100 x2=format!("{}", 100-self.props.height) y2=100 stroke=format!("{}", self.props.stroke) stroke-width=format!("{}", self.props.width) stroke-linecap="round" transform=format!("rotate({} 100 100)", self.props.angle) />
//         }
//     }
// }

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
        html! {
            <div style="height: 100%;">
                <svg style="display: block; margin-left: auto; margin-right: auto; height: 100%;" viewbox="0 0 100% 100%">
                    <circle cx=100 cy=100 r=98 fill="none" stroke="#1a202c"/>

                    // hours
                    <line x1=100 y1=100 x2=50 y2=100 stroke="#2d3748" stroke-width=4 stroke-linecap="round" transform=format!("rotate({} 100 100)", (self.date.get_hours() as f32 % 12.0) / 12.0 * 360.0) />

                    // minutes
                    <line x1=100 y1=100 x2=30 y2=100 stroke="#2d3748" stroke-width=3 stroke-linecap="round" transform=format!("rotate({} 100 100)", (self.date.get_minutes() as f32 % 60.0) / 60.0 * 360.0) />

                    // seconds
                    <line x1=100 y1=100 x2=10 y2=100 stroke="#e53e3e" stroke-width=2 stroke-linecap="round" transform=format!("rotate({} 100 100)", (self.date.get_seconds() as f32 % 60.0) / 60.0 * 360.0) />
                </svg>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
