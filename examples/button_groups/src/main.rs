use yew::prelude::*;
use yew_bootstrap::{
    component::{Button, ButtonColor, ButtonGroup, ButtonToolbar},
    util::include_inline,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            {include_inline()}
            <h1>{ "Basic example"}</h1>
            <ButtonGroup>
                <Button style={ButtonColor::Primary}>{ "Left" }</Button>
                <Button style={ButtonColor::Primary}>{ "Middle" }</Button>
                <Button style={ButtonColor::Primary}>{ "Right" }</Button>
            </ButtonGroup>
            <h1>{ "Mixed styles" }</h1>
            <ButtonGroup>
                <Button style={ButtonColor::Danger}>{ "Left" }</Button>
                <Button style={ButtonColor::Warning}>{ "Middle" }</Button>
                <Button style={ButtonColor::Success}>{ "Right" }</Button>
            </ButtonGroup>
            <h1>{ "Outlined styles" }</h1>
            <ButtonGroup>
                <Button style={ButtonColor::Primary} outline={true}>{ "Left" }</Button>
                <Button style={ButtonColor::Primary} outline={true}>{ "Middle" }</Button>
                <Button style={ButtonColor::Primary} outline={true}>{ "Right" }</Button>
            </ButtonGroup>
            <h1>{ "Button toolbar" }</h1>
            <ButtonToolbar>
                <ButtonGroup class={"me-2"}>
                    <Button>{ "1" }</Button>
                    <Button>{ "2" }</Button>
                    <Button>{ "3" }</Button>
                    <Button>{ "4" }</Button>
                </ButtonGroup>
                <ButtonGroup class={"me-2"}>
                    <Button style={ButtonColor::Secondary}>{ "5" }</Button>
                    <Button style={ButtonColor::Secondary}>{ "6" }</Button>
                    <Button style={ButtonColor::Secondary}>{ "7" }</Button>
                </ButtonGroup>
                <ButtonGroup>
                    <Button style={ButtonColor::Info}>{ "8" }</Button>
                </ButtonGroup>
            </ButtonToolbar>
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
