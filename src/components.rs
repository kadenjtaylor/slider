use yew::prelude::*;

use crate::model::Slide;
use crate::rendering::RenderableAsHtml;

#[function_component]
pub fn Slideshow() -> Html {
    let slides: Vec<Slide> = vec![
        Slide::Title {
            major: "Slideshow",
            minor: Some("by Kaden Taylor"),
        },
        Slide::Title {
            major: "Still a Slideshow",
            minor: Some("still by Kaden Taylor"),
        },
        Slide::Title {
            major: "A VERY BORING SLIDESHOW",
            minor: None,
        },
        Slide::Title {
            major: "The END",
            minor: None,
        },
    ];
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        let max = slides.len();
        log::debug!("Clicked: {:?}", counter);
        move |_| {
            let value = *counter + 1;
            if value < max {
                counter.set(value);
            } else {
                log::debug!("Refusing to increment past maximum.");
            }
        }
    };

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
