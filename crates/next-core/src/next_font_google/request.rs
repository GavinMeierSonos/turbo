use serde::Deserialize;

/// The top-most structure encoded into the query param in requests to
/// `next/font/google` generated by the next/font swc transform. e.g.
/// `next/font/google/target.css?{"path": "index.js", "import": "Inter"...`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextFontRequest {
    pub path: String,
    pub import: String,
    pub arguments: Vec<NextFontRequestArguments>,
    pub variable_name: String,
}

#[derive(Debug, Deserialize)]
pub struct NextFontRequestArguments {
    pub weight: Option<OneOrManyStrings>,
    pub subsets: Option<Vec<String>>,
    pub style: Option<OneOrManyStrings>,
    pub display: Option<String>,
    #[serde(default)]
    pub preload: bool,
    pub axes: Option<Vec<String>>,
    pub fallback: Option<Vec<String>>,
    #[serde(default)]
    pub adjust_font_fallback: bool,
    pub variable: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OneOrManyStrings {
    One(String),
    Many(Vec<String>),
}
