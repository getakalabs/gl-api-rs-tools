use itertools::Itertools;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use std::str::FromStr;

use crate::traits::prelude::*;
use crate::Swap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MongoArray<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> {
    Array(Vec<Option<Swap<T>>>),
    ArrayString(Vec<String>),
    String(String),
    None
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> Default for MongoArray<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> Dedup for MongoArray<T> {
    fn dedup(&self) -> Self {
        match self.clone() {
            Self::Array(array) => {
                let mut data = Vec::new();

                for item in array.into_iter().flatten() {
                    if item.to_string().trim().to_lowercase().as_str() != "none" {
                        data.push(item.to_string().trim().to_string());
                    }
                }

                let old_array:Vec<_> = data.into_iter().unique().collect();
                let mut new_array = Vec::new();

                for item in old_array {
                    new_array.push(Some(Swap::<T>::String(item)));
                }

                Self::Array(new_array)
            },
            _ => self.clone()
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> From<Vec<String>> for MongoArray<T> {
    fn from(value: Vec<String>) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => {
                let mut array = Vec::new();
                for item in value {
                    array.push(Some(Swap::<T>::String(item)));
                }
                Self::Array(array)
            }
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> From<Vec<T>> for MongoArray<T> {
    fn from(value: Vec<T>) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => {
                let mut array = Vec::new();
                for item in value {
                    array.push(Some(Swap::new(item)));
                }
                Self::Array(array)
            }
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> From<&[T]> for MongoArray<T> {
    fn from(value: &[T]) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => {
                let mut array = Vec::new();
                for item in value {
                    array.push(Some(Swap::new(item.clone())));
                }
                Self::Array(array)
            }
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> GetArrayObject<T> for MongoArray<T> {
    fn get_array_object(&self) -> Option<Vec<T>> {
        match self {
            Self::Array(data) => {
                let mut array = Vec::new();

                for item in data.iter().flatten() {
                    array.push(item.clone().get_swap().unwrap_or_default())
                }

                match array.is_empty() {
                    true => None,
                    false => Some(array)
                }
            },
            _ => None
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> GetArrayObjectId for MongoArray<T> {
    fn get_array_object_id(&self) -> Vec<ObjectId> {
        match self {
            Self::Array(data) => {
                let mut array = Vec::new();

                for item in data {
                    match item {
                        Some(item) => if let Some(id) = item.get_object_id() {
                            array.push(id)
                        },
                        None => {}
                    }
                }

                array
            },
            Self::ArrayString(data) => {
                let mut array = Vec::new();

                for item in data {
                    match ObjectId::from_str(item) {
                        Ok(data) => array.push(data.clone()),
                        Err(_) => {}
                    }
                }

                array
            },
            Self::String(data) => {
                let item = urlencoding::decode(data).unwrap_or_default().to_string();
                match serde_json::from_str::<Vec<String>>(&item) {
                    Ok(data) => {
                        let mut array = Vec::new();

                        for item in data {
                            match ObjectId::from_str(&item) {
                                Ok(data) => array.push(data.clone()),
                                Err(_) => {}
                            }
                        }

                        array
                    },
                    Err(_) => {
                        let mut array = Vec::new();

                        for item in data.split(',') {
                            match ObjectId::from_str(item) {
                                Ok(data) => array.push(data.clone()),
                                Err(_) => {}
                            }
                        }

                        array
                    }
                }
            },
            Self::None => Vec::new()
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> GetArrayString for MongoArray<T> {
    fn get_array_string(&self) -> Option<Vec<String>> {
        match self.clone() {
            Self::Array(array) => {
                let mut data = Vec::new();

                for item in array.into_iter().flatten() {
                    if !item.to_string().is_empty() {
                        data.push(item.to_string());
                    }
                }

                match data.is_empty() {
                    true => None,
                    false => Some(data)
                }
            },
            Self::ArrayString(data) => match data.is_empty() {
                true => None,
                false => Some(data)
            },
            Self::String(data) => {
                match data.is_empty() {
                    true => None,
                    false => Some(data.split(',').map(|x| x.to_string()).collect())
                }
            },
            Self::None => None
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> IsEmpty for MongoArray<T> {
    fn is_empty(&self) -> bool {
        match self {
            Self::Array(data) => data.is_empty(),
            Self::ArrayString(data) => data.is_empty(),
            Self::String(data) => data.is_empty(),
            Self::None => true
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> ToBson for MongoArray<T> {
    fn to_bson(&self) -> Option<Self> {
        match self {
            Self::Array(data) => {
                let mut array = Vec::new();

                for item in data {
                    match item {
                        Some(item) => {
                            match item.to_bson() {
                                Some(item) => array.push(Some(item)),
                                None => if let Some(id) = item.get_object_id() {
                                    array.push(Some(Swap::ObjectId(id)))
                                }
                            }
                        },
                        None => {}
                    }
                }

                match array.is_empty() {
                    true => None,
                    false => Some(Self::Array(array))
                }
            },
            Self::ArrayString(data) => {
                let mut array = Vec::new();

                for item in data {
                    array.push(item.clone());
                }

                match array.is_empty() {
                    true => None,
                    false => Some(Self::ArrayString(array))
                }
            }
            Self::String(data) => {
                match data.is_empty() {
                    true => None,
                    false => Some(Self::String(data.clone()))
                }
            },
            Self::None => None
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> ToJson for MongoArray<T> {
    fn to_json(&self) -> Option<Self> {
        match self {
            Self::Array(data) => {
                let mut array = Vec::new();

                for item in data {
                    match item {
                        Some(item) => {
                            if let Some(item) = item.to_json() {
                                array.push(Some(item))
                            }
                        },
                        None => {}
                    }
                }

                match array.is_empty() {
                    true => None,
                    false => Some(Self::Array(array))
                }
            },
            Self::ArrayString(data) => {
                let mut array = Vec::new();

                for item in data {
                    array.push(item.clone())
                }

                match array.is_empty() {
                    true => None,
                    false => Some(Self::ArrayString(array))
                }
            },
            Self::String(data) => {
                match data.is_empty() {
                    true => None,
                    false => Some(Self::String(data.clone()))
                }
            },
            Self::None => None
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> MongoArray<T> {
    pub fn get_opt_array(value: &Vec<T>) -> Option<Self> {
        match Self::from(value.clone()).is_empty() {
            true => None,
            false => Some(Self::from(value.to_owned()))
        }
    }
}
