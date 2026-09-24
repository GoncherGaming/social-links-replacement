use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Platform{
    id: usize,
    platform_name: String,
    destination_url: String,
    icon: Asset,
    brand_color: String,
}

fn main(){
    dioxus::launch(App);
}

#[component]
fn profile_header() -> Element{
    rsx!{
        div{
            class: "flex flex-col items-center",
            img{
                class: "w-45 h-45 rounded-full object-cover mb-4",
                src: asset!("/assets/PFP.png")
            }
            p{
                class: "text-2xl font-bold text-white tracking-normal text-center mb-4",
                "@GoncherGaming"
            }
            p{
                class: "text-base font-bold text-white text-center leading-snug max-w-xl mb-4",
                "Just a streamer tryna create an inclusive and safe community!"
            }
        }
    }
}

#[component]
fn link_card() -> Element{
    let platforms = use_signal(|| vec![
        Platform{ id: 1, platform_name: "X".to_string(), destination_url: "https://twitter.com/GoncherGaming".to_string(), icon: asset!("/assets/X.png"), brand_color: "#1da1f2".to_string()},
        Platform{ id: 2, platform_name: "Bluesky".to_string(), destination_url: "https://bsky.app/profile/gonchergaming.bsky.social".to_string(), icon: asset!("/assets/Bluesky.png"), brand_color: "#0085ff".to_string()},
        Platform{ id: 3, platform_name: "Twitch".to_string(), destination_url: "https://twitch.tv/GoncherGaming".to_string(), icon: asset!("/assets/Twitch.png"), brand_color: "#9146ff".to_string()},
        Platform{ id: 4, platform_name: "YouTube".to_string(), destination_url: "https://youtube.com/@GoncherGaming".to_string(), icon: asset!("/assets/Youtube.png"), brand_color: "#ff0000".to_string()},
        Platform{ id: 5, platform_name: "TikTok".to_string(), destination_url: "https://tiktok.com/@GoncherGaming".to_string(), icon: asset!("/assets/TikTok.png"), brand_color: "#ff0050".to_string()},
        Platform{ id: 6, platform_name: "Instagram".to_string(), destination_url: "https://instagram.com/GoncherGamingTV".to_string(), icon: asset!("/assets/Instagram.png"), brand_color: "#e1306c".to_string()},
    ]);

    rsx!{
        div{
            for platform in platforms(){
                a{
                    href: platform.destination_url,
                    target: "_blank",

                    div{
                        style: "--hover-bg: {platform.brand_color}",
                        class: "grid grid-cols-[3rem_1fr_3rem] items-center w-full max-w-md min-h-[64px] rounded-xl bg-[#1d1d1d] px-4 py-4 text-white transition-all duration-200 hover:bg-[var(--hover-bg)] hover:border-[var(--hover-bg)] hover:scale-[1.02] shadow-md cursor-pointer mb-3.75",
                        img{
                            class: "w-6 h-6 justify-self-start object-contain filter invert",
                            src: platform.icon,
                        }
                        p{
                            class: "text-base font-bold text-white text-center col-start-2",
                            "{platform.platform_name}"
                        }
                        div{}
                    }
                }
            }
        }
    }
}

#[component]
fn App() -> Element{
    rsx!{
        document::Stylesheet{href: asset!("/assets/tailwind.css")}
        div{
            class: "h-screen w-full bg-[#3d4246] flex justify-center items-center py-10 px-4 overflow-hidden",
            div{
                class: "w-full max-w-md bg-black rounded-3xl p-8 flex flex-col items-center shadow-2xl space-y-6",
                profile_header{}
                link_card{}
            }
        }
    }
}