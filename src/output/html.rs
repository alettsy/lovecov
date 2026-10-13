use std::fmt::Write;
use std::fs;

use crate::{
    entry::{
        data::{FolderData, ItemType},
        file_tree::FileTree,
    },
    output::OutputGenerator,
};

pub struct HtmlOutput;

impl OutputGenerator for HtmlOutput {
    fn generate(tree: &FileTree) {
        let mut output = String::from(HTML_TEMPLATE);
        Self::generate_html(&tree.root, &mut output);
        output.push_str(HTML_TEMPLATE_END);
        fs::write("coverage.html", output).unwrap();
    }
}

impl HtmlOutput {
    fn generate_html(folder: &FolderData, output: &mut String) {
        for item in &folder.items {
            match item {
                ItemType::File(file) => {
                    let line_percentage: f32 =
                        (file.line_hit_count as f32 / file.line_count as f32) * 100.0;

                    let function_percentage: f32 =
                        (file.function_hit_count as f32 / file.function_count as f32) * 100.0;

                    writeln!(output, r#"<div class="file row">"#).unwrap();
                    writeln!(output, r#"<div class="name">{}</div>"#, file.name).unwrap();
                    writeln!(
                        output,
                        r#"<div class="data-entry">{} / {}</div>"#,
                        file.line_hit_count, file.line_count
                    )
                    .unwrap();
                    writeln!(
                        output,
                        r#"<div class="data-entry">{} %</div>"#,
                        line_percentage
                    )
                    .unwrap();
                    writeln!(
                        output,
                        r#"<div class="data-entry">{} / {}</div>"#,
                        file.function_hit_count, file.function_count
                    )
                    .unwrap();
                    writeln!(
                        output,
                        r#"<div class="data-entry">{} %</div>"#,
                        function_percentage
                    )
                    .unwrap();
                    writeln!(output, "</div>").unwrap();
                }
                ItemType::Folder(folder) => {
                    let line_percentage: f32 =
                        (folder.line_hit_count as f32 / folder.line_count as f32) * 100.0;

                    let function_percentage: f32 =
                        (folder.function_hit_count as f32 / folder.function_count as f32) * 100.0;

                    writeln!(output, "<details>").unwrap();
                    writeln!(output, r#"<summary class="row">"#).unwrap();
                    writeln!(output, r#"<div class="name">{}</div>"#, folder.name).unwrap();
                    writeln!(
                        output,
                        r#"<div class="data-entry">{} / {}</div>"#,
                        folder.line_hit_count, folder.line_count
                    )
                    .unwrap();
                    writeln!(
                        output,
                        r#"<div class="data-entry">{} %</div>"#,
                        line_percentage
                    )
                    .unwrap();
                    writeln!(
                        output,
                        r#"<div class="data-entry">{} / {}</div>"#,
                        folder.function_hit_count, folder.function_count
                    )
                    .unwrap();
                    writeln!(
                        output,
                        r#"<div class="data-entry">{} %</div>"#,
                        function_percentage
                    )
                    .unwrap();
                    writeln!(output, "</summary>").unwrap();
                    Self::generate_html(folder, output);
                    writeln!(output, "</details>").unwrap();
                }
            }
        }
    }
}

const HTML_TEMPLATE: &str = r#"<!doctype html>
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
                padding: 0.5rem;
                padding-right: 0;
            }

            summary {
                font-weight: bold;
                list-style: none;
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
