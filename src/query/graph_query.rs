use serde::export::fmt::{self, Display};
use serde::export::Formatter;

#[derive(Clone, Debug)]
pub enum GraphQueryDirection {
    Outbound,
    Inbound,
    Any,
}

#[derive(Clone, Debug)]
pub struct GraphQueryData {
    pub direction: GraphQueryDirection,
    pub start_vertex: String,
    pub min: u16,
    pub max: u16,
    pub named_graph: bool,
}

impl Display for GraphQueryDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Inbound => "INBOUND",
                Self::Outbound => "OUTBOUND",
                _ => "ANY",
            }
        )
    }
}
