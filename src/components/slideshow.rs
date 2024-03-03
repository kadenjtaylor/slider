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
        Slide::Preview {
            title: game.metadata.title.to_string(),
            image_source: game.metadata.logo_image_source.clone(),
            categories: game.rounds.iter().map(|r| r.title.to_string()).collect(),
        },
        Slide::Bullets {
            title: "Game Rules".to_string(),
            bullets: game.metadata.rules.clone(),
        },
        Slide::Bullets {
            title: "Prizes".to_string(),
            bullets: game.metadata.prizes.clone(),
        },
    ];
    for (i, r) in game.rounds.iter().enumerate() {
        slides.push(Slide::Title {
            major: format!("Round {}:", i + 1),
            minor: Some(format!("{}", r.title)),
        });
        if !r.rules.is_empty() {
            slides.push(Slide::Bullets {
                title: "Round Rules".to_string(),
                bullets: r.rules.iter().map(|s| s.to_string()).collect(),
            });
        }
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
            RoundContent::Songs(songs) => {
                questions.push(Slide::Title {
                    major: "𝆕 AUDIO ONLY 𝆕".to_string(),
                    minor: Some("Listen carefully".to_string()),
                });
                let mut songs: Vec<String> = songs
                    .iter()
                    .map(|pair| format!("{} - {}", pair.song, pair.artist))
                    .collect();
                let split = songs.len() / 2;
                let right_half = songs.split_off(split);
                answers.push(Slide::SongsReveal {
                    title: r.title.to_string(),
                    left_block: songs,
                    right_block: right_half,
                })
            }
        }
        slides.extend(questions);
        slides.push(Slide::Title {
            major: "Grading Time!".to_string(),
            minor: None,
        });
        slides.extend(answers);
        if game
            .metadata
            .breaks_after
            .iter()
            .map(|i| i - 1)
            .collect::<Vec<usize>>()
            .contains(&i)
        {
            slides.push(Slide::Title {
                major: "Quick Break!".to_string(),
                minor: Some(format!(
                    "See you back in {} minutes",
                    game.metadata.break_duration_minutes
                )),
            });
        }
    }
    let kaden_venmo = include_str!("../resources/kaden_venmo.txt");
    let chelsea_venmo = include_str!("../resources/chelsea_venmo.txt");
    slides.push(Slide::Outro {
        major: "Thanks for playing!".to_string(),
        minor: Some("Here's how to tip your hosts!".to_string()),
        left_source: chelsea_venmo.to_string(),
        right_source: kaden_venmo.to_string(),
    });
    slides
}
