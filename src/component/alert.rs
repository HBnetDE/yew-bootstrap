use yew::classes;
use yew::prelude::*;

use crate::util::css_class::CssClass;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AlertType {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl Into<CssClass> for AlertType {
    fn into(self) -> CssClass {
        CssClass(match self {
            Self::Primary => "alert-primary",
            Self::Secondary => "alert-secondary",
            Self::Success => "alert-success",
            Self::Danger => "alert-danger",
            Self::Warning => "alert-warning",
            Self::Info => "alert-info",
            Self::Light => "alert-light",
            Self::Dark => "alert-dark",
        })
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct AlertProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(AlertType::Primary)]
    pub style: AlertType,

    #[prop_or_default]
    pub dismissable: bool,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let alert_type: CssClass = props.style.into();
    let visible = use_state_eq(|| true);
    let mut classes = classes!("alert", alert_type, &props.class);

    let onclick = {
        let visible = visible.clone();
        Callback::from(move |_| visible.set(false))
    };

    if props.dismissable {
        classes.push("alert-dismissible");
    }

    if *visible {
        html! {
            <div
                class={classes}
                role="alert"
            >
                { for props.children.iter() }
                {
                    if props.dismissable {
                        html! {
                            <button type="button" class={classes!("btn-close")} aria-label={"Close"} {onclick}>

                            </button>
                        }
                    } else { html!(<></>) }
                }
            </div>
        }
    } else {
        html!(<></>)
    }
}
