pub mod dbscan;
pub mod k_means;
pub mod pca;
pub mod shared;

#[derive(Debug, Clone)]
pub enum AiModel {
    KMeans = 1,
    PCA = 2,
    DbScan = 3,
}
