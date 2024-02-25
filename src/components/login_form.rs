#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::{components::Link, hooks::use_navigator};
use log::info;
#[component]
pub fn LoginForm(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let error_info = use_state(cx, || "".to_string());
    cx.render(rsx!{
        div{class:"py-24 px10",
            h2{class:"text-2xl font-semibold mb-2 text-center","Login"}
            form{class:"ml-6 mr-6",
                onsubmit: move |ev|{
                    info!("ev, {:?}",ev.values);
                    error_info.set("".to_string());
                    if ev.values["username"].len() ==0 || ev.values["username"].get(0).unwrap().len()==0{
                        error_info.set("username is required".to_string());
                    }else if ev.values["password"].len() ==0 || ev.values["password"].get(0).unwrap().len()==0{
                        error_info.set("password is required".to_string());
                    }else{
                        info!("login: user: {:?}, pwd: {:?} ", ev.values["username"].get(0).unwrap(),ev.values["password"].get(0).unwrap());
                        //invok auth api
                        nav.push("/home");
                    }
                    ev.stop_propagation();
                },
                div{class:"mb-4",
                    div{class:"form-control w-full mt-4",
                        label{class:"label",
                            span{class:"label-text text-base-content undefined","User Name"}
                        }
                    }
                    input{class:"input  input-bordered w-full ",r#type:"text",name:"username"}
                    div{class:"form-control w-full mt-4",
                        label{class:"label",
                            span{class:"label-text text-base-content undefined","Password"}
                        }
                    }
                    input{class:"input  input-bordered w-full ",r#type:"password",name:"password"}
                }
                div{class:"text-right text-primary",
                    Link{to:"/login/forget",
                        span{class:"text-sm  inline-block  hover:text-primary hover:underline hover:cursor-pointer transition duration-200","Forgot Password?"}
                    }
                    p{class:"text-center  text-error mt-8","{error_info}"}
                    button{r#type:"submit", class:"btn mt-2 w-full btn-primary","Login"}
                }
                div{class:"text-center mt-4", "Don't have an account yet?",
                    Link{to:"/login/register",
                        span{class:"inline-block  hover:text-primary hover:underline hover:cursor-pointer transition duration-200", "Register"}
                    }
                }
            }
        }
    })
} 