use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Deserialize, Serialize, PartialEq, Properties)]
pub struct TriviaGame {
    // TODO: Rules, prizes, metadata, etc.
    pub rounds: Vec<Round>,
}

#[derive(Deserialize, Serialize, PartialEq, Properties)]
pub struct Round {
    pub questions: Vec<TriviaQuestion>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TriviaQuestion {
    QAndA {
        question: String,
        answer: String,
    },
    FillInBlank {
        before: String,
        blank: String,
        after: String,
    },
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum Slide {
    Title {
        major: String,
        minor: Option<String>,
    },
    Question(TriviaQuestion),
    Reveal(TriviaQuestion),
}

pub fn to_slides(game: &TriviaGame) -> Vec<Slide> {
    let mut slides = vec![Slide::Title {
        major: "Slideshow Program".to_string(),
        minor: Some("by Kaden Taylor".to_string()),
    }];
    for q in game.rounds.iter().map(|r| r.questions.iter()).flatten() {
        slides.push(Slide::Question(q.clone()));
        slides.push(Slide::Reveal(q.clone()));
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
