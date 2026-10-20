use std::io::Write;

use crate::{
    errors::LovecovError,
    io::file_output::FileOutput,
    output::OutputGenerator,
    parser::{file_data::FolderData, file_tree::FileTree},
    utils,
};

pub struct CsvOutput {
    pub file_out: FileOutput,
}

impl OutputGenerator for CsvOutput {
    fn generate(&self, tree: &FileTree) -> Result<(), LovecovError> {
        let mut writer = self.file_out.open_write()?;

        writeln!(writer, "{}", FILE_HEADER).map_err(|_| LovecovError::FileWrite)?;
        Self::generate_csv(&tree.root, &mut writer).map_err(|_| LovecovError::FileWrite)?;

        Ok(())
    }
}

impl CsvOutput {
    fn generate_csv(folder: &FolderData, output: &mut dyn Write) -> Result<(), ()> {
        for file in folder.files.values() {
            let func_percent =
                utils::calc_percent(file.function_hit_count as f32, file.function_count as f32);
            let line_percent =
                utils::calc_percent(file.line_hit_count as f32, file.line_count as f32);
            let mut path = file.path.join("/");
            path.push('/');
            path.push_str(&file.name);

            writeln!(
                output,
                "{},{},{},{},{},{},{}",
                path,
                file.line_count,
                file.line_hit_count,
                line_percent,
                file.function_count,
                file.function_hit_count,
                func_percent
            )
            .map_err(|_| ())?;
        }

        for folder in folder.folders.values() {
            Self::generate_csv(folder, output)?;
        }

        Ok(())
    }
}

const FILE_HEADER: &str =
    "path,line_count,lines_hit,line_percent,function_count,function_hit,function_percent";
