
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::Link;
use log::info;
#[component]
pub fn ForgetForm(cx: Scope) -> Element {
    let error_info = use_state(cx, || "".to_string());
    cx.render(rsx!{
        div{class:"py-24 px10",
            h2{class:"text-2xl font-semibold mb-2 text-center","Forgot Password"}
            form{class:"ml-6 mr-6",
                onsubmit: move |ev|{
                    info!("ev, {:?}",ev.values);
                    error_info.set("Not implemented".to_string());
                    ev.stop_propagation();
                },
                div{class:"mb-4",
                    div{class:"form-control w-full mt-4",
                        label{class:"label",
                            span{class:"label-text text-base-content undefined","Phone"}
                        }
                    }
                    input{class:"input  input-bordered w-full ",r#type:"text",name:"phone"}
                    div{class:"form-control w-full mt-4",
                        label{class:"label",
                            span{class:"label-text text-base-content undefined","Code"}
                        }
                    }
                    input{class:"input  input-bordered w-full ",r#type:"number",name:"code"}
                }
                div{class:"text-right text-primary",
                    p{class:"text-center  text-error mt-8","{error_info}"}
                    button{r#type:"submit", class:"btn mt-2 w-full btn-primary","Get Authenticate Code"}
                }
                div{class:"text-center mt-4", "Try login again?",
                    Link{to:"/login",
                        span{class:"inline-block  hover:text-primary hover:underline hover:cursor-pointer transition duration-200", " Login"}
                    }
                }
            }
        }
    })
} 