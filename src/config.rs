use yew::Properties;

use crate::model::Slide;

#[derive(Debug, PartialEq, Properties)]
pub struct Config {
    pub slides: Vec<Slide<'static>>,
}
