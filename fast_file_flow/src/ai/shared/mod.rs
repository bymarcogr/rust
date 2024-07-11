extern crate linfa;
extern crate linfa_clustering;
extern crate ndarray;

use linfa::dataset::DatasetBase;
use ndarray::{Array2, ArrayBase, Dim, OwnedRepr};

#[derive(Debug, Clone)]

pub struct Ranges {
    pub max: f64,
    pub min: f64,
}

pub struct Shared {}
impl Shared {
    pub fn get_dataset_info(
        column1: Vec<f64>,
        column2: Vec<f64>,
    ) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
        Self::create_output_folder();

        let data: Vec<Vec<f64>> = vec![column1, column2];

        let data: Vec<Vec<f64>> = (0..data[0].len())
            .map(|i| data.iter().map(|col| col[i]).collect())
            .collect();

        let data: Array2<f64> = Array2::from_shape_vec(
            (data.len(), data[0].len()),
            data.into_iter().flatten().collect(),
        )
        .unwrap();

        data
    }

    pub fn get_ranges(
        dataset: &DatasetBase<
            ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
            ArrayBase<OwnedRepr<Option<usize>>, Dim<[usize; 1]>>,
        >, /*
           DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, ArrayBase<OwnedRepr<Option<usize>>, Dim<[usize; 1]>>>
           DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, ArrayBase<OwnedRepr<Option<usize>>, Dim<[usize; 1]>>>
           DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, ArrayBase<OwnedRepr<Option<usize>>, Dim<[usize; 1]>>>




                   */
    ) -> (Ranges, Ranges) {
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
        (x, y)
    }

    fn create_output_folder() {
        let output = String::from("output");
        let output_dir = std::path::Path::new(&output);

        if !output_dir.exists() {
            let message = format!("Could not create {} directory", output.as_str()).to_string();
            std::fs::create_dir_all(output_dir).expect(message.as_str());
        }
    }
}
