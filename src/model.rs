use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Deserialize, Serialize, PartialEq, Properties)]
pub struct TriviaGame {
    pub rounds: Vec<Round>,
    pub metadata: Metadata,
}

#[derive(Deserialize, Serialize, PartialEq, Properties)]
pub struct Metadata {
    pub rules: Vec<String>,
    pub prizes: Vec<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Properties)]
pub struct Round {
    pub title: String,
    pub rules: Vec<String>,
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
    Bullets {
        title: String,
        bullets: Vec<String>,
    },
    Question(TriviaQuestion),
    Reveal(TriviaQuestion),
}
