macro_rules! css_class_enum {
    ($name:ident, [$($variant:ident => $class:literal),*]) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant),*
        }

        impl Into<&'static str> for $name {
            fn into(self) -> &'static str {
                match self {
                    $(Self::$variant => $class),*
                }
            }
        }

        impl Into<yew::Classes> for $name {
            fn into(self) -> yew::Classes {
                let class: &'static str = self.into();
                classes!(class)
            }
        }

        impl $name {
            pub fn into_classes(self) -> yew::Classes {
                self.into()
            }
        }
    };
}

pub mod component;
pub mod util;
