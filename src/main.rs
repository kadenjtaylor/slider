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
    let cfg = props!(Config {
        slides: slides(questions_from_json())
    });
    yew::Renderer::<Slideshow>::with_props(cfg).render();
}

fn questions_from_json() -> Vec<TriviaQuestion<'static>> {
    let data = r#"[
        {
          "FillInBlank": {
            "before": "This is a",
            "blank": " demo ",
            "after": "program for creating slideshows!"
          }
        },
        {
          "FillInBlank": {
            "before": "It uses",
            "blank": " Yew for Rust+WASM ",
            "after": "which means it runs in the browser!"
          }
        },
        {
          "QAndA": {
            "question": "Why would I need something like this?",
            "answer": "I regularly put together trivia shows, and this seems easier that futzing with Google Slides all the time"
          }
        },
        {
          "FillInBlank": {
            "before": "It came from the template",
            "blank": " https://github.com/yewstack/yew-trunk-minimal-template ",
            "after": "which means I didn't have to think about it as hard"
          }
        }
      ]"#;
    serde_json::from_str(data).unwrap()
}

fn slides(questions: Vec<TriviaQuestion<'static>>) -> Vec<Slide<'static>> {
    let mut slides = vec![Slide::Title {
        major: "Slideshow Program",
        minor: Some("by Kaden Taylor"),
    }];
    for q in questions {
        slides.push(Slide::Question(q));
        slides.push(Slide::Reveal(q));
    }
    slides.push(Slide::Title {
        major: "I hope you've enjoyed",
        minor: Some("my very minimalist slideshow"),
    });
    slides.push(Slide::Title {
        major: "The END",
        minor: Some("https://github.com/kadenjtaylor/slider"),
    });
    slides
}
