pub struct FillInBlank<'a> {
    before: &'a str,
    blank: &'a str,
    after: &'a str,
}

pub enum TriviaQuestion<'a> {
    QAndA { question: &'a str, answer: &'a str },
    FillInBlank(FillInBlank<'a>),
}

pub enum Slide<'a> {
    Title {
        major: &'a str,
        minor: Option<&'a str>,
    },
    Question(TriviaQuestion<'a>),
    Reveal(TriviaQuestion<'a>),
}
