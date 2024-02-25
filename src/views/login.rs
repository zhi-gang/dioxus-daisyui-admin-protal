#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::Outlet;

use crate::route::Route;
// use dioxus_router::{components::Link, hooks::use_navigator};
// use log::info;

pub(crate) fn Login(cx: Scope) -> Element {
    render! {
        div{class:"min-h-screen bg-base-200 flex items-center",
            div{class:"card mx-auto w-full max-w-5xl  shadow-xl",
                div {class:"grid  md:grid-cols-2 grid-cols-1  bg-base-100 rounded-xl",
                    div{class:"hero min-h-full rounded-l-xl bg-base-200",
                        div{class:"hero-content py-12",
                            div{class:"max-w-md",
                                h1{class:"text-3xl text-center font-bold",
                                    img{class:"w-12 inline-block mr-2 mask mask-circle", alt:"dashwind-logo",src:"../logo192.png"}
                                    "DashWind"
                                }
                                div{class:"text-center mt-12",
                                    img{class:"w-48 inline-block",alt:"Dashwind Admin Template",src:"../intro.png"}
                                }
                                h1{class:"text-2xl mt-8 font-bold", "Admin Dashboard Starter Kit"}
                                p{class:"py-2 mt-4",
                                    "✓ ",
                                    span{class:"font-semibold","Light/dark"}
                                    "mode ToggleData"
                                }
                                p{class:"py-2",
                                    "✓ ",
                                    span{class:"font-semibold","Redux toolkit"}
                                    "and other utility libraries configured"
                                }
                                p{class:"py-2",
                                    "✓ ",
                                    span{class:"font-semibold","Calendar, Modal, Sidebar"}
                                    "components"
                                }
                                p{class:"py-2",
                                    "✓ User-friendly",
                                    span{class:"font-semibold","documentation"}
                                }
                                p{class:"py-2  mb-4",
                                    "✓ ",
                                    span{class:"font-semibold","Daisy UI"}
                                    "components,",
                                    span{class:"font-semibold","Tailwind CSS"}
                                    "support"
                                }
                            }
                        }
                    }
                    Outlet::<Route> {}
                }
            }
        }
    }
}
