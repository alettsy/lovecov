use std::fmt::Write;
use std::fs;

use crate::{
    constants,
    entry::{
        data::{FolderData, ItemType},
        file_tree::FileTree,
    },
    output::OutputGenerator,
};

pub struct HtmlOutput;

impl OutputGenerator for HtmlOutput {
    fn generate(tree: &FileTree) {
        let mut output = String::from(HTML_TEMPLATE_START);
        Self::generate_html(&tree.root, &mut output);
        output.push_str(HTML_TEMPLATE_END);
        fs::write(constants::HTML_FILE, output).unwrap();
    }
}

impl HtmlOutput {
    fn generate_html(folder: &FolderData, output: &mut String) {
        for item in &folder.items {
            match item {
                ItemType::File(file) => {
                    writeln!(output, r#"<div class="file row">"#).unwrap();
                    writeln!(output, r#"<div class="name">{}</div>"#, file.name).unwrap();
                    Self::write_line_entry(output, file.line_count, file.line_hit_count);
                    Self::write_function_entry(
                        output,
                        file.function_count,
                        file.function_hit_count,
                    );
                    writeln!(output, "</div>").unwrap();
                }
                ItemType::Folder(folder) => {
                    writeln!(output, "<details>").unwrap();
                    writeln!(output, r#"<summary class="row">"#).unwrap();
                    writeln!(output, r#"<div class="name">{}</div>"#, folder.name).unwrap();
                    Self::write_line_entry(output, folder.line_count, folder.line_hit_count);
                    Self::write_function_entry(
                        output,
                        folder.function_count,
                        folder.function_hit_count,
                    );
                    writeln!(output, "</summary>").unwrap();
                    Self::generate_html(folder, output);
                    writeln!(output, "</details>").unwrap();
                }
            }
        }
    }

    fn write_line_entry(output: &mut String, lines_total: u32, lines_hit: u32) {
        let percentage_calc: f32 = (lines_hit as f32 / lines_total as f32) * 100.0;

        let percentage = if percentage_calc.is_nan() {
            0.0
        } else {
            percentage_calc
        };

        writeln!(
            output,
            r#"<div class="data-entry">{} / {}</div>"#,
            lines_hit, lines_total
        )
        .unwrap();
        writeln!(
            output,
            r#"<div class="data-entry">{:.2} %</div>"#,
            percentage
        )
        .unwrap();
    }

    fn write_function_entry(output: &mut String, functions_total: u32, functions_hit: u32) {
        let percentage_calc: f32 = (functions_hit as f32 / functions_total as f32) * 100.0;

        let percentage = if percentage_calc.is_nan() {
            0.0
        } else {
            percentage_calc
        };

        writeln!(
            output,
            r#"<div class="data-entry">{} / {}</div>"#,
            functions_hit, functions_total
        )
        .unwrap();
        writeln!(
            output,
            r#"<div class="data-entry">{:.2} %</div>"#,
            percentage
        )
        .unwrap();
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

            .data-entry {
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
            <div class="data-entry">Lines</div>
            <div class="data-entry">Line Coverage</div>
            <div class="data-entry">Functions</div>
            <div class="data-entry">Function Coverage</div>
        </div>
        <hr />"#;

const HTML_TEMPLATE_END: &str = r#"</div>
    </body>
</html>"#;
