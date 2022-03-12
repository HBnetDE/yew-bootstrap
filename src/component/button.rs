#![allow(dead_code)]

use yew::prelude::*;

use crate::util::css_class::CssClass;

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Large,
    Normal,
    Small,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Normal
    }
}

css_class_enum!(ButtonColor, [
    Primary => "btn-primary",
    Secondary => "btn-secondary",
    Success => "btn-success",
    Danger => "btn-danger",
    Warning => "btn-warning",
    Info => "btn-info",
    Light => "btn-light",
    Dark => "btn-dark",
    Link => "btn-link"
]);

impl ButtonColor {
    fn outlined(&self) -> String {
        let class: CssClass = self.clone().into();
        let split: Vec<_> = class.0.split("-").collect();
        let color = split.last().unwrap();
        format!("btn-outline-{}", color)
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub name: String,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub outline: bool,

    #[prop_or_default]
    pub size: ButtonSize,

    #[prop_or(ButtonColor::Primary)]
    pub style: ButtonColor,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let mut classes = classes!("btn", &props.class);

    if props.outline {
        classes.push(props.style.outlined())
    } else {
        let style_class: CssClass = props.style.into();
        classes.push(style_class);
    }

    match props.size {
        ButtonSize::Large => classes.push("btn-lg"),
        ButtonSize::Small => classes.push("btn-sm"),
        _ => (),
    };

    html! {
        <button class={classes}
            disabled={props.disabled}
            onclick={&props.onclick}
            name={props.name.clone()}>
            { for props.children.iter() }
        </button>
    }
}
