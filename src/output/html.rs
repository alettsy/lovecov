use std::io::Write;

use crate::{
    errors::LovecovError,
    io::file_output::FileOutput,
    output::OutputGenerator,
    parser::{file_data::FolderData, file_tree::FileTree},
    utils,
};

pub struct HtmlOutput {
    file_out: FileOutput,
    use_bars: bool,
}

impl OutputGenerator for HtmlOutput {
    fn generate(&self, tree: &FileTree) -> Result<(), LovecovError> {
        let mut writer = self.file_out.open_write()?;
        writeln!(writer, "{}", HTML_TEMPLATE_START).map_err(|_| LovecovError::FileWrite)?;

        self.generate_html(&tree.root, &mut writer)
            .map_err(|_| LovecovError::FileWrite)?;
        writeln!(writer, "{}", HTML_TEMPLATE_END).map_err(|_| LovecovError::FileWrite)?;

        Ok(())
    }
}

impl HtmlOutput {
    pub fn new(file_out: FileOutput, use_bars: bool) -> Self {
        Self { file_out, use_bars }
    }

    fn generate_html(&self, folder: &FolderData, output: &mut dyn Write) -> Result<(), ()> {
        let mut folder_names: Vec<_> = folder.folders.keys().cloned().collect();
        folder_names.sort();

        for name in folder_names {
            let folder = folder.folders.get(&name).unwrap();

            writeln!(output, "<details>").map_err(|_| ())?;
            writeln!(output, r#"<summary class="row">"#).map_err(|_| ())?;
            writeln!(output, r#"<div class="name">{}</div>"#, folder.name).map_err(|_| ())?;
            self.write_line_data(output, folder.line_count, folder.line_hit_count)?;
            self.write_function_data(output, folder.function_count, folder.function_hit_count)?;
            writeln!(output, "</summary>").map_err(|_| ())?;
            self.generate_html(folder, output)?;
            writeln!(output, "</details>").map_err(|_| ())?;
        }

        let mut file_names: Vec<_> = folder.files.keys().cloned().collect();
        file_names.sort();

        for name in file_names {
            let file = folder.files.get(&name).unwrap();

            writeln!(output, r#"<div class="file row">"#).map_err(|_| ())?;
            writeln!(output, r#"<div class="name">{}</div>"#, file.name).map_err(|_| ())?;
            self.write_line_data(output, file.line_count, file.line_hit_count)?;
            self.write_function_data(output, file.function_count, file.function_hit_count)?;
            writeln!(output, "</div>").map_err(|_| ())?;
        }

        Ok(())
    }

    fn write_line_data(
        &self,
        output: &mut dyn Write,
        lines_total: u32,
        lines_hit: u32,
    ) -> Result<(), ()> {
        let percentage = utils::calc_percent(lines_hit as f32, lines_total as f32);
        let color = Self::get_color_from_percent(percentage, lines_total);

        writeln!(
            output,
            r#"<div class="data-record {}">{} / {}</div>"#,
            color, lines_hit, lines_total
        )
        .map_err(|_| ())?;

        if self.use_bars {
            writeln!(
                output,
                r#"<div class="data-record"><progress value={} min="0" max="100" class="progress {}" /></div>"#,
                percentage, color
            )
            .map_err(|_| ())?;
        } else {
            writeln!(
                output,
                r#"<div class="data-record {}">{:.2} %</div>"#,
                color, percentage
            )
            .map_err(|_| ())?;
        }

        Ok(())
    }

    fn write_function_data(
        &self,
        output: &mut dyn Write,
        functions_total: u32,
        functions_hit: u32,
    ) -> Result<(), ()> {
        let percentage = utils::calc_percent(functions_hit as f32, functions_total as f32);
        let color = Self::get_color_from_percent(percentage, functions_total);

        writeln!(
            output,
            r#"<div class="data-record {}">{} / {}</div>"#,
            color, functions_hit, functions_total
        )
        .map_err(|_| ())?;

        if self.use_bars {
            writeln!(
                output,
                r#"<div class="data-record"><progress value={} min="0" max="100" class="progress {}" /></div>"#,
                percentage, color
            )
            .map_err(|_| ())?;
        } else {
            writeln!(
                output,
                r#"<div class="data-record {}">{:.2} %</div>"#,
                color, percentage
            )
            .map_err(|_| ())?;
        }

        Ok(())
    }

    fn get_color_from_percent(percent: f32, denominator: u32) -> String {
        if denominator == 0 {
            "non-existent".into()
        } else if percent >= 80.0 {
            "good".into()
        } else if percent >= 50.0 {
            "ok".into()
        } else {
            "bad".into()
        }
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

                --good-color: #2dc937;
                --ok-color: #db7b2b;
                --bad-color: #cc3232;
                --grey-color: #c4c4c4;
            }

            hr {
                margin-bottom: 2px;
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
                color: #06b6d4;
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

            .good {
                color: var(--good-color);
            }

            .ok {
                color: var(--ok-color);
            }

            .bad {
                color: var(--bad-color);
            }

            .non-existent {
                color: var(--grey-color);
            }

            .progress {
                max-width: 100px;
                background-color: transparent;
                border-radius: 4px;
                appearance: none;
                -webkit-appearance: none;
                border: 1px solid var(--grey-color);
                height: 18px;
            }

            @-moz-document url-prefix() {
              .progress {
                height: 16px;
              }
            }

            .progress::-webkit-progress-bar {
              background-color: transparent;
              border-radius: 4px;
            }

            .good.progress::-webkit-progress-value {
                background: var(--good-color);
                border-radius: 4px;
            }

            .good.progress::-moz-progress-bar {
                background: var(--good-color);
            }

            .ok.progress::-webkit-progress-value {
                background: var(--ok-color);
                border-radius: 4px;
            }

            .ok.progress::-moz-progress-bar {
                background: var(--ok-color);
            }

            .bad.progress::-webkit-progress-value {
                background: var(--bad-color);
                border-radius: 4px;
            }

            .bad.progress::-moz-progress-bar {
                background: var(--bad-color);
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
