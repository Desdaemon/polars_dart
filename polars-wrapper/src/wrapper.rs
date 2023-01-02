use anyhow::Result;
use flutter_rust_bridge::*;
pub use polars::prelude::*;
pub use std::sync::RwLock;
use std::{fs::File, path::Path};

macro_rules! unlock {
    (ref $bind:ident, $self:expr, $method:path) => {
        let $bind = $self
            .0
            .read()
            .map_err(|err| anyhow::anyhow!(concat!(stringify!($method), " failed ({})"), err))?;
    };
    (mut $bind:ident, $self:expr, $method:path) => {
        let mut $bind = $self
            .0
            .try_write()
            .map_err(|err| anyhow::anyhow!(concat!(stringify!($method), " failed ({})"), err))?;
    };
}

pub type PDataFrame = polars::prelude::DataFrame;
pub struct DataFrame(pub RustOpaque<RwLock<PDataFrame>>);
impl DataFrame {
    #[inline]
    fn new(df: PDataFrame) -> Self {
        DataFrame(RustOpaque::new(RwLock::new(df)))
    }
}

pub type PSeries = polars::prelude::Series;
pub struct Series(pub RustOpaque<RwLock<PSeries>>);
impl Series {
    #[inline]
    fn new(series: PSeries) -> Self {
        Series(RustOpaque::new(RwLock::new(series)))
    }
}

pub fn read_csv(
    path: String,
    has_header: Option<bool>,
    columns: Option<Vec<String>>,
    delimiter: Option<u8>,
    // TODO(Desdaemon): Implement ffi.AbiSpecificInteger upstream
    // skip_rows: Option<usize>,
    // skip_rows_after_header: Option<usize>,
    // chunk_size: Option<usize>,
) -> Result<DataFrame> {
    let mut reader = CsvReader::from_path(path)?.with_columns(columns);
    if let Some(has_header) = has_header {
        reader = reader.has_header(has_header)
    }
    if let Some(delimiter) = delimiter {
        reader = reader.with_delimiter(delimiter)
    }
    // if let Some(skip) = skip_rows {
    //     reader = reader.with_skip_rows(skip)
    // }
    // if let Some(skip) = skip_rows_after_header {
    //     reader = reader.with_skip_rows_after_header(skip)
    // }
    // if let Some(size) = chunk_size {
    //     reader = reader.with_chunk_size(size)
    // }
    Ok(DataFrame::new(reader.finish()?))
}

pub fn read_json(path: String) -> Result<DataFrame> {
    let path = resolve_homedir(Path::new(&path));
    let file = File::open(path)?;
    Ok(DataFrame::new(JsonReader::new(file).finish()?))
}

impl DataFrame {
    pub fn column(&self, column: String) -> Result<Series> {
        unlock!(ref my, self, column);
        Ok(Series::new(my.column(&column)?.clone()))
    }

    pub fn columns(&self, columns: Vec<String>) -> Result<Vec<Series>> {
        unlock!(ref my, self, columns);
        Ok(my
            .columns(columns)?
            .into_iter()
            .cloned()
            .map(Series::new)
            .collect())
    }

    pub fn dump(&self) -> Result<String> {
        unlock!(ref my, self, dump);
        Ok(format!("{}", my))
    }
}

impl Series {
    pub fn of_strings(name: String, values: Option<Vec<String>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Utf8)
        })
    }
    pub fn of_i32(name: String, values: Option<Vec<i32>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Int32)
        })
    }
    pub fn of_f64(name: String, values: Option<Vec<f64>>) -> Series {
        Series::new(if let Some(values) = values {
            PSeries::new(&name, values)
        } else {
            PSeries::new_empty(&name, &DataType::Float64)
        })
    }

    // TODO(Desdaemon): implement Vec<bool> upstream
    // pub fn of_bools(name: String, values: Option<Vec<bool>>) -> PSeries {
    //     PSeries::new(if let Some(values) = values {
    //         Series::new(&name, values)
    //     } else {
    //         Series::new_empty(&name, &DataType::Boolean)
    //     })
    // }

    /// Throws an error if trying to append to self.
    pub fn append(&self, other: Series) -> Result<()> {
        unlock!(ref rhs, other, append);
        unlock!(mut lhs, self, append);
        lhs.append(&rhs)?;
        Ok(())
    }

    pub fn as_strings(&self) -> Result<Vec<Option<String>>> {
        unlock!(ref my, self, as_strings);
        Ok(my
            .utf8()?
            .into_iter()
            .map(|e| e.map(ToOwned::to_owned))
            .collect())
    }

    pub fn as_i32(&self) -> Result<Vec<Option<i32>>> {
        unlock!(ref my, self, as_i32);
        Ok(my.i32()?.into_iter().collect())
    }

    pub fn as_f64(&self) -> Result<Vec<Option<f64>>> {
        unlock!(ref my, self, as_f64);
        Ok(my.f64()?.into_iter().collect())
    }
}
