extern crate linfa;
extern crate ndarray;
extern crate ndarray_rand;
extern crate plotters;

use linfa::prelude::*;
use linfa::traits::{Fit, Predict};
use linfa_linear::LinearRegression;
use ndarray::{Array1, Array2};
use plotters::prelude::*;

use crate::ai::shared::Ranges;
use crate::constants::path::LR_IMAGE_RESULT;

#[derive(Debug, Clone)]
pub struct LnRegression {
    interceptor: String,
    regression_coeficient: String,
    r2: String,
    pub result_image_path: String,
    pub is_dirty: bool,
}

impl LnRegression {
    pub fn new() -> Self {
        Self {
            result_image_path: String::default(),
            is_dirty: false,
            interceptor: String::default(),
            regression_coeficient: String::default(),
            r2: String::default(),
        }
    }

    pub async fn get_prediction(
        &mut self,
        column1: Vec<f64>,
        column2: Vec<f64>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if self.is_dirty {
            println!("linear regression already exists");
            return Ok(self.to_string());
        }

        println!("Loading Data");
        let col1_array: Array2<f64> = Array2::from_shape_vec((column1.len(), 1), column1)?;
        let col2_array: Array1<f64> = Array1::from_vec(column2);

        let dataset = DatasetBase::new(col1_array.clone(), col2_array.clone());

        println!("Start Predict");
        let model = LinearRegression::default().fit(&dataset)?;
        let prediction_dataset = model.predict(dataset.records());

        println!("Get Ranges");

        let min_x = col1_array
            .column(0)
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let max_x = col1_array
            .column(0)
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let max_y = col2_array
            .iter()
            .cloned()
            .chain(prediction_dataset.iter().cloned())
            .fold(f64::INFINITY, f64::min);
        let min_y = col2_array
            .iter()
            .cloned()
            .chain(prediction_dataset.iter().cloned())
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

        let intercept = model.intercept();
        let coefficients = model.params();
        let ss_total: f64 = col2_array
            .iter()
            .map(|&yi| (yi - col2_array.mean().unwrap()).powi(2))
            .sum();
        let ss_res: f64 = col2_array
            .iter()
            .zip(prediction_dataset.iter())
            .map(|(&yi, &y_pred_i)| (yi - y_pred_i).powi(2))
            .sum();
        let r2 = 1.0 - (ss_res / ss_total);

        self.interceptor = format!("Interceptot: {:.4}", intercept);
        self.regression_coeficient = format!("Regression coeficient: {:.4}", coefficients[0]);
        self.r2 = format!("R²: {:.4}", r2);

        println!("Start Printing Image");
        let path = LR_IMAGE_RESULT;

        let root = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Linear Regression", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x.min..x.max, y.min..y.max)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(
            col1_array
                .outer_iter()
                .zip(col2_array.iter())
                .map(|(x, y)| Circle::new((x[0], *y), 5, BLUE.filled())),
        )?;

        chart
            .draw_series(LineSeries::new(
                col1_array
                    .column(0)
                    .iter()
                    .cloned()
                    .zip(prediction_dataset.iter().cloned()),
                &RED,
            ))?
            .label("Linear Regression")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .draw_series(LineSeries::new(
                col1_array
                    .outer_iter()
                    .zip(prediction_dataset.iter())
                    .map(|(x, y)| (x[0], *y)),
                &RED,
            ))?
            .label("Linear Regression")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        self.result_image_path = path.to_owned();
        self.is_dirty = true;
        println!("{}", self.to_string());

        Ok(self.to_string())
    }

    fn to_string(&self) -> String {
        format!(
            "{},
{},
{}",
            self.interceptor, self.regression_coeficient, self.r2
        )
    }
}
