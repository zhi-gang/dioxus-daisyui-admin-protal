#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use log::info;
pub(crate) fn Login(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    render!{
        div{class:"min-h-screen bg-base-200 flex items-center",
            div{class:"card mx-auto w-full max-w-5xl  shadow-xl",
                div {class:"grid  md:grid-cols-2 grid-cols-1  bg-base-100 rounded-xl",
                    div{class:"hero min-h-full rounded-l-xl bg-base-200",
                        div{class:"hero-content py-12",
                            div{class:"max-w-md",
                                h1{class:"text-3xl text-center font-bold",
                                    img{class:"w-12 inline-block mr-2 mask mask-circle", alt:"dashwind-logo",src:"logo192.png"}
                                    "DashWind"
                                }
                                div{class:"text-center mt-12",
                                    img{class:"w-48 inline-block",alt:"Dashwind Admin Template",src:"intro.png"}
                                }
                                h1{class:"text-2xl mt-8 font-bold",
                                    "Admin Dashboard Starter Kit"
                                }
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
                    div{class:"py-24 px10",
                        h2{class:"text-2xl font-semibold mb-2 text-center","Login"}
                        div{class:"ml-6 mr-6",
                            div{class:"mb-4",
                                div{class:"form-control w-full mt-4",
                                    label{class:"label",
                                        span{class:"label-text text-base-content undefined","User Name"}
                                    }
                                }
                                input{class:"input  input-bordered w-full ",
                                    r#type:"Text",
                                    name:"username",
                                    placeholder:"",
                                    value:""
                                }
                                div{class:"form-control w-full mt-4",
                                    label{class:"label",
                                        span{class:"label-text text-base-content undefined","Password"}
                                    }
                                }
                                input{class:"input  input-bordered w-full ",
                                    r#type:"Text",
                                    name:"password",
                                    placeholder:"",
                                    value:""
                                }
                            }
                            div{class:"text-right text-primary",
                                a{href:"/forgot-password",
                                    span{class:"text-sm  inline-block  hover:text-primary hover:underline hover:cursor-pointer transition duration-200","Forgot Password?"}
                                }
                                p{class:"text-center  text-error mt-8",}
                                button{r#type:"submit", class:"btn mt-2 w-full btn-primary","Login"}
                            }
                            div{class:"text-center mt-4", "Don't have an account yet?",
                                a{href:"/register",
                                    span{class:"inline-block  hover:text-primary hover:underline hover:cursor-pointer transition duration-200", "Register"}
                                }
                            }  
                        }
                    }
                }
            }   
        }
    }

   
}
