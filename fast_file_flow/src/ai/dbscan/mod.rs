extern crate linfa;
extern crate linfa_clustering;
extern crate ndarray;

use crate::ai::dbscan::linfa::dataset::Labels;
use crate::ai::shared::{Ranges, Shared};
use crate::constants::path::DBSCAN_IMAGE_RESULT;
use linfa::traits::Transformer;
use linfa::DatasetBase;
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
    pub result_image_path: String,
    pub is_dirty: bool,
}

impl DensityBaseClustering {
    pub fn new() -> Self {
        Self {
            noise_points: String::default(),
            cluster_points: String::default(),
            result: String::default(),
            result_image_path: String::default(),
            is_dirty: false,
        }
    }
    pub async fn dbscan_analysis(
        &mut self,
        column1: Vec<f64>,
        column2: Vec<f64>,
        tolerance: f64,
        min_points: usize,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if self.is_dirty {
            return Ok(self.to_string());
        }
        let data = Shared::get_dataset_info(column1, column2);
        let dataset = DatasetBase::from(data);
        let ndata = dataset.clone();

        println!("Start Clustering");
        let assigned_clusters =
            Dbscan::params_with(min_points, L2Dist, CommonNearestNeighbour::KdTree)
                .tolerance(tolerance)
                .transform(dataset)?;

        println!("Get Ranges");
        let (min_x, max_x) = assigned_clusters
            .records()
            .column(0)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
                (min.min(val), max.max(val))
            });

        let x: Ranges = Ranges {
            max: max_x,
            min: min_x,
        };
        let (min_y, max_y) = assigned_clusters
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

        for (i, point) in ndata.to_owned().records().outer_iter().enumerate() {
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
        self.result_image_path = path.to_owned();
        self.is_dirty = true;
        self.result = builder.string().unwrap();
        println!("{}", self.to_string());
        Ok(self.to_string())
    }

    pub fn to_string(&self) -> String {
        self.result.clone()
    }
}
