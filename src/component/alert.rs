use yew::classes;
use yew::prelude::*;

use crate::util::Color;

#[derive(Properties, Clone, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(Color::Primary)]
    pub style: Color,

    #[prop_or_default]
    pub text: String,
}

#[function_component(Alert)]
pub fn alert(props: &ComponentProps) -> Html {
    html! {
        <div
            class={classes!("alert", format!("alert-{}", props.style), &props.class)}
            role="alert"
        >
            { &props.text }
            { for props.children.iter() }
        </div>
    }
}
