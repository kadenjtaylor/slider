use std::iter;

use yew::prelude::*;

use crate::model::{Slide, TriviaQuestion};

pub trait RenderableAsHtml {
    fn render(s: &Self) -> Html;
}

impl RenderableAsHtml for Slide {
    fn render(s: &Self) -> Html {
        match s {
            Slide::Title { major, minor } => {
                html! {
                    <div>
                        <h1>{ major }</h1>
                            if minor.is_some() {
                                <h2>{minor.clone().unwrap()}</h2>
                            }
                    </div>
                }
            }
            Slide::Question(TriviaQuestion::FillInBlank {
                before,
                blank: _,
                after,
            }) => {
                let spaces: String = iter::repeat("_").take(11).collect();
                html! {
                    <slide>
                        <h2>{ before }</h2>
                        <h2><blank>{ spaces }</blank></h2>
                        <h2>{ after }</h2>
                    </slide>
                }
            }
            Slide::Reveal(TriviaQuestion::FillInBlank {
                before,
                blank,
                after,
            }) => {
                html! {
                    <slide>
                        <h2>{ before }</h2>
                        <h2><blank>{ blank }</blank></h2>
                        <h2>{ after }</h2>
                    </slide>
                }
            }
            Slide::Question(TriviaQuestion::QAndA {
                question,
                answer: _,
            }) => {
                html! {
                    <slide>
                        <h2>{ question }</h2>
                    </slide>
                }
            }
            Slide::Reveal(TriviaQuestion::QAndA { question, answer }) => {
                html! {
                    <slide>
                        <h2>{ question }</h2>
                        <h2>{ answer }</h2>
                    </slide>
                }
            }
        }
    }
}
