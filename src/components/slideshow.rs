use gloo::events::EventListener;

use yew::prelude::*;

use crate::{
    model::{RoundContent, Slide, TriviaGame},
    rendering::RenderableAsHtml,
};
use wasm_bindgen::JsCast;

#[function_component]
pub fn Slideshow(game: &TriviaGame) -> Html {
    let slides = to_slides(game);
    let num_slides = slides.len();
    let counter: UseStateHandle<usize> = use_state(|| 0);

    let onclick = {
        let clone_counter = counter.clone();
        move |_: MouseEvent| {
            let value = *clone_counter + 1;
            if value < num_slides {
                clone_counter.set(value);
            } else {
                log::debug!("Refusing to increment past maximum.");
            }
        }
    };

    let onkeydown = {
        let clone_counter = counter.clone();
        move |event: &Event| {
            let event: &KeyboardEvent = event.dyn_ref::<KeyboardEvent>().unwrap();
            match event.key().as_str() {
                " " | "ArrowRight" => {
                    let value = *clone_counter + 1;
                    if value < num_slides {
                        clone_counter.set(value);
                    } else {
                        log::debug!("Refusing to increment past maximum.");
                    }
                }
                "ArrowLeft" => {
                    if *clone_counter >= 1 {
                        clone_counter.set(*clone_counter - 1);
                    } else {
                        log::debug!("Refusing to increment past minimum.");
                    }
                }
                key => log::debug!("Ignoring keypress: {}", key),
            }
        }
    };

    use_effect(move || {
        // Attach a keydown event listener to the document.
        let document = gloo::utils::document();
        let listener = EventListener::new(&document, "keydown", onkeydown);
        // Called when the component is unmounted.  The closure has to hold on to `listener`, because if it gets
        // dropped, `gloo` detaches it from the DOM. So it's important to do _something_, even if it's just dropping it.
        || drop(listener)
    });

    html! {
        <main {onclick}>
            { RenderableAsHtml::render(slides.get(*counter).unwrap()) }
        </main>
    }
}

fn to_slides(game: &TriviaGame) -> Vec<Slide> {
    let mut slides = vec![
        Slide::Title {
            major: game.metadata.title.to_string(),
            minor: game
                .metadata
                .presenter
                .clone()
                .map(|name| format!("Presented by {}", name)),
        },
        Slide::Bullets {
            title: "Rules".to_string(),
            bullets: game.metadata.rules.clone(),
        },
    ];
    for r in game.rounds.iter() {
        slides.push(Slide::Title {
            major: r.title.to_string(),
            minor: None,
        });
        slides.push(Slide::Bullets {
            title: "Rules".to_string(),
            bullets: r.rules.iter().map(|s| s.to_string()).collect(),
        });
        let mut questions: Vec<Slide> = vec![];
        let mut answers: Vec<Slide> = vec![];
        match &r.content {
            RoundContent::Questions(qs) => {
                for (i, q) in qs.iter().enumerate() {
                    questions.push(Slide::Question(i + 1, q.clone()));
                    answers.push(Slide::Reveal(i + 1, q.clone()));
                }
            }
            RoundContent::Pictures(pics) => {
                questions.push(Slide::PictureQuestion(pics.clone()));
                answers.push(Slide::PictureReveal(pics.clone()));
            }
        }
        slides.extend(questions);
        slides.push(Slide::Title {
            major: "Grading Time".to_string(),
            minor: None,
        });
        slides.extend(answers);
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
