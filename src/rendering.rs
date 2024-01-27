use yew::prelude::*;

use crate::model::Slide;

pub trait RenderableAsHtml {
    fn render(s: &Self) -> Html;
}

impl RenderableAsHtml for Slide<'_> {
    fn render(s: &Self) -> Html {
        match s {
            Slide::Title { major, minor } => {
                html! {
                    <div>
                        <h1>{ major }</h1>
                            if minor.is_some() {                         
                                <h2>{minor.unwrap()}</h2>
                            }
                    </div>
                }
            }
            _ => html! {
                {"Hello world" }
            },
        }
    }
}
