pub(crate) mod plot;

use crate::helpers::myerror;
use crate::helpers::AppResult;
use clap::{Arg, Command, ValueHint};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Points(Vec<Point>);

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Point {
    x: f64,
    y: f64,
}

impl From<Points> for Vec<(f64, f64)> {
    fn from(points: Points) -> Vec<(f64, f64)> {
        points
            .0
            .into_iter()
            .map(|point| {
                let Point { x, y } = point;
                (x, y)
            })
            .collect()
    }
}

pub(crate) struct App {
    cli: Option<Command>,
    data: Option<Points>,
}

impl App {
    pub(crate) fn new() -> Self {
        let clap_command = Command::new(clap::crate_name!())
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about(clap::crate_description!())
            .arg(
                Arg::new("dataset-name")
                    .value_name("DATASET_NAME")
                    .value_hint(ValueHint::FilePath)
                    .env("DATASET_NAME")
                    .value_parser(clap::builder::ValueParser::new(dataset_file_parse_wrap))
                    .help("Dataset file .json file to load")
                    .required(true)
                    .num_args(1),
            );
        App { cli: Some(clap_command), data: None }
    }

    pub(crate) fn load(&mut self) -> AppResult<()> {
        use std::fs;
        let cli = self.take_cli().ok_or(myerror("CLI not defined"))?;

        let matches = cli.get_matches();

        let file_path =
            matches.get_one::<PathBuf>("dataset-name").ok_or(myerror("Argument not found"))?;

        // dbg!(file_path);

        let string_from_file = fs::read_to_string(file_path)?;
        let data: Points = serde_json::from_str(&string_from_file)?;
        self.data = Some(data);
        Ok(())
    }

    pub(crate) fn take_cli(&mut self) -> Option<Command> {
        self.cli.take()
    }
    pub(crate) fn take_data(&mut self) -> Option<Points> {
        self.data.take()
    }
}
/////////////
fn dataset_file_parse_wrap(filename: &str) -> Result<PathBuf, clap::Error> {
    let mut path: PathBuf = PathBuf::new();
    path.push(filename);
    path.set_extension("json");

    let file_path = path
        .exists()
        .then_some(path)
        .ok_or(clap::Error::raw(clap::error::ErrorKind::ValueValidation, "File does not exist"))?;

    Ok(file_path)
}
////////////
