use crate::object::Group;
use crate::types::DataType;
use crate::value::Value;

use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;

type Aggregation = fn(&str, &[String], &Group) -> Value;

pub struct AggregationPrototype {
    pub parameter: DataType,
    pub result: DataType,
}

lazy_static! {
    pub static ref AGGREGATIONS: HashMap<&'static str, Aggregation> = {
        let mut map: HashMap<&'static str, Aggregation> = HashMap::new();
        map.insert("max", aggregation_max);
        map.insert("min", aggregation_min);
        map.insert("sum", aggregation_sum);
        map.insert("avg", aggregation_average);
        map.insert("count", aggregation_count);
        map
    };
}

lazy_static! {
    pub static ref AGGREGATIONS_PROTOS: HashMap<&'static str, AggregationPrototype> = {
        let mut map: HashMap<&'static str, AggregationPrototype> = HashMap::new();
        map.insert(
            "max",
            AggregationPrototype {
                parameter: DataType::Variant(vec![
                    DataType::Integer,
                    DataType::Float,
                    DataType::Text,
                    DataType::Date,
                    DataType::Time,
                    DataType::DateTime,
                ]),
                result: DataType::Integer,
            },
        );
        map.insert(
            "min",
            AggregationPrototype {
                parameter: DataType::Variant(vec![
                    DataType::Integer,
                    DataType::Float,
                    DataType::Text,
                    DataType::Date,
                    DataType::Time,
                    DataType::DateTime,
                ]),
                result: DataType::Integer,
            },
        );
        map.insert(
            "sum",
            AggregationPrototype {
                parameter: DataType::Integer,
                result: DataType::Integer,
            },
        );
        map.insert(
            "avg",
            AggregationPrototype {
                parameter: DataType::Integer,
                result: DataType::Integer,
            },
        );
        map.insert(
            "count",
            AggregationPrototype {
                parameter: DataType::Any,
                result: DataType::Integer,
            },
        );
        map
    };
}

fn aggregation_max(field_name: &str, titles: &[String], objects: &Group) -> Value {
    let column_index = titles.iter().position(|r| r.eq(&field_name)).unwrap();
    let mut max_value = objects.rows[0].values.get(column_index).unwrap();
    for row in &objects.rows {
        let field_value = &row.values.get(column_index).unwrap();
        if max_value.compare(field_value) == Ordering::Greater {
            max_value = field_value;
        }
    }
    max_value.clone()
}

fn aggregation_min(field_name: &str, titles: &[String], objects: &Group) -> Value {
    let column_index = titles.iter().position(|r| r.eq(&field_name)).unwrap();
    let mut min_value = objects.rows[0].values.get(column_index).unwrap();
    for row in &objects.rows {
        let field_value = &row.values.get(column_index).unwrap();
        if min_value.compare(field_value) == Ordering::Less {
            min_value = field_value;
        }
    }
    min_value.clone()
}

fn aggregation_sum(field_name: &str, titles: &[String], objects: &Group) -> Value {
    let mut sum: i64 = 0;
    let column_index = titles.iter().position(|r| r.eq(&field_name)).unwrap();
    for row in &objects.rows {
        let field_value = &row.values.get(column_index).unwrap();
        sum += field_value.as_int();
    }
    Value::Integer(sum)
}

fn aggregation_average(field_name: &str, titles: &[String], objects: &Group) -> Value {
    let mut sum: i64 = 0;
    let count: i64 = objects.len().try_into().unwrap();
    let column_index = titles.iter().position(|r| r.eq(&field_name)).unwrap();
    for row in &objects.rows {
        let field_value = &row.values.get(column_index).unwrap();
        sum += field_value.as_int();
    }
    let avg = sum / count;
    Value::Integer(avg)
}

fn aggregation_count(_field_name: &str, _titles: &[String], objects: &Group) -> Value {
    Value::Integer(objects.len() as i64)
}
