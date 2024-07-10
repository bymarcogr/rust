extern crate linfa;
extern crate linfa_clustering;
extern crate ndarray;

use crate::ai::dbscan::linfa::dataset::Labels;
use crate::ai::shared::Shared;
use crate::constants::path::DBSCAN_IMAGE_RESULT;
use linfa::traits::Transformer;
use linfa_clustering::Dbscan;
use linfa_nn::distance::L2Dist;
use linfa_nn::CommonNearestNeighbour;
use plotters::prelude::*;
use string_builder::Builder;

#[derive(Debug, Clone)]
pub struct DensityBaseClustering {
    pub noise_points: String,
    pub cluster_points: String,
    result: String,
}

impl DensityBaseClustering {
    pub fn new() -> Self {
        Self {
            noise_points: String::default(),
            cluster_points: String::default(),
            result: String::default(),
        }
    }
    pub async fn dbscan_analysis(
        column1: Vec<f64>,
        column2: Vec<f64>,
        eps: f64,
        min_points: usize,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let (dataset, x, y) = Shared::get_dataset_info(column1, column2);

        println!("Start Clustering");
        let assigned_clusters =
            Dbscan::params_with(min_points, L2Dist, CommonNearestNeighbour::KdTree)
                .tolerance(eps)
                .transform(dataset.clone())
                .unwrap();

        // sigle target dataset
        let label_count = assigned_clusters.label_count().remove(0);

        let mut builder = Builder::default();

        builder.append("Result: \n");
        for (label, count) in label_count {
            match label {
                None => builder.append(format!(" - {} noise points\n", count)),
                Some(i) => builder.append(format!(" - {} points in cluster {}\n", count, i)),
            }
        }
        println!("Start Printing Image");
        let path = DBSCAN_IMAGE_RESULT;

        let root = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("DBSCAN Clustering", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x.min..x.max, y.min..y.max)?;

        chart.configure_mesh().draw()?;

        for (i, point) in assigned_clusters.records().outer_iter().enumerate() {
            let cluster = assigned_clusters.targets()[i];
            let color = match cluster {
                Some(0) => RED,
                Some(1) => BLUE,
                Some(2) => GREEN,
                _ => BLACK,
            };
            chart.draw_series(std::iter::once(Circle::new(
                (point[0], point[1]),
                5,
                color.filled(),
            )))?;
        }

        let result = builder.string().unwrap();
        println!("{}", &result);
        Ok(result)
    }

    pub fn to_string(&self) -> String {
        self.result.clone()
    }
}
