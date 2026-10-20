use crate::{
    cli::{args::LovecovArgs, generator_type::GeneratorType},
    constants,
    errors::LovecovError,
    io::file_output::FileOutput,
    output::{OutputGenerator, csv::CsvOutput, html::HtmlOutput, summary::SummaryOutput},
    parser::file_tree::FileTree,
};

pub enum Generator {
    Summary,
    Html(FileOutput),
    Csv(FileOutput),
}

impl Generator {
    pub fn from_type_and_args(args: LovecovArgs, gen_type: GeneratorType) -> Self {
        match gen_type {
            GeneratorType::Summary => Generator::Summary,
            GeneratorType::Html => Generator::Html(FileOutput::from_path_and_default(
                args.output_path,
                constants::HTML_FILE.to_string(),
            )),
            GeneratorType::Csv => Generator::Csv(FileOutput::from_path_and_default(
                args.output_path,
                constants::CSV_FILE.to_string(),
            )),
        }
    }

    pub fn generate(&self, tree: &FileTree) -> Result<(), LovecovError> {
        match self {
            Generator::Summary => SummaryOutput {}.generate(tree)?,
            Generator::Html(file_out) => {
                let file_out = file_out.clone();
                let out = HtmlOutput { file_out };
                out.generate(tree)?
            }
            Generator::Csv(file_out) => {
                let file_out = file_out.clone();
                let out = CsvOutput { file_out };
                out.generate(tree)?
            }
        }

        Ok(())
    }
}
