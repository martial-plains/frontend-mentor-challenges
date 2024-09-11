use derive_more::derive::IsVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, IsVariant)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Quiz {
    pub title: String,
    pub icon: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub options: Vec<String>,
    pub answer: String,
}
