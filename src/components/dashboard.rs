#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::customized_svg::CustomizedSvg;
pub(crate) fn Dashboard(cx: Scope) -> Element {
    render!(
        main{class:"container mx-w-6xl mx-auto py-4",
            div{class:"flex flex-col space-y-8",
                //start first row
                div{class:"grid grid-cols-1 md:grid-cols-4 xl:grid-cols-5 px-4 xl:p-0 gap-y-4 md:gap-6",
                    div{class:"md:col-span-2 xl:col-span-3 bg-base-100 p-6 rounded-2xl border border-base-50",
                        div{class:"flex flex-col space-y-6 md:h-full md:justify-between",
                            div{class:"flex justify-between",
                                span{class:"text-xs font-semibold uppercase tracking-wider","Main Account"}
                                span{class:"text-xs  font-semibold uppercase tracking-wider","Avaiable Funds"}
                            }
                            div{class:"flex gap-2 md:gap-4 justify-between items-center",
                                div{class:"flex flex-col space-y-4",
                                    h2{class:"font-bold tracking-widest leading-tight","Derol's Savings Accoun"}
                                    div{class:"flex items-center gap-4",
                                        p{class:"text-lg tracking-wider", "**** **** *321"}
                                        CustomizedSvg{s:6, d:"M17 8l4 4m0 0l-4 4m4-4H3"}
                                    },
                                }
                                h2{class:"text-lg md:text-xl xl:text-3xl font-black tracking-wider",
                                    span{class:"md:text-xl","$"}
                                    "92,817.45"
                                }
                            }
                            div{class:"flex gap-2 md:gap-4",
                                a{href:"#", class:"bg-blue-600 px-5 py-3 w-full text-center md:w-auto rounded-lg text-white text-xs tracking-wider font-semibold hover:bg-blue-800","Transfer Money"}
                                a{href:"#", class:"bg-blue-50 px-5 py-3 w-full text-center md:w-auto rounded-lg text-blue-600 text-xs tracking-wider font-semibold hover:bg-blue-600 hover:text-white", "Link Account"}
                            }
                        }
                    }
                    div{class:"col-span-2 p-6 rounded-2xl bg-gradient-to-r from-blue-500 to-blue-800 flex flex-col justify-between",
                        div{class:"text-white font-bold","Lorem ipsum dolor sit amet"}
                        p{class:"mt-1 text-xs md:text-sm text-gray-50 font-light leading-tight max-w-sm",
                            "Lorem ipsum dolor sit amet consectetur adipisicing elit. Odio soluta saepe consequuntur
                            facilis ab a. Molestiae ad saepe assumenda praesentium rem dolore? Exercitationem, neque
                            obcaecati?"
                        }
                    }
                }
                //end first row
                //start second row
                div{class:"grid grid-cols-1 md:grid-cols-5 items-start px-4 xl:p-0 gap-y-4 md:gap-6",
                    div{class:"col-start-1 col-end-5",
                        h2{class:"text-xs md:text-sm  font-bold tracking-wide","Summary Transactions"}
                    }
                    div{class:"col-span-2 bg-base-100 p-6 rounded-xl border border-gray-50 flex flex-col space-y-6",
                        div{class:"grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 flex justify-between items-center",
                            div{class:"p-4 cursor-pointer border",
                                span{class:"text-xs  font-semibold","Daily"}
                                h2{class:" font-bold tracking-wider", "$ 27.80"}
                             }
                            div{class:"p-4 cursor-pointer border",
                                span{class:"text-xs  font-semibold","Weekly"}
                                h2{class:" font-bold tracking-wider","$ 192.92"}
                            }
                            div{class:"p-4 cursor-pointer border",
                                span{class:"text-xs  font-semibold", "Monthly"}
                                h2{class:" font-bold tracking-wider","$ 501.10"}
                            }  
                        }
                        canvas{id:"myChart"}
                    }
                    div{class:"col-span-3 bg-base-100 p-6 rounded-xl border border-base-50 flex flex-col space-y-6",
                        div{class:"flex justify-between items-center",
                            h2{class:"text-sm font-bold tracking-wide","Latest Transactions"}
                            a{href:"#",class:"px-4 py-2 text-xs bg-blue-100 text-blue-500 rounded uppercase tracking-wider font-semibold hover:bg-blue-300","More"}
                        }
                        ul{class:"divide-y-2 divide-gray-100 overflow-x-auto w-full",
                            li{class:"py-3 flex justify-between text-sm font-semibold",
                                p{class:"px-4 font-semibold","Today"}
                                p{class:"px-4 ","McDonald"}
                                p{class:"px-4 tracking-wider","Cash"}
                                p{class:"px-4 ", "Food"}
                                p{class:"md:text-base text-gray-800 flex items-center gap-2",
                                    "16.90"
                                    CustomizedSvg{s:6, d:"M19 9l-7 7-7-7"}
                                }
                            }
                        }
                        ul{class:"divide-y-2 divide-gray-100 overflow-x-auto w-full",
                            li{class:"py-3 flex justify-between text-sm font-semibold",
                                p{class:"px-4 font-semibold","Today"}
                                p{class:"px-4 ","McDonald"}
                                p{class:"px-4 tracking-wider","Cash"}
                                p{class:"px-4 ", "Food"}
                                p{class:"md:text-base text-gray-800 flex items-center gap-2",
                                    "16.90"
                                    CustomizedSvg{s:6, d:"M19 9l-7 7-7-7"}
                                }
                            }
                        }
                        ul{class:"divide-y-2 divide-gray-100 overflow-x-auto w-full",
                            li{class:"py-3 flex justify-between text-sm font-semibold",
                                p{class:"px-4 font-semibold","Today"}
                                p{class:"px-4 ","McDonald"}
                                p{class:"px-4 tracking-wider","Cash"}
                                p{class:"px-4 ", "Food"}
                                p{class:"md:text-base text-gray-800 flex items-center gap-2",
                                    "16.90"
                                    CustomizedSvg{s:6, d:"M19 9l-7 7-7-7"}
                                }
                            }
                        }
                        ul{class:"divide-y-2 divide-gray-100 overflow-x-auto w-full",
                            li{class:"py-3 flex justify-between text-sm font-semibold",
                                p{class:"px-4 font-semibold","Today"}
                                p{class:"px-4 ","McDonald"}
                                p{class:"px-4 tracking-wider","Cash"}
                                p{class:"px-4 ", "Food"}
                                p{class:"md:text-base text-gray-800 flex items-center gap-2",
                                    "16.90"
                                    CustomizedSvg{s:6, d:"M19 9l-7 7-7-7"}
                                }
                            }
                        }
                    }
                }

            },
        } 
    )
}