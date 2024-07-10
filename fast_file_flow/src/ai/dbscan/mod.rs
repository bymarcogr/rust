extern crate linfa;
extern crate linfa_clustering;
extern crate ndarray;

use crate::ai::dbscan::linfa::dataset::Labels;
use crate::constants::path::DBSCAN_IMAGE_RESULT;
use linfa::dataset::{DatasetBase, Records};
use linfa::traits::Transformer;
use linfa_clustering::Dbscan;
use linfa_nn::distance::L2Dist;
use linfa_nn::CommonNearestNeighbour;
use ndarray::Array2;
use plotters::prelude::*;
use string_builder::Builder;

#[derive(Debug, Clone)]
pub struct DensityBaseClustering {}

impl DensityBaseClustering {
    pub async fn dbscan_analysis(
        column1: Vec<f64>,
        column2: Vec<f64>,
        eps: f64,
        min_points: usize,
    ) -> Result<String, Box<dyn std::error::Error>> {
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
        // default implementation
        // let clusters = Dbscan::params(min_points)

        println!(
            "Clustering #{} data points grouped in 4 clusters of {} points each",
            dataset.nsamples(),
            min_points
        );

        println!("Start Clustering");
        let assigned_clusters =
            Dbscan::params_with(min_points, L2Dist, CommonNearestNeighbour::KdTree)
                .tolerance(eps)
                .transform(dataset.clone())
                .unwrap();

        // sigle target dataset
        let label_count = assigned_clusters.label_count().remove(0);

        let mut builder = Builder::default();

        builder.append("Result: ");
        for (label, count) in label_count {
            match label {
                None => builder.append(format!(" - {} noise points", count)),
                Some(i) => builder.append(format!(" - {} points in cluster {}", count, i)),
            }
        }

        let path = DBSCAN_IMAGE_RESULT;
        let output_dir = std::path::Path::new("output");

        // Crear el directorio si no existe
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).expect(DBSCAN_IMAGE_RESULT);
        }
        // Visualización de los clusters
        let root = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("DBSCAN Clustering", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

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
}
