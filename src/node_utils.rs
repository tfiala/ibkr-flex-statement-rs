use anyhow::Result;
use roxmltree::Node;
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub struct NodeWrapper<'a> {
    pub node: Node<'a, 'a>,
}

impl NodeWrapper<'_> {
    pub fn get_attribute(&self, attribute_name: &str) -> Result<String> {
        Ok(self.node.attribute(attribute_name).unwrap().to_string())
    }

    pub fn get_attribute_opt(&self, attribute_name: &str) -> Option<String> {
        match self.node.attribute(attribute_name) {
            Some(s) => {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            }
            None => None,
        }
    }

    pub fn parse_attribute<T: FromStr>(&self, attribute_name: &str) -> Result<T>
    where
        <T as FromStr>::Err: Send + Sync + Debug + Display,
        <T as FromStr>::Err: 'static,
    {
        self.node
            .attribute(attribute_name)
            .unwrap()
            .parse::<T>()
            .map_err(anyhow::Error::msg)
    }

    pub fn parse_attribute_opt<T: FromStr>(&self, attribute_name: &str) -> Result<Option<T>>
    where
        <T as FromStr>::Err: Send + Sync + Debug + Display,
        <T as FromStr>::Err: 'static,
    {
        match self.node.attribute(attribute_name) {
            Some(s) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(s.parse::<T>().map_err(anyhow::Error::msg)?))
                }
            }
            None => Ok(None),
        }
    }
}
