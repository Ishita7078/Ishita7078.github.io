#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/projects")]
    Projects {},
    #[route("/resume")]
    Resume {},
    #[route("/skills")]
    Skills {},
    #[route("/about")]
    About {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        header {
            style: "background-color: #b2c062; padding: 20px; margin: 0; display: flex; align-items: center; justify-content: space-between; font-family: Times New Roman, sans-serif; font-size: 20px; border: none; box-sizing: border-box;", // Adjust styles as needed
            h1 {
                style: "color: #fffdd0;", // Change the color of your name
                "Projects"
            }
            Link {
                to: Route::Home {},
                style: "background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "Home"
            }
        }
        body {
            style: "background-color: #c4cc7e; padding: 500px; margin: 0; font-family: Times New Roman, sans-serif; font-size: 20px; border: none; box-sizing: border-box;", // Adjust font size as needed
        }
    }
}

fn Resume() -> Element {
    rsx! {
        header {
            style: "background-color: #b2c062; padding: 20px; margin: 0; display: flex; align-items: center; justify-content: space-between; font-family: Times New Roman, sans-serif; font-size: 20px; border: none; box-sizing: border-box;", // Adjust styles as needed
            h1 {
                style: "color: #fffdd0;", // Change the color of your name
                "Resume"
            }
            Link {
                to: Route::Home {},
                style: "background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "Home"
            }
        }
        body {
            style: "background-color: #c4cc7e; padding: 500px; margin: 0; font-family: Times New Roman, sans-serif; font-size: 20px; border: none; box-sizing: border-box;", // Adjust font size as needed
        }
    }
}

fn Skills() -> Element {
    rsx! {
        header {
            style: "background-color: #b2c062; padding: 20px; margin: 0; display: flex; align-items: center; justify-content: space-between; font-family: Times New Roman, sans-serif; font-size: 20px; border: none; box-sizing: border-box;", // Adjust styles as needed
            h1 {
                style: "color: #fffdd0;", // Change the color of your name
                "Skills"
            }
            Link {
                to: Route::Home {},
                style: "background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "Home"
            }
        }
        body {
            style: "background-color: #c4cc7e; padding: 500px; margin: 0; font-family: Times New Roman, sans-serif; font-size: 20px; border: none; box-sizing: border-box;", // Adjust font size as needed
        }
    }
}

fn About() -> Element {
    rsx! {
        header {
            style: "background-color: #b2c062; padding: 20px; margin: 0; display: flex; align-items: center; justify-content: space-between; font-family: Times New Roman, sans-serif; font-size: 20px; border: none; box-sizing: border-box;", // Adjust styles as needed
            h1 {
                style: "color: #fffdd0; flex: 1 0 auto;", // Allow the text to wrap if necessary
                "Personal Pursuits"
            }
            Link {
                to: Route::Home {},
                style: "background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "Home"
            }
        }
        body {
            style: "background-color: #c4cc7e; padding: 500px; margin: 0; font-family: Times New Roman, sans-serif; font-size: 20px; border: none; box-sizing: border-box;", // Adjust font size as needed
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        header {
            style: "background-color: #b2c062; padding: 20px; margin: 0; display: flex; align-items: center; font-family: Times New Roman, sans-serif; font-size: 20px; border: none; box-sizing: border-box;", // Adjust styles as needed
            h1 {
                style: "color: #fffdd0;", // Change the color of your name
                "Ishita Mehta"
            }
            Link {
                to: Route::Projects {},
                style: "margin-left: 60px; background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "Projects"
            }
            Link {
                to: Route::Resume {},
                style: "margin-left: 60px; background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "Resume"
            }
            Link {
                to: Route::Skills {},
                style: "margin-left: 60px; background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "Skills"
            }
            Link {
                to: "https://www.linkedin.com/in/-ishita-mehta/",
                style: "margin-left: 60px; background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "LinkedIn"
            }
            Link {
                to: "https://github.com/Ishita7078",
                style: "margin-left: 60px; background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "GitHub"
            }
            Link {
                to: Route::About {},
                style: "margin-left: 60px; background-color: #acaf61; color: #fffdd0; padding: 20px 25px; text-decoration: none; border-radius: 5px; font-size: 20px;",
                "Personal Pursuits"
            }
        }

        main {
            style: "background-color: #c4cc7e; padding: 20px; margin: 0; font-family: Times New Roman, sans-serif; font-size: 22px; border: none; box-sizing: border-box; display: flex;",
            div {
                style: "width: 60%; padding-right: 20px; color: #fffdd0;", // Adjust the width and spacing as needed
                h2 { "About Me" }
                p {
                    "I am currently an undergraduate student at the University of Colorado Boulder."                
                }
                p {
                    "In my free time I like to bake, read, and crochet!"
                }
     
            }
            div{
                style: "width: 40%; padding-right: 20px; color: #fffdd0;", // Adjust the width and spacing as needed
                img {
                    src: "https://github.com/Ishita7078/image-personal-website/blob/main/IMG_6843.jpeg?raw=true",
                    style: "max-width: 100%; max-height: 100%;", // Ensure the image fits within its container
                    alt: "Description of the image",
                }
            }
        }
    }
}
