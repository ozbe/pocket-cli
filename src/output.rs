use std::str::FromStr;
use std::fmt;
use std::fmt::{Display, Debug};
use serde::Serialize;
use std::error::Error;
use serde::export::Formatter;
use std::io::Write;

#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Json
    }
}

impl FromStr for OutputFormat {
    type Err = ParseOutputFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" | "yml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            s => Err(ParseOutputFormatError(s.to_string()))
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        };
        fmt::Display::fmt(display, f)
    }
}

#[derive(Debug)]
pub struct ParseOutputFormatError(String);

impl fmt::Display for ParseOutputFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "provided invalid output format: {}", self.0)
    }
}

pub struct Output<W>
    where
        W: Write
{
    pub format: OutputFormat,
    pub writer: W,
}

impl<W> Output<W>
    where
        W: Write
{
    pub fn new(format: OutputFormat, writer: W) -> Output<W> {
        Output {
            format,
            writer,
        }
    }

    pub fn write<T: Serialize>(&mut self, value: T) -> Result<(), OutputError> {
        match self.format {
            OutputFormat::Json => self.json(&value),
            OutputFormat::Yaml => self.yaml(&value),
            OutputFormat::Toml => self.toml(&value),
        }
    }

    fn json<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), OutputError> {
        serde_json::to_writer(&mut self.writer, value)
            .map_err(|_| OutputError {})
    }

    fn yaml<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), OutputError> {
        serde_yaml::to_writer(&mut self.writer, &value)
            .map_err(|_| OutputError {})
    }

    fn toml<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), OutputError> {
        let s = toml::to_string(value)
            .map_err(|_| OutputError {})?;

        write!(self.writer, "{}", s)
            .map_err(|_| OutputError {})
    }
}

#[derive(Debug)]
pub struct OutputError {}

impl Error for OutputError {}

impl Display for OutputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt( "unexpected output error", f)
    }
}