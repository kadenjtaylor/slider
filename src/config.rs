use yew::Properties;

use crate::model::{Slide, TriviaQuestion};

#[derive(Debug, PartialEq, Properties)]
pub struct Config {
    pub slides: Vec<Slide>,
}

pub const SAMPLE_JSON: &str = r#"[
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

pub fn to_slides(questions: Vec<TriviaQuestion>) -> Vec<Slide> {
    let mut slides = vec![Slide::Title {
        major: "Slideshow Program".to_string(),
        minor: Some("by Kaden Taylor".to_string()),
    }];
    for q in questions {
        slides.push(Slide::Question(q.clone()));
        slides.push(Slide::Reveal(q));
    }
    slides.push(Slide::Title {
        major: "I hope you've enjoyed".to_string(),
        minor: Some("my very minimalist slideshow".to_string()),
    });
    slides.push(Slide::Title {
        major: "The END".to_string(),
        minor: Some("https://github.com/kadenjtaylor/slider".to_string()),
    });
    slides
}
