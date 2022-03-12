pub(crate) use util::css_class::CssClass;

macro_rules! css_class_enum {
    ($name:ident, [$($variant:ident => $class:literal),*]) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant),*
        }

        impl Into<$crate::CssClass> for $name {
            fn into(self) -> $crate::CssClass {
                $crate::CssClass(match self {
                    $(Self::$variant => $class),*
                })
            }
        }
    };
}

pub mod component;
pub mod util;
