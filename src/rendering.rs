use std::iter;

use gloo::console::console_dbg;
use yew::prelude::*;

use crate::model::{IdentifyPicture, PictureGrid, Slide, TriviaQuestion};

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
            Slide::Bullets { title, bullets } => {
                html! {
                    <slide>
                        <bullets>
                            <h1>{ title }</h1>
                                <ul>
                                    { for bullets.iter().map(render_item) }
                                </ul>
                        </bullets>
                    </slide>
                }
            }
            Slide::PictureQuestion(pics) => render_picture_grid(pics, false),
            Slide::PictureReveal(pics) => render_picture_grid(pics, true),
        }
    }
}

fn render_item(s: &String) -> Html {
    html!(<li>{ s }</li>)
}

// TODO: Add indexes, answers (only if reveal == true)
fn render_picture_grid(pics: &PictureGrid, reveal: bool) -> Html {
    let (columns, pics): (u32, Vec<IdentifyPicture>) = match pics {
        PictureGrid::FourByFour { pics } => (4, pics.to_vec()),
        PictureGrid::ThreeByFour { pics } => (4, pics.to_vec()),
        PictureGrid::ThreeByFive { pics } => (5, pics.to_vec()),
    };
    console_dbg!(columns);
    html! {
        <div class={"grid"} style={ format!("--columns:{}", columns) }>
            { for pics.iter().enumerate().map(|(i, ip)| html!{
                // <span>{format!("{}", i)}</span>
                <img src={ip.source.to_string()}/>
            }) }
        </div>
    }
}
