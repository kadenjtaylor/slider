use std::iter;

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
                                <ol>
                                    { for bullets.iter().map(render_item) }
                                </ol>
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
                        <div>
                            <h1 style="font-size: 8vw; margin:auto">{title}</h1>
                            <div style="display:flex; align-items: center; width 100%">
                                <div style="width: 45%; float: left;">
                                    <img src={image_source.to_string()} style="width: 100%; "/>
                                </div>
                                <div style="text-align: left; font-size: 3vw">
                                    {"Categories:"}
                                    <ul style="padding-left: 4vw">
                                        { for categories.iter().map(render_item) }
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </slide>
                }
            }
            Slide::Outro {
                major,
                minor,
                left_source,
                right_source,
            } => {
                html! {
                    <slide>
                        <div style="width:100%; max-height:90%">
                            <h1 style="font-size: 5vw; margin:auto">{major}</h1>
                            if let Some(msg) = minor {
                                <h1 style="font-size: 3.5vw; margin:auto">{msg}</h1>
                            }
                            <div style="display:flex; align-items: center; justify-content: center; padding-top: 5%">
                                <div style="width: 45%; float: left;">
                                    <img src={left_source.to_string()} width="80%" style="border-radius:20px;"/>
                                </div>
                                <div style="width: 50vh;">
                                    <img src={right_source.to_string()} width="80%" style="border-radius:20px;"/>
                                </div>
                            </div>
                        </div>
                    </slide>
                }
            }
            Slide::SongsReveal {
                title,
                left_block,
                right_block,
            } => {
                html! {
                    <slide>
                        <div>
                            <h1 style="margin:auto">{ title }</h1>
                            <div  style="display:flex; align-items: center; justify-content: center;">
                                <songs style="width:50%; margin-right:3%">
                                    <ol>
                                        { for left_block.iter().map(render_item) }
                                    </ol>
                                </songs>
                                <songs style="margin-left:3%">
                                    <ol start={format!("{}", left_block.len() + 1)}>
                                        { for right_block.iter().map(render_item) }
                                    </ol>
                                </songs>
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
