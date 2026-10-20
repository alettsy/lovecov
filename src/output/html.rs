use std::fmt::Write;

use crate::{
    errors::LovecovError,
    io::file_output::FileOutput,
    output::OutputGenerator,
    parser::{
        file_data::{FolderData, ItemType},
        file_tree::FileTree,
    },
    utils,
};

pub struct HtmlOutput {
    pub file_out: FileOutput,
}

impl OutputGenerator for HtmlOutput {
    fn generate(&self, tree: &FileTree) -> Result<(), LovecovError> {
        let mut output = String::from(HTML_TEMPLATE_START);
        Self::generate_html(&tree.root, &mut output).map_err(|_| LovecovError::FileWrite)?;
        output.push_str(HTML_TEMPLATE_END);

        self.file_out.write(&output)?;
        Ok(())
    }
}

impl HtmlOutput {
    fn generate_html(folder: &FolderData, output: &mut String) -> Result<(), ()> {
        for item in &folder.items {
            match item {
                ItemType::File(file) => {
                    writeln!(output, r#"<div class="file row">"#).map_err(|_| ())?;
                    writeln!(output, r#"<div class="name">{}</div>"#, file.name).map_err(|_| ())?;
                    Self::write_line_data(output, file.line_count, file.line_hit_count)?;
                    Self::write_function_data(
                        output,
                        file.function_count,
                        file.function_hit_count,
                    )?;
                    writeln!(output, "</div>").map_err(|_| ())?;
                }
                ItemType::Folder(folder) => {
                    writeln!(output, "<details>").map_err(|_| ())?;
                    writeln!(output, r#"<summary class="row">"#).map_err(|_| ())?;
                    writeln!(output, r#"<div class="name">{}</div>"#, folder.name)
                        .map_err(|_| ())?;
                    Self::write_line_data(output, folder.line_count, folder.line_hit_count)?;
                    Self::write_function_data(
                        output,
                        folder.function_count,
                        folder.function_hit_count,
                    )?;
                    writeln!(output, "</summary>").map_err(|_| ())?;
                    Self::generate_html(folder, output)?;
                    writeln!(output, "</details>").map_err(|_| ())?;
                }
            }
        }

        Ok(())
    }

    fn write_line_data(output: &mut String, lines_total: u32, lines_hit: u32) -> Result<(), ()> {
        let percentage = utils::calc_percent(lines_hit as f32, lines_total as f32);

        writeln!(
            output,
            r#"<div class="data-record">{} / {}</div>"#,
            lines_hit, lines_total
        )
        .map_err(|_| ())?;
        writeln!(
            output,
            r#"<div class="data-record">{:.2} %</div>"#,
            percentage
        )
        .map_err(|_| ())?;

        Ok(())
    }

    fn write_function_data(
        output: &mut String,
        functions_total: u32,
        functions_hit: u32,
    ) -> Result<(), ()> {
        let percentage = utils::calc_percent(functions_hit as f32, functions_total as f32);

        writeln!(
            output,
            r#"<div class="data-record">{} / {}</div>"#,
            functions_hit, functions_total
        )
        .map_err(|_| ())?;
        writeln!(
            output,
            r#"<div class="data-record">{:.2} %</div>"#,
            percentage
        )
        .map_err(|_| ())?;

        Ok(())
    }
}

const HTML_TEMPLATE_START: &str = r#"<!doctype html>
<html>
    <head>
        <title></title>
        <style>
            body {
                background-color: #252525;
                color: white;
                font-family: "Roboto";
            }

            details {
                margin-left: 20px;
            }

            summary {
                font-weight: bold;
                list-style: none;
                cursor: pointer;
            }

            details > summary::before {
                content: "+";
            }

            details[open] > summary::before {
                content: "-";
            }

            .wrapper {
                padding: 10px;
                width: calc(100% - 20px);
            }

            .wrapper > details:first-of-type {
                margin-left: 0;
            }

            .row {
                padding-top: 4px;
                display: flex;
                gap: 0.75rem;
                width: 100%;
                border-bottom: 1px solid grey;
                padding-bottom: 4px;
            }

            .name {
                flex-grow: 1;
            }

            .header {
                display: flex;
                width: 100%;
                gap: 0.75rem;
            }

            .data-record {
                width: 8.5rem;
                text-align: right;
            }

            .file > .name {
                padding-left: 40px;
            }
        </style>
    </head>
    <body>
    <div class="wrapper">
        <div class="header">
            <div class="name">Name</div>
            <div class="data-record">Lines</div>
            <div class="data-record">Line Coverage</div>
            <div class="data-record">Functions</div>
            <div class="data-record">Function Coverage</div>
        </div>
        <hr />"#;

const HTML_TEMPLATE_END: &str = r#"</div>
    </body>
</html>"#;
