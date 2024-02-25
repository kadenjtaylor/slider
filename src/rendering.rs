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
                    <slide>
                        <div>
                            <h1>{ major }</h1>
                                if minor.is_some() {
                                    <h2>{minor.clone().unwrap()}</h2>
                                }
                        </div>
                    </slide>
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
                    <slide>
                        <h2>
                            { format!("{}.  {} ", num, before) }<blank>{ spaces }</blank>{ format!(" {}", after) }
                        </h2>
                    </slide>
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
                    <slide>
                        <h2>
                            { format!("{}.  {} ", num, before) }<blank>{ blank }</blank>{ format!(" {}", after) }
                        </h2>
                    </slide>
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
                    <slide>
                        <h2>
                        { format!("{}.  {}", num, question) }
                        </h2>
                    </slide>
                }
            }
            Slide::Reveal(num, TriviaQuestion::QAndA { question, answer }) => {
                html! {
                    <slide>
                        <div>
                        <h2>
                            { format!("{}.  {}", num, question) }
                        </h2>
                        <h2><answer>{ answer }</answer></h2>
                        </div>
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
            Slide::Preview {
                title,
                image_source,
                categories,
            } => {
                html! {
                    <slide>
                        <div style="width:100%; max-height:90%">
                            <h1 style="font-size: 7vw; margin:auto">{title}</h1>
                            <div style="display:flex; align-items: center">
                                <div style="width: 50%; float: left;">
                                    <img src={image_source.to_string()} width="80%"/>
                                </div>
                                <div style="text-align: left; padding-left:3%; font-size: 4vw">
                                    <ul>
                                        { for categories.iter().map(render_item) }
                                    </ul>
                                </div>
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
                    <image_number>{format!("{}", i+1)}</image_number>
                    if reveal {
                        <overlay_text>{format!("{}", ip.answer)}</overlay_text>
                    }
                </image_container>
            }) }
        </div>
    }
}
