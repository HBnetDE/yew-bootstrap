use yew::prelude::*;
use yew_bootstrap::{
    component::{Button, ButtonColor},
    util::include_inline,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <section>
            {include_inline()}
            <h1>{"Alerts"}</h1>
            <section>
                        <h1>{"Buttons"}</h1>
                        <Button style={ButtonColor::Primary}>{"Primary"}</Button>
                        <Button style={ButtonColor::Secondary}>{"Secondary"}</Button>
                        <Button style={ButtonColor::Success}>{"Success"}</Button>
                        <Button style={ButtonColor::Danger}>{"Danger"}</Button>
                        <Button style={ButtonColor::Warning}>{"Warning"}</Button>
                        <Button style={ButtonColor::Info}>{"Info"}</Button>
                        <Button style={ButtonColor::Light}>{"Light"}</Button>
                        <Button style={ButtonColor::Dark}>{"Dark"}</Button>
                        <Button style={ButtonColor::Link}>{"Link"}</Button>
                    </section>
                    <section>
                        <h2>{"Outline buttons"}</h2>
                        <Button style={ButtonColor::Primary} outline={true}>{"Primary"}</Button>
                        <Button style={ButtonColor::Secondary} outline={true}>{"Secondary"}</Button>
                        <Button style={ButtonColor::Success} outline={true}>{"Success"}</Button>
                        <Button style={ButtonColor::Danger} outline={true}>{"Danger"}</Button>
                        <Button style={ButtonColor::Warning} outline={true}>{"Warning"}</Button>
                        <Button style={ButtonColor::Info} outline={true}>{"Info"}</Button>
                        <Button style={ButtonColor::Light} outline={true}>{"Light"}</Button>
                        <Button style={ButtonColor::Dark} outline={true}>{"Dark"}</Button>
                    </section>
        </section>
    }
}

fn main() {
    yew::start_app::<App>();
}
