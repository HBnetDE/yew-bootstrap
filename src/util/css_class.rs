use yew::{classes, Classes};

pub(crate) struct CssClass(pub &'static str);

impl Into<Classes> for CssClass {
    fn into(self) -> Classes {
        classes!(self.0)
    }
}
