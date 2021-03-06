use serde::export::Formatter;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::io::Write;
use std::str::FromStr;

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
            s => Err(ParseOutputFormatError(s.to_string())),
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
    W: Write,
{
    pub format: OutputFormat,
    writer: W,
}

impl<W> Output<W>
where
    W: Write,
{
    pub fn new(format: OutputFormat, writer: W) -> Output<W> {
        Output { format, writer }
    }

    pub fn write<T: Serialize>(&mut self, value: T) -> Result<(), OutputError> {
        match self.format {
            OutputFormat::Json => self.json(&value),
            OutputFormat::Yaml => self.yaml(&value),
            OutputFormat::Toml => self.toml(&value),
        }?;
        self.writer.flush().map_err(|_| OutputError {})
    }

    fn json<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), OutputError> {
        serde_json::to_writer(&mut self.writer, value).map_err(|_| OutputError {})
    }

    fn yaml<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), OutputError> {
        serde_yaml::to_writer(&mut self.writer, &value).map_err(|_| OutputError {})
    }

    fn toml<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), OutputError> {
        let s = toml::to_string(value).map_err(|_| OutputError {})?;

        write!(self.writer, "{}", s).map_err(|_| OutputError {})
    }
}

#[cfg(test)]
impl Output<Vec<u8>> {
    pub fn into_vec(self) -> Vec<u8> {
        self.writer
    }
}

#[derive(Debug)]
pub struct OutputError {}

impl Error for OutputError {}

impl Display for OutputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt("unexpected output error", f)
    }
}
