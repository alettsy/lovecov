use crate::{
    file_tree::FileTree,
    files::{FolderData, ItemType},
};

pub enum Generator {
    Summary,
}

impl Generator {
    pub fn from_str(name: &str) -> Self {
        match name {
            "summary" => Generator::Summary,
            _ => panic!("Generator {} not recognised", name),
        }
    }

    pub fn generate(&self, tree: &FileTree) {
        match self {
            Generator::Summary => {
                let mut summary = SummaryOutput::default();
                Self::generate_summary(&tree.root, &mut summary);

                let line_percentage: f32 =
                    (summary.line_hit_total as f32 / summary.line_total as f32) * 100.0;

                let function_percentage: f32 =
                    (summary.function_hit_total as f32 / summary.function_total as f32) * 100.0;

                println!("Summary:");
                println!(
                    "Line coverage: {} / {} ({} %)",
                    summary.line_hit_total, summary.line_total, line_percentage
                );
                println!(
                    "Function coverage: {} / {} ({} %)",
                    summary.function_hit_total, summary.function_total, function_percentage
                );
            }
        }
    }

    fn generate_summary(folder: &FolderData, summary: &mut SummaryOutput) {
        for item in &folder.items {
            match item {
                ItemType::File(file) => {
                    summary.line_total += file.line_count;
                    summary.line_hit_total += file.line_hit_count;
                    summary.function_total += file.function_count;
                    summary.function_hit_total += file.function_hit_count;
                }
                ItemType::Folder(folder) => {
                    Self::generate_summary(folder, summary);
                }
            }
        }
    }
}

#[derive(Default)]
struct SummaryOutput {
    line_total: u32,
    line_hit_total: u32,
    function_total: u32,
    function_hit_total: u32,
}
