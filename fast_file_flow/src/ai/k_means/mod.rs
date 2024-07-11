extern crate linfa;
extern crate linfa_clustering;
extern crate ndarray;
extern crate ndarray_rand;
extern crate plotters;

use crate::ai::shared::{Ranges, Shared};
use crate::constants::path::KMEANS_IMAGE_RESULT;
use linfa::{
    traits::{Fit, Predict},
    DatasetBase,
};
use linfa_clustering::KMeans;
use plotters::prelude::*;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct KMeansClustering {
    pub centroid_details: String,
    pub result_image_path: String,
    pub is_dirty: bool,
}

impl KMeansClustering {
    pub fn new() -> Self {
        Self {
            centroid_details: String::default(),
            result_image_path: String::default(),
            is_dirty: false,
        }
    }

    pub async fn get_prediction(
        &mut self,
        column1: Vec<f64>,
        column2: Vec<f64>,
        n_clusters: usize,
        iteraciones: u64,
    ) -> Result<String, Box<dyn Error>> {
        if self.is_dirty {
            return Ok(self.to_string());
        }

        let data = Shared::get_dataset_info(column1, column2);
        let dataset = DatasetBase::from(data);

        println!("Start Predicting");
        let model = KMeans::params(n_clusters)
            .max_n_iterations(iteraciones)
            .fit(&dataset)
            .expect("KMeans fitting failed");

        // Predecir los clusters
        let assigned_clusters = model.predict(&dataset);

        println!("Get Ranges");
        let (min_x, max_x) = dataset
            .records()
            .column(0)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
                (min.min(val), max.max(val))
            });

        let x: Ranges = Ranges {
            max: max_x,
            min: min_x,
        };
        let (min_y, max_y) = dataset
            .records()
            .column(1)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
                (min.min(val), max.max(val))
            });
        let y: Ranges = Ranges {
            max: max_y,
            min: min_y,
        };

        println!("Results");
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

        self.centroid_details = centroids;
        self.result_image_path = path.to_owned();
        self.is_dirty = true;

        Ok(self.to_string())
    }

    pub fn to_string(&self) -> String {
        self.centroid_details.clone()
    }
}
