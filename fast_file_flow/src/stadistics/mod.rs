extern crate statistical;
use data_classification::DataClassification;
use data_type::DataType;
use dynamictable::iced_column::IcedColumn;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};
pub mod data_classification;
pub mod data_type;
use ndarray::{Array1, ArrayView1};
use rayon::prelude::*;

use crate::dynamictable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stadistics {
    pub header: String,
    pub classification: DataClassification,
    pub data_type: DataType,
    pub distinct: String,
    pub minimum: String,
    pub maximum: String,
    pub mean: String,
    pub median: String,
    pub mode: String,
    pub range: String,
    pub variance: String,
    pub quartil: String,
    pub percentil: String,
    pub std_dev: String,
}

impl Stadistics {
    pub fn default() -> Self {
        Self {
            classification: DataClassification::Unknown,
            data_type: DataType::Unknown,
            distinct: String::default(),
            minimum: String::default(),
            maximum: String::default(),
            mean: String::default(),
            median: String::default(),
            mode: String::default(),
            range: String::default(),
            variance: String::default(),
            quartil: String::default(),
            percentil: String::default(),
            std_dev: String::default(),
            header: String::default(),
        }
    }
    pub async fn new(selected_column: &IcedColumn, full_column: Vec<String>) -> Self {
        let (classification, data_type) = Self::get_column_analysis(&full_column);
        if classification == DataClassification::Quantitative {
            let (
                distinct_values,
                max,
                min,
                mean_value,
                median_value,
                mode_value,
                range,
                variance_value,
                std_dev_value,
                percentil,
                quartil,
            ) = Self::get_analysis_numeric(&full_column);
            Self {
                classification,
                data_type,
                header: selected_column.column_header.clone(),
                distinct: distinct_values.to_formatted_string(&Locale::en),
                minimum: format!("{:.6}", min),
                maximum: format!("{:.6}", max),
                mean: format!("{:.6}", mean_value),
                median: format!("{:.6}", median_value),
                mode: format!("{:.6}", mode_value),
                range: format!("{:.6}", range),
                variance: format!("{:.6}", variance_value),
                quartil: format!("{:.6}", quartil),
                percentil: format!("{:.6}", percentil),
                std_dev: format!("{:.6}", std_dev_value),
            }
        } else {
            let (
                distinct_values,
                mode_value,
                max,
                min,
                mean_value,
                median_value,
                range,
                variance_value,
                std_dev_value,
                percentil,
                quartil,
            ) = Self::get_analysis_text(&full_column);
            Self {
                classification,
                data_type,
                header: selected_column.column_header.clone(),
                distinct: distinct_values.to_formatted_string(&Locale::en),
                minimum: format!("{:.6}", min),
                maximum: format!("{:.6}", max),
                mean: format!("{:.6}", mean_value),
                median: format!("{:.6}", median_value),
                mode: format!("{:.6}", mode_value),
                range: format!("{:.6}", range),
                variance: format!("{:.6}", variance_value),
                percentil: format!("{:.6}", percentil),
                quartil: format!("{:.6}", quartil),
                std_dev: format!("{:.6}", std_dev_value),
            }
        }
    }

    pub fn get_column_analysis(column: &Vec<String>) -> (DataClassification, DataType) {
        let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$|^\d{2}/\d{2}/\d{4}$").unwrap();
        let time_re = Regex::new(r"^\d{2}:\d{2}(:\d{2})?$").unwrap();
        let datetime_re = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}(:\d{2})?$").unwrap();
        let coordinates_re =
            Regex::new(r"^-?\d+\.\d+,\s*-?\d+\.\d+$|^\(-?\d+\.\d+,\s*-?\d+\.\d+\)$").unwrap();

        let mut counts: HashMap<DataClassification, usize> = HashMap::new();
        let mut type_counts: HashMap<DataType, usize> = HashMap::new();

        for value in column {
            let data_classification = if value.parse::<f64>().is_ok() {
                DataClassification::Quantitative
            } else {
                DataClassification::Qualitative
            };

            *counts.entry(data_classification.clone()).or_insert(0) += 1;

            let data_type = if value.parse::<i64>().is_ok() {
                DataType::Integer
            } else if value.parse::<f64>().is_ok() {
                DataType::Float
            } else if date_re.is_match(&value) {
                DataType::Date
            } else if time_re.is_match(&value) {
                DataType::Time
            } else if datetime_re.is_match(&value) {
                DataType::DateTime
            } else if coordinates_re.is_match(&value) {
                DataType::Coordinates
            } else {
                DataType::Text
            };

            *type_counts.entry(data_type).or_insert(0) += 1;
        }

        let data_classification = if counts.get(&DataClassification::Quantitative).unwrap_or(&0)
            >= counts.get(&DataClassification::Qualitative).unwrap_or(&0)
        {
            DataClassification::Quantitative
        } else {
            DataClassification::Qualitative
        };

        let data_type = type_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(data_type, _)| data_type)
            .unwrap_or(DataType::Unknown);

        (data_classification, data_type)
    }

    pub fn get_analysis_numeric(
        column: &Vec<String>,
    ) -> (usize, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let start = Instant::now();
        let data: Vec<f64> = column
            .par_iter()
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();
        let distinct_values: HashSet<_> = data.par_iter().map(|&x| x.to_bits()).collect();
        let distinct_count = distinct_values.len();

        let mut occurrences = HashMap::new();
        for &value in &data {
            *occurrences.entry(value.to_bits()).or_insert(0) += 1;
        }
        let mode_value = occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| f64::from_bits(val))
            .unwrap_or(f64::NAN);

        let max = *data
            .par_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let min = *data
            .par_iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let range = max - min;

        let mut sorted_data = data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let q1 = Self::calculate_quantile(&sorted_data, 0.25);
        let q3 = Self::calculate_quantile(&sorted_data, 0.75);
        let new_array = ndarray::Array1::from(data);
        let mean_value = &new_array.mean().unwrap();
        let median_value = Self::calculate_median(new_array.view());
        let variance_value = Self::manual_variance(&new_array, mean_value);
        let std_dev_value = variance_value.sqrt();

        crate::util::print_timer("Stadistical Analysis", start);

        (
            distinct_count,
            max,
            min,
            *mean_value,
            median_value,
            mode_value,
            range,
            variance_value,
            std_dev_value,
            q1,
            q3,
        )
    }

    fn manual_variance(data: &Array1<f64>, mean_value: &f64) -> f64 {
        let len = data.len() as f64;
        let sum: f64 = data
            .par_iter() // Usar iterador paralelo
            .map(|&x| (x - mean_value).powi(2))
            .sum();
        sum / len
    }

    fn calculate_median(data: ArrayView1<f64>) -> f64 {
        let mut sorted_data: Vec<f64> = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let len = sorted_data.len();
        if len % 2 == 0 {
            (sorted_data[len / 2 - 1] + sorted_data[len / 2]) / 2.0
        } else {
            sorted_data[len / 2]
        }
    }

    fn calculate_quantile(sorted_data: &[f64], quantile: f64) -> f64 {
        let pos = (sorted_data.len() as f64 - 1.0) * quantile;
        let base = pos.floor() as usize;
        let rest = pos - base as f64;

        if base + 1 < sorted_data.len() {
            sorted_data[base] + rest * (sorted_data[base + 1] - sorted_data[base])
        } else {
            sorted_data[base]
        }
    }

    fn calculate_quantile_text(sorted_data: &Vec<usize>, quantile: f64) -> f64 {
        let pos = (sorted_data.len() as f64 - 1.0) * quantile;
        let base = pos.floor() as usize;
        let rest = pos - base as f64;

        if base + 1 < sorted_data.len() {
            sorted_data[base] as f64
                + rest * (sorted_data[base + 1] as f64 - sorted_data[base] as f64)
        } else {
            sorted_data[base] as f64
        }
    }

    fn get_analysis_text(
        column: &Vec<String>,
    ) -> (
        usize,
        String,
        usize,
        usize,
        f64,
        f64,
        usize,
        f64,
        f64,
        f64,
        f64,
    ) {
        let start = Instant::now();
        let data: Vec<&str> = column
            .par_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.as_str())
            .collect();

        let distinct_values: HashSet<_> = data.par_iter().cloned().collect();

        let mut occurrences = HashMap::new();
        for &value in data.iter() {
            *occurrences.entry(value).or_insert(0) += 1;
        }
        let mode_value = occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val);

        let lengths: Vec<usize> = data.par_iter().map(|s| s.len()).collect();
        let max = *lengths.par_iter().max().unwrap();
        let min = *lengths.par_iter().min().unwrap();
        let mean = lengths.par_iter().sum::<usize>() as f64 / lengths.len() as f64;

        let mut sorted_lengths = lengths.clone();
        sorted_lengths.sort();
        let median = if sorted_lengths.len() % 2 == 0 {
            (sorted_lengths[sorted_lengths.len() / 2 - 1]
                + sorted_lengths[sorted_lengths.len() / 2]) as f64
                / 2.0
        } else {
            sorted_lengths[sorted_lengths.len() / 2] as f64
        };

        let variance = {
            let mean = mean;
            let variance_sum = lengths
                .par_iter()
                .map(|&len| {
                    let diff = len as f64 - mean;
                    diff * diff
                })
                .sum::<f64>();
            variance_sum / lengths.len() as f64
        };

        let std_dev = variance.sqrt();
        let range = max - min;

        let q1 = Self::calculate_quantile_text(&sorted_lengths, 0.25);
        let q3 = Self::calculate_quantile_text(&sorted_lengths, 0.75);
        crate::util::print_timer("Stadistical Analysis Text", start);
        (
            distinct_values.len(),
            String::from(mode_value.unwrap()),
            max,
            min,
            mean,
            median,
            range,
            variance,
            std_dev,
            q1,
            q3,
        )
    }
}
impl Default for Stadistics {
    fn default() -> Self {
        Stadistics::default()
    }
}
