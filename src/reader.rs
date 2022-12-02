use crate::string_utils::fill;
use std::fmt::Display;
use std::fs::File;
use std::io::{Error, Read};

#[derive(Debug)]
pub enum ReaderResultError {
    RowLongerThanHeader(String),
}

impl Display for ReaderResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ReaderResultError::RowLongerThanHeader(error) => write!(f, "{}", error),
        }
    }
}

#[derive(Debug)]
pub enum ReaderError {
    IOERROR(Error),
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pre_error = "An error occured:";

        match &self {
            ReaderError::IOERROR(error) => write!(f, "{pre_error} {error}"),
        }
    }
}

type ReaderResultType = Vec<Vec<String>>;

#[derive(Debug)]
pub struct ReaderResult {
    data: ReaderResultType,
}

impl ReaderResult {
    pub fn display(&self) -> Result<(), ReaderResultError> {
        let mut sizes: Vec<usize> = vec![];

        // Max lengths
        let data_iter_len = &mut self.data.iter();
        let headers_len;

        if let Some(headers_data) = data_iter_len.next() {
            headers_len = headers_data.len();

            for field in headers_data {
                sizes.push(field.len());
            }
        } else {
            return Ok(());
        }

        for row in data_iter_len {
            //TODO: Move into proper validator
            let row_len = row.len();
            if row_len > headers_len {
                return Err(ReaderResultError::RowLongerThanHeader(format!(
                    "headers length: {headers_len}, rows length: {row_len}"
                )));
            }

            for (i, field) in row.iter().enumerate() {
                if field.len() > sizes[i] {
                    sizes[i] = field.len();
                }
            }
        }

        //Body section
        let data_iter = &mut self.data.iter();

        if let Some(headers_data) = data_iter.next() {
            let mut header_top_line = String::new();
            let mut header_middle = String::from("|");
            let mut header_bottom_line = String::new();

            for (i, field) in headers_data.iter().enumerate() {
                let mut field_cp = String::from(field);
                fill(&mut field_cp, ' ', sizes[i]);

                header_middle.push_str(&format!(" {field_cp} |"))
            }

            for _ in 0..header_middle.len() {
                header_top_line.push('-');
                header_bottom_line.push('-');
            }
            println!("{header_top_line}\n{header_middle}\n{header_bottom_line}");
        } else {
            return Ok(());
        }

        for row in data_iter {
            let mut row_middle = String::from("|");
            let mut row_bottom_line = String::new();

            for (i, field) in row.iter().enumerate() {
                let mut field_cp = String::from(field);
                fill(&mut field_cp, ' ', sizes[i]);

                row_middle.push_str(&format!(" {field_cp} |"))
            }

            for _ in 0..row_middle.len() {
                row_bottom_line.push('-');
            }

            println!("{row_middle}\n{row_bottom_line}");
        }

        Ok(())
    }
}

pub struct CSVReader {
    filename: &'static str,
}

impl CSVReader {
    pub fn new(filename: &'static str) -> Self {
        Self { filename }
    }

    pub fn read(&self) -> Result<ReaderResult, ReaderError> {
        let mut file_result = String::new();

        let file = File::open(self.filename);

        match file {
            Ok(mut file) => match file.read_to_string(&mut file_result) {
                Ok(_) => {
                    let data = file_result
                        .split("\n")
                        .filter(|row| row.trim().len() != 0)
                        .map(|row| {
                            row.split(",")
                                .map(|row_value| row_value.to_string())
                                .collect::<Vec<String>>()
                        })
                        .collect::<ReaderResultType>();
                    Ok(ReaderResult { data })
                }

                Err(error_content) => Err(ReaderError::IOERROR(error_content)),
            },

            Err(error_content) => Err(ReaderError::IOERROR(error_content)),
        }
    }
}
