use yew::classes;
use yew::prelude::*;

css_class_enum!(AlertType, [
    Primary => "alert-primary",
    Secondary => "alert-secondary",
    Success => "alert-success",
    Danger => "alert-danger",
    Warning => "alert-warning",
    Info => "alert-info",
    Light => "alert-light",
    Dark => "alert-dark"
]);

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
    let visible = use_state_eq(|| true);
    let mut classes = classes!("alert", &props.class, props.style.into_classes());

    let onclick = {
        let visible = visible.clone();
        Callback::from(move |_| visible.set(false))
    };

    if props.dismissable {
        classes.push("alert-dismissible");
    }

    // TODO: events
    // To be the same as the bootstrap javascript library this needs events

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

#[derive(Properties, PartialEq)]
pub struct AlertLinkProps {
    #[prop_or("#".to_string())]
    pub to: String,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(AlertLink)]
pub fn alert_link(props: &AlertLinkProps) -> Html {
    html! {
        <a href={props.to.clone()} class={classes!("alert-link")}>{ for props.children.iter() }</a>
    }
}
