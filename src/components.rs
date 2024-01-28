use yew::prelude::*;

use crate::model::{Slide, TriviaQuestion};
use crate::rendering::RenderableAsHtml;

#[function_component]
pub fn Slideshow() -> Html {
    let questions: Vec<TriviaQuestion> = vec![
        TriviaQuestion::FillInBlank {
            before: "I can barely hear myself",
            blank: " think ",
            after: "in here!",
        },
        TriviaQuestion::QAndA {
            question: "What do you call the cross between a skunk, and angel, and an owl?",
            answer: "An animal that stinks to high heaven and doesn't give a hoot.",
        },
    ];
    let slides: Vec<Slide> = vec![
        Slide::Title {
            major: "Slideshow",
            minor: Some("by Kaden Taylor"),
        },
        Slide::Question(&questions[0]),
        Slide::Reveal(&questions[0]),
        Slide::Question(&questions[1]),
        Slide::Reveal(&questions[1]),
        Slide::Title {
            major: "A VERY BORING SLIDESHOW",
            minor: None,
        },
        Slide::Title {
            major: "The END",
            minor: None,
        },
    ];
    let counter: UseStateHandle<usize> = use_state(|| 0);
    let num_slides = slides.len();

    let onclick = {
        let counter = counter.clone();
        log::debug!("Clicked: {:?}", counter);
        move |_| {
            let value = *counter + 1;
            if value < num_slides {
                counter.set(value);
            } else {
                log::debug!("Refusing to increment past maximum.");
            }
        }
    };

    // TODO: This doesn't work yet - figure that out
    let onkeypress = {
        move |event: KeyboardEvent| {
            log::debug!("{:?}", event);
        }
    };

    html! {
        <main {onclick} onkeypress={onkeypress}>
            { RenderableAsHtml::render(slides.get(*counter).unwrap()) }
        </main>
    }
}
