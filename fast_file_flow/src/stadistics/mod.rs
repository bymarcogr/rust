extern crate statistical;
use crate::dynamictable::IcedColumn;
use data_classification::DataClassification;
use data_type::DataType;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use statistical::{mean, median, standard_deviation, variance};
use std::collections::{HashMap, HashSet};
pub mod data_classification;
pub mod data_type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stadistics {
    pub header: String,
    pub classification: DataClassification,
    pub data_type: DataType,
    pub distinct: String, // numero diferentes
    pub minimum: String,  // Minimo si aplica
    pub maximum: String,  // Maximo si aplica
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
        println!("waiting analisys");
        let (classification, data_type) = Self::get_column_analysis(&full_column);
        println!("loading");
        if classification == DataClassification::Quantitative {
            println!("quantitativa");
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
            println!("quantitativo");
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

    fn get_column_analysis(column: &Vec<String>) -> (DataClassification, DataType) {
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
        println!("Filtering");
        let data: Vec<f64> = column
            .iter()
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();
        println!("data");
        let distinct_values: HashSet<_> = data.iter().map(|&x| x.to_bits()).collect();
        let distinct_count = distinct_values.len();
        println!("distinct {}", distinct_count);
        // Most repeated value (mode)
        let mut occurrences = HashMap::new();
        for &value in &data {
            *occurrences.entry(value.to_bits()).or_insert(0) += 1;
        }
        let mode_value = occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| f64::from_bits(val))
            .unwrap_or(f64::NAN);

        println!("mode {}", mode_value);
        // Basic statistics
        let max = *data
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        println!("max {}", max);
        let min = *data
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        println!("min {}", min);
        let mean_value = mean(&data);
        println!("mean {}", mean_value);
        let median_value = median(&data);
        println!("median {}", median_value);
        let variance_value = variance(&data, None);
        println!("variance {}", variance_value);
        let std_dev_value = standard_deviation(&data, None);
        println!("std dev {}", std_dev_value);
        let range = max - min;
        println!("range {}", range);

        let mut sorted_data = data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("sorted");
        let q1 = Self::calculate_quantile(&sorted_data, 0.25);
        println!("q1 {}", q1);
        let q3 = Self::calculate_quantile(&sorted_data, 0.75);
        println!("q3 {}", q3);
        (
            distinct_count,
            max,
            min,
            mean_value,
            median_value,
            mode_value,
            range,
            variance_value,
            std_dev_value,
            q1,
            q3,
        )
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
        let data: Vec<&str> = column
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.as_str())
            .collect();

        // Distinct values
        let distinct_values: HashSet<_> = data.iter().cloned().collect();

        // Most repeated value (mode)
        let mut occurrences = HashMap::new();
        for &value in data.iter() {
            *occurrences.entry(value).or_insert(0) += 1;
        }
        let mode_value = occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val);

        // Basic statistics
        let lengths: Vec<usize> = data.iter().map(|s| s.len()).collect();
        let max = *lengths.iter().max().unwrap();
        let min = *lengths.iter().min().unwrap();
        let mean = lengths.iter().sum::<usize>() as f64 / lengths.len() as f64;

        // Calculate median manually
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
                .iter()
                .map(|&len| {
                    let diff = len as f64 - mean;
                    diff * diff
                })
                .sum::<f64>();
            variance_sum / lengths.len() as f64
        };

        let std_dev = variance.sqrt();
        let range = max - min;

        // Calculate quartiles
        let q1 = Self::calculate_quantile_text(&sorted_lengths, 0.25);
        let q3 = Self::calculate_quantile_text(&sorted_lengths, 0.75);

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
