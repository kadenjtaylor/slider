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