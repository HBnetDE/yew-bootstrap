use yew::prelude::*;

css_class_enum!(ButtonGroupSize, [
    Small => "btn-group-sm",
    Normal => "",
    Large => "btn-group-lg"
]);

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub role: String,

    #[prop_or_default]
    pub vertical: bool,

    #[prop_or(ButtonGroupSize::Normal)]
    pub size: ButtonGroupSize,
}

#[function_component(ButtonGroup)]
pub fn button_group(props: &ButtonGroupProps) -> Html {
    let mut classes = classes!("btn-group", &props.class);

    classes.push(props.size.into_classes());

    html! {
        <div class={classes} role={"group"}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonToolbarProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(ButtonToolbar)]
pub fn button_toolbar(props: &ButtonToolbarProps) -> Html {
    html! {
        <div class={classes!("btn-toolbar", &props.class)} role={"toolbar"}>
            { for props.children.iter() }
        </div>
    }
}

/* impl Component for ButtonGroup {
    type Message = ();
    type Properties = ComponentProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::new();
        if self.props.vertical {
            classes.push("btn-group-vertical");
        } else {
            classes.push("btn-group");
        }
        classes.push(self.props.class.clone());

        html! {
            <div
                class=classes
                role=self.props.role.clone()
                aria-label=self.props.label.clone()
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}
 */
