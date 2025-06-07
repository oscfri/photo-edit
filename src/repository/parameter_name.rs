use core::fmt;

#[derive(Debug)]
pub enum ParameterName {
    ExportPath
}

impl fmt::Display for ParameterName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}