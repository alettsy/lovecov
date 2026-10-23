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
    Html(FileOutput, bool),
    Csv(FileOutput),
}

impl Generator {
    pub fn from_type_and_args(args: &LovecovArgs, gen_type: GeneratorType) -> Self {
        match gen_type {
            GeneratorType::Summary => Generator::Summary,
            GeneratorType::Html => Generator::Html(
                FileOutput::from_path_and_default(
                    args.output_path.clone(),
                    constants::HTML_FILE.to_string(),
                ),
                args.html_use_bars,
            ),
            GeneratorType::Csv => Generator::Csv(FileOutput::from_path_and_default(
                args.output_path.clone(),
                constants::CSV_FILE.to_string(),
            )),
        }
    }

    pub fn generate(&self, tree: &FileTree) -> Result<(), LovecovError> {
        match self {
            Generator::Summary => SummaryOutput {}.generate(tree)?,
            Generator::Html(file_out, use_bars) => {
                let file_out = file_out.clone();
                let out = HtmlOutput::new(file_out, *use_bars);
                out.generate(tree)?
            }
            Generator::Csv(file_out) => {
                let file_out = file_out.clone();
                let out = CsvOutput::new(file_out);
                out.generate(tree)?
            }
        }

        Ok(())
    }
}
