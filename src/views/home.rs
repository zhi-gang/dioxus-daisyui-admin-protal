#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::Outlet;
use log::info;

use crate::route::Route;

pub fn Home(cx: Scope) ->Element {
    let mode = use_shared_state::<String>(cx).unwrap();
    render!{
        div{class:"drawer lg:drawer-open",
            input{id:"my-drawer-2", r#type:"checkbox", class:"drawer-toggle"}
            div{class:"drawer-content flex flex-col",
                div{class:"navbar sticky top-0 bg-base-100  z-10 shadow-md ",
                    div{class:"flex-1",
                        label{r#for:"my-drawer-2", class:"btn btn-primary drawer-button lg:hidden",
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none",  stroke:"currentColor" ,class:"h-5 inline-block w-5 stroke-2",
                                path{d:"M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",style:"stroke-linecap:round;stroke-linejoin:round"}
                            }
                        }
                    }
                    div{class:"",
                        label{class:"swap ",
                            input{r#type:"checkbox",
                                onclick:move |_ev|{
                                    if *mode.read() == "light".to_string(){
                                        *mode.write() = "dark".to_string();
                                    }else{
                                        *mode.write() = "light".to_string();
                                    }
                                }
                            
                            },
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none", stroke:"currentColor", class:"fill-current w-6 h-6 swap-off stroke-2",
                                path{style:"stroke-linecap:round;stroke-linejoin:round", d:"M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z"}
                            }
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none", stroke:"currentColor", class:"fill-current w-6 h-6 swap-on",
                                path{style:"stroke-linecap:round;stroke-linejoin:round", d:"M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z"}
                            }   
                        }
                    }
                }
                main{class:"flex-1 overflow-y-auto md:pt-4 pt-4 px-6  bg-base-200",
                    Outlet::<Route> {}
                }
            }  
            div{class:"drawer-side z-30",
                label{r#for:"my-drawer-2" , class:"drawer-overlay"}
                ul{class:"menu pt-2  w-80 min-h-full bg-base-100 text-base-content",
                    li{class:"mb-2 font-semibold text-xl",
                    a{href:"/app/welcome",
                        img{class:"mask mask-squircle w-10", src:"../logo192.png",alt:"DashWind Logo"}
                        "DashWind"
                    }
                    }
                    li{class:"",
                        a{class:"font-normal", href:"/app/dashboard",
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none", stroke:"currentColor", class:"h-6 w-6 stroke-2",
                                path{d:"M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z",style:"stroke-linecap:round;stroke-linejoin:round",}
                            } 
                            "Dashboard"
                        }
                    }
                }
            }    
        }
    }
}

/* pub(crate) fn Home(cx: Scope) -> Element {
    // let nav = use_navigator(cx);
    render!(
        div{class:"drawer lg:drawer-open",
            id:"main",
            input{id:"drawer-box", r#type:"checkbox", class:"drawer-toggle"},
            div{class:"drawer-content flex flex-col ",
                div{class:"navbar sticky top-0 bg-base-100  z-10 shadow-md ",
                    id: "header"
                }
                main{class:"flex-1 overflow-y-auto md:pt-4 pt-4 px-6  bg-base-200",
                    Outlet::<Route> {}
                }
            }
            div{class:"drawer-side z-30" ,
                label{r#for:"drawer-box", class:"drawer-overlay"}, 
                div{style:"width:10rem", 
                    ul{class:"menu  pt-2 w-80 bg-base-100 min-h-full text-base-content",
                        button{class:"btn btn-ghost bg-base-300  btn-circle z-50 top-0 right-0 mt-4 mr-2 absolute lg:hidden",
                            svg{xmlns:"http://www.w3.org/2000/svg", fill:"none",  stroke:"currentColor",class:"h-5 inline-block w-5",
                                path{ d:"M6 18L18 6M6 6l12 12", style:"stroke-linecap:round;stroke-linejoin:round"}
                            }
                        }
                        li{class:"mb-2 font-semibold text-xl",
                            a{href:"/app/welcome",
                                img{class:"mask mask-squircle w-10", src:"../logo192.png",alt:"DashWind Logo"}
                                "DashWind"
                            }
                        }
                        li{class:"",
                            a{class:"font-normal", href:"/app/dashboard",
                                svg{xmlns:"http://www.w3.org/2000/svg", fill:"none", stroke:"currentColor", class:"h-6 w-6",
                                    path{d:"M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z",style:"stroke-linecap:round;stroke-linejoin:round",}
                                } 
                                "Dashboard"
                            }
                        }
                    }
                }
            }
        })
} */

