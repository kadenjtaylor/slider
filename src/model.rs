use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Deserialize, Serialize, PartialEq, Properties)]
pub struct TriviaGame {
    pub rounds: Vec<Round>,
    pub metadata: Metadata,
}

#[derive(Deserialize, Serialize, PartialEq, Properties)]
pub struct Metadata {
    pub title: String,
    pub presenter: Option<String>,
    pub logo_image_source: String,
    pub rules: Vec<String>,
    pub prizes: Vec<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Properties)]
pub struct Round {
    pub title: String,
    pub rules: Vec<String>,
    pub content: RoundContent,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RoundContent {
    Questions(Vec<TriviaQuestion>),
    Pictures(PictureGrid),
    Songs(Vec<SongArtistPair>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SongArtistPair {
    pub song: String,
    pub artist: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentifyPicture {
    pub answer: String,
    pub source: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum PictureGrid {
    FourByFour { pics: [IdentifyPicture; 16] },
    ThreeByFive { pics: [IdentifyPicture; 15] },
    ThreeByFour { pics: [IdentifyPicture; 12] },
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
    Preview {
        title: String,
        image_source: String,
        categories: Vec<String>,
    },
    Title {
        major: String,
        minor: Option<String>,
    },
    Bullets {
        title: String,
        bullets: Vec<String>,
    },
    Question(usize, TriviaQuestion),
    Reveal(usize, TriviaQuestion),
    PictureQuestion(PictureGrid),
    PictureReveal(PictureGrid),
}
