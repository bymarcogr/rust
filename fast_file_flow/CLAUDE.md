# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Fast File Flow** is a Rust-based GUI desktop application for data engineering tasks. It provides a visual interface for loading, filtering, processing, and analyzing CSV/JSON files with integrated ML capabilities.

## Build Commands

```bash
# Build the project
cargo build

# Run in development
cargo run

# Build release version
cargo build --release

# Check for compile errors without building
cargo check

# Clean build artifacts
cargo clean
```

## Architecture

The application follows an **Elm-style architecture** using the Iced GUI framework:

### Core Architecture Pattern

1. **Model-Update-View Pattern** (src/fast_file_flow/mod.rs:50-77)
   - `FastFileFlow` struct holds all application state
   - `FastFileFlowMessage` enum defines all possible state transitions (src/fast_file_flow/mod.rs:79-125)
   - `update()` method handles message dispatch
   - `view()` method renders the UI based on current state

2. **Page-based Routing**
   - `Page` enum tracks the current view (Home, Preview, AI, etc.)
   - Navigation happens via `FastFileFlowMessage::Router(Page)`

3. **Async Data Loading** (src/stored_file/mod.rs:35-463)
   - `StoredFile` struct represents a loaded dataset
   - Uses `tokio` + `csv-async` for non-blocking file I/O
   - Uses `rayon` for parallel CPU-intensive operations (ML, statistics)

### Key Modules

| Module | Purpose |
|--------|---------|
| `src/fast_file_flow/` | Main application state, message handling, view rendering |
| `src/stored_file/` | File loading, column/row storage, data access |
| `src/ai/` | ML analysis: K-Means, PCA, DBSCAN, Linear Regression (via linfa) |
| `src/dynamictable/` | Table UI components (columns, rows, scrollable views) |
| `src/stadistics/` | Statistical analysis (data classification: Qualitative/Quantitative) |
| `src/save_options/` | Filter and process options for data export |
| `src/correlation_analysis/` | Correlation calculations between columns |

### Data Flow

1. File selected → `StoredFile::new()` async loads columns (headers) and first 50 rows
2. Full columns loaded on-demand via `StoredFile::get_full_column()` for ML/statistics
3. ML operations (`get_kmeans()`, `get_pca_analysis()`, etc.) return results as images/text
4. Export generates timestamped files in `./output/`

## File Formats

- **Primary**: CSV (detected via csv-async parsing)
- **Secondary**: JSON (basic detection via serde_json)
- **Project**: `.ffflow` custom format for saving app state

## Key Dependencies

- **iced 0.12** - GUI framework with tokio support
- **tokio** - Async runtime for file I/O
- **csv-async** - Async CSV parsing
- **linfa** - ML algorithms (clustering, linear regression, PCA)
- **ndarray + rayon** - Parallel numerical computing
- **plotters** - Chart generation for AI results

## Configuration Files

- `config.ffflow` - Stores last loaded file path and column settings
- `.vscode/tasks.json` - VS Code build task (`cargo build`)
- `.vscode/launch.json` - Debug configuration (requires LLDB)

## Development Notes

- Custom icon font loaded from `src/resources/fonts/iced-fff.ttf`
- Application window is fixed-size (APP_WIDTH x APP_HEIGHT from constants::sizes)
- Statistics are computed lazily when a column is first selected
- AI analysis generates PNG plots saved to `./output/`
