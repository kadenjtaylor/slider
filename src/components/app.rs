use std::collections::HashMap;

use gloo::file::callbacks::FileReader;
use gloo::file::File;
use js_sys::encode_uri_component;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html};

use crate::components::slideshow::Slideshow;
use crate::model::{
    IdentifyPicture, Metadata, Round, RoundContent, SongArtistPair, TriviaGame, TriviaQuestion,
};

struct FileDetails {
    name: String,
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(String, Vec<u8>),
    Files(Vec<File>),
}

pub struct App {
    readers: HashMap<String, FileReader>,
    file: Option<FileDetails>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            file: Option::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, data) => {
                self.file = Some(FileDetails {
                    data,
                    name: file_name.clone(),
                });
                self.readers.remove(&file_name);
                true
            }
            Msg::Files(files) => {
                if let Some(file) = files.last() {
                    let file_name = file.name();

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.file {
            None => Self::upload_page(ctx),
            Some(file) => Self::render_slideshow(file),
        }
    }
}

impl App {
    fn upload_page(ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <div id="wrapper">
                    <h2>{ "Upload Your Questions File" }</h2>
                    <label for="file-upload">
                        <div
                            id="drop-container"
                            ondrop={ctx.link().callback(|event: DragEvent| {
                                event.prevent_default();
                                let files = event.data_transfer().unwrap().files();
                                Self::upload_files(files)
                            })}
                            ondragover={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
                            ondragenter={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
                        >
                            <i class="fa fa-cloud-upload"></i>
                            <p>{"Drop your file here or click to select"}</p>
                        </div>
                    </label>
                    <input
                        id="file-upload"
                        type="file"
                        accept="text/*, .json"
                        multiple={false}
                        onchange={ctx.link().callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Self::upload_files(input.files())
                        })}
                    />
                    { Self::demo_download() }
                </div>
            </main>
        }
    }

    fn demo_download() -> Html {
        let dummy_logo_url = "https://dynamic.brandcrowd.com/asset/logo/6d930c0b-9c8a-485a-b0e4-629a18c937b2/logo-search-grid-1x?logoTemplateVersion=2&v=638308718659730000";
        let demo_game = TriviaGame {
            metadata: Metadata {
                title: "Awesome Trivia Slideshow".to_string(),
                presenter: Some("Kaden Taylor".to_string()),
                rules: vec!["4 people per team maximum".to_string(), "no phones".to_string(), "no yelling answers".to_string()],
                prizes: vec!["1st - BIG plastic trophy".to_string(), "2nd - smaller plastic trohpy".to_string()],
                logo_image_source: "https://dynamic.brandcrowd.com/asset/logo/6d930c0b-9c8a-485a-b0e4-629a18c937b2/logo-search-grid-1x?logoTemplateVersion=2&v=638308718659730000".to_string(),
                breaks_after: vec![2, 4],
                break_duration_minutes: 5
            },
            rounds: vec![
                Round {
                    title: "Current Events".to_string(),
                    rules: vec![],
                    content: RoundContent::Questions(vec![
                        TriviaQuestion::FillInBlank {
                            before: "Did you hear? ".to_string(),
                            blank: "EVERYONE".to_string(),
                            after: " is getting married!".to_string(),
                        },
                        TriviaQuestion::QAndA {
                            question: "Who is having kids?"
                                .to_string(),
                            answer: "EVERYONE".to_string(),
                        },
                    ]),
                },
                Round {
                    title: "𝆕 Name that Tune 𝆕".to_string(),
                    rules: vec![
                        "One point per song".to_string(),
                        "One point for an artist listed on the track".to_string(),
                        "No additional points for additional artists".to_string(),
                        "NO PHONES!".to_string()
                    ],
                    content: RoundContent::Songs(vec![
                        SongArtistPair{
                            song: "Spinning".to_string(),
                            artist: "Jack's Mannequin".to_string()
                        },
                        SongArtistPair {
                            song: "Piano Man".to_string(),
                            artist: "Billy Joel".to_string()
                        }
                    ]),
                },
                Round {
                    title: "Random".to_string(),
                    rules: vec![],
                    content: RoundContent::Questions(vec![
                        TriviaQuestion::QAndA {
                            question: "Are ya ready, kids?!?"
                                .to_string(),
                            answer: "Aye Aye, Captain!".to_string(),
                        },
                    ]),
                },
                Round {
                    title: "📷 Guess That Logo 📷".to_string(),
                    rules: vec!["You will have 10 minutes".to_string()],
                    content: RoundContent::Pictures(crate::model::PictureGrid::ThreeByFive {
                        pics: [
                            dummy_logo_url, dummy_logo_url, dummy_logo_url, dummy_logo_url, dummy_logo_url, dummy_logo_url,
                            dummy_logo_url, dummy_logo_url, dummy_logo_url, dummy_logo_url, dummy_logo_url, dummy_logo_url,
                            dummy_logo_url, dummy_logo_url, dummy_logo_url,
                        ]
                        .map(|img| IdentifyPicture {
                            answer: "Logo Answer".to_string(),
                            source: img.to_string(),
                        }),
                    }),
                },
                Round {
                    title: "Blue Things".to_string(),
                    rules: vec![],
                    content: RoundContent::Questions(vec![
                        TriviaQuestion::QAndA {
                            question: "Which actor was painted blue in the movie Big Fat Liar?"
                                .to_string(),
                            answer: "Paul Giamatti".to_string(),
                        },
                    ]),
                },
            ]
        };
        let demo_json = serde_json::to_string_pretty(&demo_game).unwrap();
        html! {
            <div>
                <a href={format!("data:text/json;charset=utf-8,{}", encode_uri_component(&demo_json))} download={"questions.json"}>{"Download Sample Questions"}</a>
            </div>
        }
    }

    fn render_slideshow(file: &FileDetails) -> Html {
        let my_data = file.data.clone();
        let file_str = String::from_utf8(my_data).unwrap();
        match serde_json::from_str::<TriviaGame>(&file_str) {
            Ok(game) => {
                html! {
                    <Slideshow rounds={ game.rounds } metadata = {game.metadata}></Slideshow>
                }
            }
            Err(msg) => html! {
                <main>
                    <div id="wrapper">
                        <div id="error-container">
                            <h2> { "Error parsing file:" }</h2>
                            <p>{file.name.clone()}</p>
                            <p>{msg}</p>
                        </div>
                    </div>
                </main>
            },
        }
    }

    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let last_file = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from)
                .last()
                .unwrap();
            result = vec![last_file];
        }
        Msg::Files(result)
    }
}
