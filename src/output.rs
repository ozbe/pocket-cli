use std::str::FromStr;
use std::fmt;
use std::fmt::{Display, Debug};
use serde::Serialize;
use std::error::Error;
use serde::export::Formatter;

#[derive(Debug)]
pub enum OutputFormat {
    Json,
}

#[derive(Debug)]
pub struct ParseOutputFormatError(String);

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Json
    }
}

impl fmt::Display for ParseOutputFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "provided invalid output format: {}", self.0)
    }
}

impl FromStr for OutputFormat {
    type Err = ParseOutputFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            s => Err(ParseOutputFormatError(s.to_string()))
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            OutputFormat::Json => "json",
        };
        fmt::Display::fmt(display, f)
    }
}


pub trait Output {
    fn write(&mut self, value: impl Serialize + Debug) -> Result<(), OutputError>;
}

#[derive(Debug)]
pub struct OutputError {}

impl Error for OutputError {}

impl Display for OutputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt( "unexpected output error", f)
    }
}

pub struct JsonOutput<W>
    where
        W: std::io::Write
{
    serializer: serde_json::Serializer<W>,
}

impl<W> JsonOutput<W>
    where
        W: std::io::Write
{
    pub fn new(writer: W) -> JsonOutput<W> {
        JsonOutput {
            serializer: serde_json::Serializer::new(writer)
        }
    }
}

impl<W> Output for JsonOutput<W>
    where
        W: std::io::Write
{
    fn write(&mut self, value: impl Serialize + Debug) -> Result<(), OutputError> {
        value.serialize(&mut self.serializer)
            .map_err(|_| OutputError {})
    }
}