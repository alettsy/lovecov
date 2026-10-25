use crate::{
    errors::LovecovError,
    output::OutputGenerator,
    parser::{file_data::FolderData, file_tree::FileTree},
    utils::percent::calc_percent,
};

pub struct SummaryOutput;

impl OutputGenerator for SummaryOutput {
    fn generate(&self, tree: &FileTree) -> Result<(), LovecovError> {
        let mut summary = SummaryOutputData::default();
        Self::generate_summary(&tree.root, &mut summary);

        let line_percentage: f32 =
            calc_percent(summary.line_hit_total as f32, summary.line_total as f32);

        let function_percentage: f32 = calc_percent(
            summary.function_hit_total as f32,
            summary.function_total as f32,
        );

        println!("Summary:");
        println!(
            "Line coverage: {} / {} ({:.2} %)",
            summary.line_hit_total, summary.line_total, line_percentage
        );
        println!(
            "Function coverage: {} / {} ({:.2} %)",
            summary.function_hit_total, summary.function_total, function_percentage
        );

        Ok(())
    }
}

impl SummaryOutput {
    fn generate_summary(folder: &FolderData, summary: &mut SummaryOutputData) {
        for file in folder.files.values() {
            summary.line_total += file.line_count;
            summary.line_hit_total += file.line_hit_count;
            summary.function_total += file.function_count;
            summary.function_hit_total += file.function_hit_count;
        }

        for folder in folder.folders.values() {
            Self::generate_summary(folder, summary);
        }
    }
}

#[derive(Default)]
struct SummaryOutputData {
    line_total: u32,
    line_hit_total: u32,
    function_total: u32,
    function_hit_total: u32,
}
