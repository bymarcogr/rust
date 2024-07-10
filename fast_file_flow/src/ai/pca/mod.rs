extern crate csv;
extern crate linfa;
extern crate linfa_reduction;
extern crate ndarray;

use crate::constants::english::AI_IMAGE_RESULT_FOLDER;
use crate::constants::path::PCA_IMAGE_RESULT;

use linfa::prelude::*;
use linfa::DatasetBase;
use linfa_reduction::Pca;
use ndarray::Array2;
use plotters::prelude::*;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct PrincipalComponentsAnalisys {}
impl PrincipalComponentsAnalisys {
    pub async fn pca_analysis(
        column1: Vec<f64>,
        column2: Vec<f64>,
        embedding_size: usize,
    ) -> Result<(), Box<dyn Error>> {
        let data: Vec<Vec<f64>> = vec![column1, column2];

        let data: Vec<Vec<f64>> = (0..data[0].len())
            .map(|i| data.iter().map(|col| col[i]).collect())
            .collect();

        let data: Array2<f64> = Array2::from_shape_vec(
            (data.len(), data[0].len()),
            data.into_iter().flatten().collect(),
        )
        .unwrap();

        // Aplicar PCA\
        let dataset = DatasetBase::from(data);
        let pca = Pca::params(embedding_size).fit(&dataset)?;
        let transformed_data = pca.transform(dataset);

        let path = PCA_IMAGE_RESULT;
        let output_dir = std::path::Path::new("output");

        // Crear el directorio si no existe
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).expect(AI_IMAGE_RESULT_FOLDER);
        }
        // Visualización de los datos transformados
        let root = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("PCA Result", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

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
