use components::Slideshow;
use config::Config;
use model::{Slide, TriviaQuestion};
use yew::props;

mod components;
mod config;
mod model;
mod rendering;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let cfg = props!(Config { slides: slides() });
    yew::Renderer::<Slideshow>::with_props(cfg).render();
}

fn slides() -> Vec<Slide<'static>> {
    let questions: Vec<TriviaQuestion> = vec![
        TriviaQuestion::FillInBlank {
            before: "This is a",
            blank: " demo ",
            after: "program for creating slideshows!",
        },
        TriviaQuestion::FillInBlank {
            before: "It uses",
            blank: " Yew for Rust+WASM ",
            after: "which means it runs in the browser!",
        },
        TriviaQuestion::QAndA {
            question: "Why would I need something like this?",
            answer: "I regularly put together trivia shows, and this seems easier that futzing with Google Slides all the time",
        },
        TriviaQuestion::FillInBlank {
            before: "It came from the template",
            blank: " https://github.com/yewstack/yew-trunk-minimal-template ",
            after: "which means I didn't have to think about it as hard",
        },
    ];
    let slides: Vec<Slide> = vec![
        Slide::Title {
            major: "Slideshow Program",
            minor: Some("by Kaden Taylor"),
        },
        Slide::Question(questions[0]),
        Slide::Reveal(questions[0]),
        Slide::Question(questions[1]),
        Slide::Reveal(questions[1]),
        Slide::Question(questions[2]),
        Slide::Reveal(questions[2]),
        Slide::Question(questions[3]),
        Slide::Reveal(questions[3]),
        Slide::Title {
            major: "I hope you've enjoyed",
            minor: Some("my very minimalist slideshow"),
        },
        Slide::Title {
            major: "The END",
            minor: Some("https://github.com/kadenjtaylor/slider"),
        },
    ];
    slides
}
