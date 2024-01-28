pub enum TriviaQuestion<'a> {
    QAndA {
        question: &'a str,
        answer: &'a str,
    },
    FillInBlank {
        before: &'a str,
        blank: &'a str,
        after: &'a str,
    },
}

pub enum Slide<'a> {
    Title {
        major: &'a str,
        minor: Option<&'a str>,
    },
    Question(&'a TriviaQuestion<'a>),
    Reveal(&'a TriviaQuestion<'a>),
}
