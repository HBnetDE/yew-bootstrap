use yew::prelude::*;
use yew_bootstrap::{
    component::{Alert, AlertLink, AlertType},
    util::include_inline,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <section>
            {include_inline()}
            <h1>{"Alerts"}</h1>
            <Alert style={AlertType::Primary} dismissable={true}>
                { "This is a " } <AlertLink>{ "primary" }</AlertLink> {" alert—check it out!"}
            </Alert>
            <Alert style={AlertType::Secondary} dismissable={true} class={"show fade"}>
                { "This is a secondary alert—check it out!" }
            </Alert>
            <Alert style={AlertType::Success}>
                { "This is a success alert—check it out!" }
            </Alert>
            <Alert style={AlertType::Danger}>
                { "This is a danger alert—check it out!" }
            </Alert>
            <Alert style={AlertType::Warning}>
                { "This is a warning alert—check it out!" }
            </Alert>
            <Alert style={AlertType::Info}>
                { "This is a info alert—check it out!" }
            </Alert>
            <Alert style={AlertType::Light}>
                { "This is a light alert—check it out!" }
            </Alert>
            <Alert style={AlertType::Dark}>
                { "This is a dark alert—check it out!" }
            </Alert>
        </section>
    }
}

fn main() {
    yew::start_app::<App>();
}
