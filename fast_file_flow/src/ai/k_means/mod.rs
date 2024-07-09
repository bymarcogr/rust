extern crate linfa;
extern crate linfa_clustering;
extern crate ndarray;
extern crate ndarray_rand;
extern crate plotters;

use linfa::traits::{Fit, Predict};
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use ndarray::Array2;
use plotters::prelude::*;

#[derive(Debug, Clone)]
pub struct KMeansClustering {
    pub centroid_details: String,
    pub result_image_path: String,
}

impl KMeansClustering {
    pub async fn default() -> Self {
        Self {
            centroid_details: String::default(),
            result_image_path: String::default(),
        }
    }
    pub async fn new(base: Vec<f64>, compare: Vec<f64>, clusters: usize) -> Self {
        let (centroid_details, result_image_path) = Self::get_prediction(base, compare, clusters);
        Self {
            centroid_details,
            result_image_path,
        }
    }

    pub fn get_prediction(
        column1: Vec<f64>,
        column2: Vec<f64>,
        n_clusters: usize,
    ) -> (String, String) {
        let data: Vec<Vec<f64>> = vec![column1, column2];

        let data: Vec<Vec<f64>> = (0..data[0].len())
            .map(|i| data.iter().map(|col| col[i]).collect())
            .collect();
        let data: Array2<f64> = Array2::from_shape_vec(
            (data.len(), data[0].len()),
            data.into_iter().flatten().collect(),
        )
        .unwrap();
        let dataset = DatasetBase::from(data);
        // Crear el modelo K-means
        let model = KMeans::params(n_clusters)
            .max_n_iterations(100)
            .fit(&dataset)
            .expect("KMeans fitting failed");

        // Predecir los clusters
        let assigned_clusters = model.predict(&dataset);

        // Imprimir los centros de los clusters
        let centroids = format!("Cluster centers:\n{:?}", model.centroids());
        println!("{}", centroids);

        let path = r"\\src\\ai\kmeans_plot.png";
        // Visualización de los clusters
        let root = BitMapBackend::new("kmeans_plot.png", (1024, 768)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("K-means Clustering", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0.0..10.0, 0.0..10.0)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        for (i, point) in dataset.records().outer_iter().enumerate() {
            let cluster = assigned_clusters[i];
            let color = match cluster {
                0 => &RED,
                1 => &BLUE,
                _ => &GREEN,
            };
            chart
                .draw_series(std::iter::once(Circle::new(
                    (point[0], point[1]),
                    5,
                    ShapeStyle {
                        color: color.to_rgba(),
                        filled: true,
                        stroke_width: 1,
                    },
                )))
                .unwrap();
        }

        for center in model.centroids().outer_iter() {
            chart
                .draw_series(std::iter::once(Cross::new(
                    (center[0], center[1]),
                    10,
                    &BLACK,
                )))
                .unwrap();
        }

        (centroids, path.to_string())
    }
}
