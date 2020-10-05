use crate::query::{Filter, SortDirection};

#[derive(Debug, Clone)]
pub enum AqlOperation {
    Filter(Filter),
    Prune(Filter),
    Limit { skip: Option<u32>, limit: u32 },
    Sort { field: String, direction: SortDirection },
}

#[derive(Debug, Clone)]
pub struct OperationContainer(pub Vec<AqlOperation>);

impl OperationContainer {
    pub fn to_aql(&self, collection_id: &str) -> String {
        let mut res = String::new();
        let mut last_was_sort = false;
        for operation in self.0.iter() {
            match operation {
                AqlOperation::Limit { skip, limit } => {
                    let skip_str = match skip {
                        None => String::new(),
                        Some(val) => format!("{}, ", val)
                    };
                    res = format!("{} LIMIT {}{}", res, skip_str, limit);
                    last_was_sort = false;
                }
                AqlOperation::Filter(filter) => {
                    res = format!("{} FILTER {}", res, filter.to_aql(collection_id));
                    last_was_sort = false;
                }
                AqlOperation::Prune(filter) => {
                    res = format!("{} PRUNE {}", res, filter.to_aql(collection_id));
                    last_was_sort = false;
                }
                AqlOperation::Sort { field, direction } => {
                    if !last_was_sort {
                        res += " SORT";
                    } else {
                        res += ",";
                    }
                    res = format!("{} {}.{} {}", res, collection_id, field, direction);
                    last_was_sort = true;
                }
            }
        }
        String::from(res.trim_start())
    }
}