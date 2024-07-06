use std::time::Instant;

use tokio::join;

use crate::stadistics::data_classification::DataClassification;
#[derive(Debug, Clone)]
pub struct CorrelationAnalysis {
    pub spearman_correlation: f64,
    pub pearson_correlation: f64,
    pub covariance: f64,
}

impl CorrelationAnalysis {
    pub fn default() -> Self {
        Self {
            spearman_correlation: 0.0,
            pearson_correlation: 0.0,
            covariance: 0.0,
        }
    }

    pub async fn new(
        column_base: &Vec<f64>,
        base_type: DataClassification,
        column_compare: &Vec<f64>,
        compare_class: DataClassification,
    ) -> Self {
        let start = Instant::now();

        // transformacion de columna

        let (spearman, pearson, cov) = join!(
            Self::spearman_correlation(column_base, column_compare),
            Self::pearson_correlation(column_base, column_compare),
            Self::covariance(column_base, column_compare)
        );
        let duration = start.elapsed();

        println!(
            "CorrelationAnalysis Execution time: {:?}",
            duration.as_secs_f64()
        );

        Self {
            spearman_correlation: spearman,
            pearson_correlation: pearson,
            covariance: cov,
        }
    }

    async fn pearson_correlation(column_base: &Vec<f64>, column_compare: &Vec<f64>) -> f64 {
        let len = column_base.len();
        assert!(
            len > 0 && len == column_compare.len(),
            "Vectors must be non-empty and of the same length"
        );

        // Calcular la media de cada vector
        let mean_x: f64 = column_base.iter().sum::<f64>() / len as f64;
        let mean_y: f64 = column_compare.iter().sum::<f64>() / len as f64;

        // Inicializar las variables para los cálculos
        let mut numerator = 0.0;
        let mut denominator_x = 0.0;
        let mut denominator_y = 0.0;

        // Calcular las sumas necesarias en una única iteración
        for i in 0..len {
            let diff_x = column_base[i] - mean_x;
            let diff_y = column_compare[i] - mean_y;
            numerator += diff_x * diff_y;
            denominator_x += diff_x * diff_x; // Corregido de diff_x * diff_y a diff_x * diff_x
            denominator_y += diff_y * diff_y; // Corregido de diff_x * diff_y a diff_y * diff_y
        }

        if denominator_x == 0.0 || denominator_y == 0.0 {
            return 0.0;
        }

        numerator / (denominator_x.sqrt() * denominator_y.sqrt())
    }

    async fn spearman_correlation(column_base: &Vec<f64>, column_compare: &Vec<f64>) -> f64 {
        let ranks_x = Self::rankify(column_base);
        let ranks_y = Self::rankify(column_compare);

        Self::pearson_correlation(&ranks_x, &ranks_y).await
    }

    async fn covariance(column_base: &Vec<f64>, column_compare: &Vec<f64>) -> f64 {
        let len = column_base.len();
        assert!(
            len > 0 && len == column_compare.len(),
            "Vectors must be non-empty and of the same length"
        );

        let mean_x: f64 = column_base.iter().sum::<f64>() / len as f64;
        let mean_y: f64 = column_compare.iter().sum::<f64>() / len as f64;

        let mut covariance = 0.0;

        for i in 0..len {
            let diff_x = column_base[i] - mean_x;
            let diff_y = column_compare[i] - mean_y;
            covariance += diff_x * diff_y;
        }

        covariance / len as f64
    }

    fn rankify(v: &[f64]) -> Vec<f64> {
        let mut ranks = vec![0.0; v.len()];
        let mut sorted: Vec<_> = v.iter().enumerate().collect();
        sorted.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let mut rank = 1.0;
        for (i, &(_, val)) in sorted.iter().enumerate() {
            if i > 0 && val != sorted[i - 1].1 {
                rank = i as f64 + 1.0;
            }
            ranks[sorted[i].0] = rank;
        }
        ranks
    }
}
