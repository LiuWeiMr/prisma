use prisma_query::ast::*;

use chrono::{DateTime, Utc};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

pub type PrismaListValue = Vec<PrismaValue>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum GraphqlId {
    String(String),
    Int(usize),
    UUID(Uuid),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PrismaValue {
    String(String),
    Float(f64),
    Boolean(bool),
    DateTime(DateTime<Utc>),
    Enum(String),
    Json(String),
    Int(i32),
    Relation(usize),
    Null,
    Uuid(Uuid),
    GraphqlId(GraphqlId),
    List(PrismaListValue),
}

impl PrismaValue {
    pub fn is_null(&self) -> bool {
        match self {
            PrismaValue::Null => true,
            _ => false,
        }
    }
}

impl fmt::Display for PrismaValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrismaValue::String(x) => x.fmt(f),
            PrismaValue::Float(x) => x.fmt(f),
            PrismaValue::Boolean(x) => x.fmt(f),
            PrismaValue::DateTime(x) => x.fmt(f),
            PrismaValue::Enum(x) => x.fmt(f),
            PrismaValue::Json(x) => x.fmt(f),
            PrismaValue::Int(x) => x.fmt(f),
            PrismaValue::Relation(x) => x.fmt(f),
            PrismaValue::Null => "null".fmt(f),
            PrismaValue::Uuid(x) => x.fmt(f),
            PrismaValue::GraphqlId(x) => match x {
                GraphqlId::String(x) => x.fmt(f),
                GraphqlId::Int(x) => x.fmt(f),
                GraphqlId::UUID(x) => x.fmt(f),
            },
            PrismaValue::List(x) => {
                let as_string = format!("{:?}", x);
                as_string.fmt(f)
            }
        }
    }
}

impl From<&str> for PrismaValue {
    fn from(s: &str) -> Self {
        PrismaValue::from(s.to_string())
    }
}

impl From<String> for PrismaValue {
    fn from(s: String) -> Self {
        PrismaValue::String(s)
    }
}

impl From<f64> for PrismaValue {
    fn from(s: f64) -> Self {
        PrismaValue::Float(s)
    }
}

impl From<f32> for PrismaValue {
    fn from(s: f32) -> Self {
        PrismaValue::Float(s as f64)
    }
}

impl From<bool> for PrismaValue {
    fn from(s: bool) -> Self {
        PrismaValue::Boolean(s)
    }
}

impl From<i32> for PrismaValue {
    fn from(s: i32) -> Self {
        PrismaValue::Int(s)
    }
}

impl From<Uuid> for PrismaValue {
    fn from(s: Uuid) -> Self {
        PrismaValue::Uuid(s)
    }
}

impl From<PrismaListValue> for PrismaValue {
    fn from(s: PrismaListValue) -> Self {
        PrismaValue::List(s)
    }
}

impl From<GraphqlId> for PrismaValue {
    fn from(id: GraphqlId) -> PrismaValue {
        PrismaValue::GraphqlId(id)
    }
}

impl From<GraphqlId> for DatabaseValue {
    fn from(id: GraphqlId) -> DatabaseValue {
        match id {
            GraphqlId::String(s) => s.into(),
            GraphqlId::Int(i) => (i as i64).into(),
            GraphqlId::UUID(u) => u.to_hyphenated_ref().to_string().into(),
        }
    }
}

impl From<&GraphqlId> for DatabaseValue {
    fn from(id: &GraphqlId) -> DatabaseValue {
        id.clone().into()
    }
}

impl From<PrismaValue> for DatabaseValue {
    fn from(pv: PrismaValue) -> DatabaseValue {
        match pv {
            PrismaValue::String(s) => s.into(),
            PrismaValue::Float(f) => (f as f64).into(),
            PrismaValue::Boolean(b) => b.into(),
            PrismaValue::DateTime(d) => d.timestamp_millis().into(),
            PrismaValue::Enum(e) => e.into(),
            PrismaValue::Json(j) => j.into(),
            PrismaValue::Int(i) => (i as i64).into(),
            PrismaValue::Relation(i) => (i as i64).into(),
            PrismaValue::Null => DatabaseValue::Parameterized(ParameterizedValue::Null),
            PrismaValue::Uuid(u) => u.to_hyphenated_ref().to_string().into(),
            PrismaValue::GraphqlId(id) => id.into(),
            PrismaValue::List(_) => panic!("List values are not supported here"),
        }
    }
}

impl FromSql for GraphqlId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()
            .map(|strval| GraphqlId::String(strval.to_string()))
            .or_else(|_| value.as_i64().map(|intval| GraphqlId::Int(intval as usize)))
    }
}

impl From<&str> for GraphqlId {
    fn from(s: &str) -> Self {
        GraphqlId::from(s.to_string())
    }
}

impl From<String> for GraphqlId {
    fn from(s: String) -> Self {
        GraphqlId::String(s)
    }
}

impl From<usize> for GraphqlId {
    fn from(id: usize) -> Self {
        GraphqlId::Int(id)
    }
}

impl From<Uuid> for GraphqlId {
    fn from(uuid: Uuid) -> Self {
        GraphqlId::UUID(uuid)
    }
}
