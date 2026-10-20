use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone)]
pub enum GeneratorType {
    Summary,
    Html,
    Csv,
}
