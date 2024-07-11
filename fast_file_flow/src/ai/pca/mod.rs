extern crate csv;
extern crate linfa;
extern crate linfa_reduction;
extern crate ndarray;
use crate::{ai::shared::Ranges, constants::path::PCA_IMAGE_RESULT};
use linfa::prelude::*;
use linfa_reduction::Pca;
use plotters::prelude::*;
use std::error::Error;

use super::shared::Shared;

#[derive(Debug, Clone)]
pub struct PrincipalComponentsAnalisys {
    explained_variance: String,
    explained_variance_ratio: String,
    singular_values: String,
    pub result_image_path: String,
    pub is_dirty: bool,
}
impl PrincipalComponentsAnalisys {
    pub fn new() -> Self {
        Self {
            explained_variance: String::default(),
            explained_variance_ratio: String::default(),
            singular_values: String::default(),
            result_image_path: String::default(),
            is_dirty: false,
        }
    }
    pub async fn pca_analysis(
        &mut self,
        column1: Vec<f64>,
        column2: Vec<f64>,
        embedding_size: usize,
    ) -> Result<String, Box<dyn Error>> {
        if self.is_dirty {
            println!("pca already exists");
            return Ok(self.to_string());
        }
        println!("Loading Data");
        let data = Shared::get_dataset_info(column1, column2);
        let dataset = DatasetBase::from(data);

        println!("Start Transforming");
        let pca = Pca::params(embedding_size).fit(&dataset)?;
        //let transformed_data = pca.predict(dataset);
        let transformed_data = pca.transform(dataset);

        println!("Get Ranges");

        let transformed_array = transformed_data.records().to_owned();
        if transformed_array.shape()[1] < embedding_size {
            return Err(format!(
                "The number of principal components is less than {}. Cannot be graphed",
                embedding_size
            )
            .into());
        }

        let min_x = transformed_array
            .column(0)
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);

        let max_x = transformed_array
            .column(0)
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        let min_y = transformed_array
            .column(1)
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);

        let max_y = transformed_array
            .column(1)
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        let x: Ranges = Ranges {
            max: max_x,
            min: min_x,
        };

        let y: Ranges = Ranges {
            max: max_y,
            min: min_y,
        };

        println!("Results");
        self.explained_variance = String::from(format!(
            "Explained variance: {:?}",
            pca.explained_variance()
        ));
        self.explained_variance_ratio = format!(
            "Explained variance ratio: {:?}",
            pca.explained_variance_ratio()
        );
        self.singular_values = format!("Singular values: {:?}", pca.singular_values());

        println!("Start Printing Image");
        let path = PCA_IMAGE_RESULT;

        let root = BitMapBackend::new(path, (1920, 768)).into_drawing_area();
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
        self.result_image_path = path.to_owned();
        self.is_dirty = true;

        Ok(self.to_string())
    }

    pub fn to_string(&self) -> String {
        format!(
            "{},
{},
{}",
            self.explained_variance, self.explained_variance_ratio, self.singular_values
        )
    }
}
