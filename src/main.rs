use components::Slideshow;
use config::Config;
use model::{Slide, TriviaQuestion};
use yew::props;

mod components;
mod config;
mod model;
mod rendering;

fn main() {
    // Init application logging
    wasm_logger::init(wasm_logger::Config::default());

    let cfg = props!(Config { slides: slides() });

    yew::Renderer::<Slideshow>::with_props(cfg).render();
}

fn slides() -> Vec<Slide<'static>> {
    let questions: Vec<TriviaQuestion> = vec![
        TriviaQuestion::FillInBlank {
            before: "I can barely hear myself",
            blank: " think ",
            after: "in here!",
        },
        TriviaQuestion::QAndA {
            question: "What do you call the cross between a skunk, and angel, and an owl?",
            answer: "An animal that stinks to high heaven and doesn't give a hoot.",
        },
    ];
    let slides: Vec<Slide> = vec![
        Slide::Title {
            major: "Slideshow",
            minor: Some("by Kaden Taylor"),
        },
        Slide::Question(questions[0]),
        Slide::Reveal(questions[0]),
        Slide::Question(questions[1]),
        Slide::Reveal(questions[1]),
        Slide::Title {
            major: "A VERY BORING SLIDESHOW",
            minor: None,
        },
        Slide::Title {
            major: "The END",
            minor: None,
        },
    ];
    slides
}
