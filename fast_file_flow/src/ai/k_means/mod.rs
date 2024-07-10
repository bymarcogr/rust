extern crate linfa;
extern crate linfa_clustering;
extern crate ndarray;
extern crate ndarray_rand;
extern crate plotters;

use linfa::traits::{Fit, Predict};
use linfa_clustering::KMeans;
use plotters::prelude::*;

use crate::ai::shared::Shared;
use crate::constants::path::KMEANS_IMAGE_RESULT;

#[derive(Debug, Clone)]
pub struct KMeansClustering {
    pub centroid_details: String,
    pub result_image_path: String,
}

impl KMeansClustering {
    pub fn default() -> Self {
        Self {
            centroid_details: String::default(),
            result_image_path: String::default(),
        }
    }
    pub async fn new(base: Vec<f64>, compare: Vec<f64>, clusters: usize, iteraciones: u64) -> Self {
        let (centroid_details, result_image_path) =
            Self::get_prediction(base, compare, clusters, iteraciones);
        Self {
            centroid_details,
            result_image_path,
        }
    }

    pub fn get_prediction(
        column1: Vec<f64>,
        column2: Vec<f64>,
        n_clusters: usize,
        iteraciones: u64,
    ) -> (String, String) {
        let (dataset, x, y) = Shared::get_dataset_info(column1, column2);
        // Crear el modelo K-means
        let model = KMeans::params(n_clusters)
            .max_n_iterations(iteraciones)
            .fit(&dataset)
            .expect("KMeans fitting failed");

        // Predecir los clusters
        let assigned_clusters = model.predict(&dataset);

        // Imprimir los centros de los clusters
        let centroids = format!("Cluster centers:\n{:?}", model.centroids());
        println!("{}", centroids);

        let path = KMEANS_IMAGE_RESULT;
        // Visualización de los clusters
        let root = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("K-means Clustering", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x.min..x.max, y.min..y.max)
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

    pub fn to_string(&self) -> String {
        self.centroid_details.clone()
    }
}
