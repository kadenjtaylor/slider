use yew::prelude::*;

use crate::config::Config;
use crate::rendering::RenderableAsHtml;

#[function_component]
pub fn Slideshow(cfg: &Config) -> Html {
    let counter: UseStateHandle<usize> = use_state(|| 0);
    let num_slides = cfg.slides.len();

    let onclick = {
        let counter = counter.clone();
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
            { RenderableAsHtml::render(cfg.slides.get(*counter).unwrap()) }
        </main>
    }
}
