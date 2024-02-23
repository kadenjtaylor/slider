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
                    <centered_slide>
                        <div>
                            <h1>{ major }</h1>
                                if minor.is_some() {
                                    <h2>{minor.clone().unwrap()}</h2>
                                }
                        </div>
                    </centered_slide>
                }
            }
            Slide::Question(
                num,
                TriviaQuestion::FillInBlank {
                    before,
                    blank: _,
                    after,
                },
            ) => {
                let spaces: String = iter::repeat("_").take(11).collect();
                html! {
                    <centered_slide>
                        <h2>
                            { format!("{} ", before) }<blank>{ spaces }</blank>{ format!(" {}", after) }
                        </h2>
                    </centered_slide>
                }
            }
            Slide::Reveal(
                num,
                TriviaQuestion::FillInBlank {
                    before,
                    blank,
                    after,
                },
            ) => {
                html! {
                    <centered_slide>
                        <p>{num}</p>
                        <h2>
                            { format!("{} ", before) }<blank>{ blank }</blank>{ format!(" {}", after) }
                        </h2>
                    </centered_slide>
                }
            }
            Slide::Question(
                num,
                TriviaQuestion::QAndA {
                    question,
                    answer: _,
                },
            ) => {
                html! {
                    <centered_slide>
                        <p>{num}</p>
                        <h2>{ question }</h2>
                    </centered_slide>
                }
            }
            Slide::Reveal(num, TriviaQuestion::QAndA { question, answer }) => {
                html! {
                    <centered_slide>
                        <p>{num}</p>
                        <h2>{ question }</h2>
                        <h2>{ answer }</h2>
                    </centered_slide>
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
            Slide::Preview {
                title,
                image_source,
                categories,
            } => {
                html! {
                    <slide>
                        <h1>{title}</h1>
                        <div style="width: 100%;">
                            <div style="width: 50%; float: left;">
                                <img src={image_source.to_string()} width="800px"/>
                            </div>
                            <div style="margin-left: 50%;">
                                <ul>
                                    { for categories.iter().map(render_item) }
                                </ul>
                            </div>
                        </div>
                    </slide>
                }
            }
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
                <image_container>
                    <img src={ip.source.to_string()}/>
                    <question_number>{format!("{}", i+1)}</question_number>
                    if reveal {
                        <overlay_text>{format!("{}", ip.answer)}</overlay_text>
                    }
                </image_container>
            }) }
        </div>
    }
}
