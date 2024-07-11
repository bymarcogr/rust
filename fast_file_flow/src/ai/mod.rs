pub mod dbscan;
pub mod k_means;
pub mod linear_regression;
pub mod pca;
pub mod shared;

#[derive(Debug, Clone, Copy)]
pub enum AiModel {
    KMeans = 1,
    PCA = 2,
    DbScan = 3,
    LRegression = 4,
}
