#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
pub enum Slide<'a> {
    Title {
        major: &'a str,
        minor: Option<&'a str>,
    },
    Question(TriviaQuestion<'a>),
    Reveal(TriviaQuestion<'a>),
}
