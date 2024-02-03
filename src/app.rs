use std::collections::HashMap;

use gloo::file::callbacks::FileReader;
use gloo::file::File;
use js_sys::encode_uri_component;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html};

use crate::model::{to_slides, TriviaQuestion};
use crate::slideshow::Slideshow;

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
            None => {
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
                            <div>
                                <a href={format!("data:text/json;charset=utf-8,{}", encode_uri_component(SAMPLE_JSON))} download={"questions.json"}>{"Download Sample Questions"}</a>
                            </div>
                        </div>
                    </main>
                }
            }
            Some(file) => {
                html! {
                    { Self::render_slideshow(file) }
                }
            }
        }
    }
}

impl App {
    fn render_slideshow(file: &FileDetails) -> Html {
        let my_data = file.data.clone();
        let file_str = String::from_utf8(my_data).unwrap();
        match serde_json::from_str::<Vec<TriviaQuestion>>(&file_str) {
            Ok(questions) => {
                html! {
                    <Slideshow slides={ to_slides(questions) }></Slideshow>
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

pub const SAMPLE_JSON: &str = r#"[
    {
      "FillInBlank": {
        "before": "This is a",
        "blank": " demo ",
        "after": "program for creating slideshows!"
      }
    },
    {
      "FillInBlank": {
        "before": "It uses",
        "blank": " Yew for Rust+WASM ",
        "after": "which means it runs in the browser!"
      }
    },
    {
      "QAndA": {
        "question": "Why would I need something like this?",
        "answer": "I regularly put together trivia shows, and this seems easier that futzing with Google Slides all the time"
      }
    },
    {
      "FillInBlank": {
        "before": "It came from the template",
        "blank": " https://github.com/yewstack/yew-trunk-minimal-template ",
        "after": "which means I didn't have to think about it as hard"
      }
    }
  ]"#;
