use crate::node_utils::NodeWrapper;
use anyhow::Result;
use chrono_tz::Tz;
use std::collections::HashMap;

pub trait StatementSection {
    fn from_node(node: &NodeWrapper) -> Result<Self>
    where
        Self: Sized;
}

pub trait StatementSectionWithTimezone {
    fn from_node(node: &NodeWrapper, timezone_map: &HashMap<String, Tz>) -> Result<Self>
    where
        Self: Sized;
}
