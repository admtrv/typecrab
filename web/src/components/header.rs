use dioxus::prelude::*;

const LOGO_PNG: Asset = asset!("/assets/logo.min.png");

#[component]
pub fn Header() -> Element { 
    rsx! {
        header { 
                a{ href: "/",
                    img { id: "logo",
                        src: LOGO_PNG, 
                        alt: "Typecrab Logo"
                    }
                },
            nav { 
                ul { 
                    li {
                        a { 
                            href: "/",
                            "Test"
                        }
                    }
                    li {
                        a { 
                            href: "/settings",
                            "Settings"
                        }
                    }
                }
            }
        }
    }
}
