use std::{collections::HashMap, ops::Deref};

use leptos::{
    attr::custom::{custom_attribute, CustomAttr},
    html::button,
    prelude::AddAnyAttr,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attributes(Option<HashMap<String, Option<String>>>);

impl Attributes {
    pub fn with_defaults<I: Into<Attributes>>(mut self, defaults: I) -> Attributes {
        let defaults: Attributes = defaults.into();

        self.0 = match (self.0, defaults.0) {
            (Some(map), Some(defaults)) => Some(defaults.into_iter().chain(map).collect()),
            (Some(map), None) => Some(map),
            (None, Some(defaults)) => Some(defaults),
            (None, None) => None,
        };

        self
    }

    pub fn to_custom_attributes(self) -> Vec<CustomAttr<String, Option<String>>> {
        self.0
            .map(|map| {
                map.into_iter()
                    .map(|(key, value)| custom_attribute(key, value))
                    .collect()
            })
            .unwrap_or_default()
    }

    // pub fn test(self) {
    //     let attrs = self.to_custom_attributes();

    //     let mut tag = button();

    //     let test = attrs
    //         .into_iter()
    //         .fold(tag, |tag, attr| tag.add_any_attr(attr));
    // }
}

impl Deref for Attributes {
    type Target = Option<HashMap<String, Option<String>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashMap<String, Option<String>>> for Attributes {
    fn from(value: HashMap<String, Option<String>>) -> Attributes {
        Attributes(Some(value))
    }
}

impl From<HashMap<String, String>> for Attributes {
    fn from(value: HashMap<String, String>) -> Attributes {
        Attributes(Some(
            value
                .into_iter()
                .map(|(key, value)| (key, Some(value)))
                .collect(),
        ))
    }
}

impl<const N: usize> From<[(&str, Option<&str>); N]> for Attributes {
    fn from(value: [(&str, Option<&str>); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(value.map(|(key, value)| {
            (key.to_string(), value.map(|value| value.to_string()))
        }))))
    }
}

impl<const N: usize> From<[(&str, &str); N]> for Attributes {
    fn from(value: [(&str, &str); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(
            value.map(|(key, value)| (key.to_string(), Some(value.to_string()))),
        )))
    }
}

impl<const N: usize> From<[(&str, Option<String>); N]> for Attributes {
    fn from(value: [(&str, Option<String>); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(
            value.map(|(key, value)| (key.to_string(), value)),
        )))
    }
}

impl<const N: usize> From<[(&str, String); N]> for Attributes {
    fn from(value: [(&str, String); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(
            value.map(|(key, value)| (key.to_string(), Some(value))),
        )))
    }
}

impl<const N: usize> From<[(String, Option<String>); N]> for Attributes {
    fn from(value: [(String, Option<String>); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(value)))
    }
}

impl<const N: usize> From<[(String, String); N]> for Attributes {
    fn from(value: [(String, String); N]) -> Attributes {
        Attributes(Some(HashMap::from_iter(
            value.map(|(key, value)| (key, Some(value))),
        )))
    }
}
