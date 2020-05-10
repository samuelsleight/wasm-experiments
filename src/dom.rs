use std::fmt::{
    self,
    Display,
    Formatter
};

use web_sys::{
    Window,
    Document,
    HtmlCanvasElement
};

use wasm_bindgen::JsCast;

pub enum Error {
    NoWindow,
    NoDocument,
    NoElement(String),
    InvalidCanvasCast
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoWindow => write!(f, "No global window found in DOM"),
            Error::NoDocument => write!(f, "No document found in window"),
            Error::NoElement(ref id) => write!(f, "No element found with id {}", id),
            Error::InvalidCanvasCast => write!(f, "Failed converting element to cavase")
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn window() -> Result<Window> {
    web_sys::window().ok_or(Error::NoWindow)
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or(Error::NoDocument)
}

pub fn canvas(id: &str) -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|| Error::NoElement(id.to_string()))?
        .dyn_into()
        .map_err(|_| Error::InvalidCanvasCast)
}
