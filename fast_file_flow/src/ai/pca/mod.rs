extern crate csv;
extern crate linfa;
extern crate linfa_reduction;
extern crate ndarray;
use crate::constants::path::PCA_IMAGE_RESULT;
use linfa::prelude::*;
use linfa_reduction::Pca;
use plotters::prelude::*;
use std::error::Error;

use super::shared::Shared;

#[derive(Debug, Clone)]
pub struct PrincipalComponentsAnalisys {}
impl PrincipalComponentsAnalisys {
    pub async fn pca_analysis(
        column1: Vec<f64>,
        column2: Vec<f64>,
        embedding_size: usize,
    ) -> Result<(), Box<dyn Error>> {
        let (dataset, x, y) = Shared::get_dataset_info(column1, column2);

        let pca = Pca::params(embedding_size).fit(&dataset)?;
        let transformed_data = pca.transform(dataset);

        let path = PCA_IMAGE_RESULT;

        // Visualización de los datos transformados
        let root = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("PCA Result", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x.min..x.max, y.min..y.max)?;

        chart.configure_mesh().draw()?;

        for point in transformed_data.records().outer_iter() {
            chart.draw_series(std::iter::once(Circle::new(
                (point[0], point[1]),
                5,
                RED.filled(),
            )))?;
        }

        Ok(())
    }
}
