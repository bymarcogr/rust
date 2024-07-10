extern crate csv;
extern crate linfa;
extern crate linfa_reduction;
extern crate ndarray;
use crate::constants::path::PCA_IMAGE_RESULT;
use linfa::prelude::*;
use linfa_reduction::Pca;
use plotters::prelude::*;
use std::{error::Error, fmt::Write};

use super::shared::Shared;

#[derive(Debug, Clone)]
pub struct PrincipalComponentsAnalisys {
    explained_variance: String,
    explained_variance_ratio: String,
    singular_values: String,
}
impl PrincipalComponentsAnalisys {
    pub fn new() -> Self {
        Self {
            explained_variance: String::default(),
            explained_variance_ratio: String::default(),
            singular_values: String::default(),
        }
    }
    pub async fn pca_analysis(
        &mut self,
        column1: Vec<f64>,
        column2: Vec<f64>,
        embedding_size: usize,
    ) -> Result<String, Box<dyn Error>> {
        let (dataset, x, y) = Shared::get_dataset_info(column1, column2);

        let pca = Pca::params(embedding_size).fit(&dataset)?;
        //let transformed_data = pca.predict(dataset);
        let transformed_data = pca.transform(dataset);

        let path = PCA_IMAGE_RESULT;

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

        self.explained_variance = String::from(format!(
            "Explained variance: {:?}",
            pca.explained_variance()
        ));
        self.explained_variance_ratio =
            format!("Explained variance: {:?}", pca.explained_variance_ratio());
        self.singular_values = format!("Singular values: {:?}", pca.singular_values());

        Ok(self.to_string())
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        writeln!(out, "{}", self.explained_variance);
        writeln!(out, "{}", self.explained_variance_ratio);
        writeln!(out, "{}", self.singular_values);
        out
    }
}
