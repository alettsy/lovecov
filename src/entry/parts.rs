#[derive(Clone)]
pub struct Line {
    pub number: u32,
    pub execute_count: u32,
}

impl Line {
    pub fn new(number: u32, execute_count: u32) -> Self {
        Self {
            number,
            execute_count,
        }
    }

    pub fn merge_with(&self, other: &Line) -> Self {
        Self {
            number: self.number,
            execute_count: self.execute_count + other.execute_count,
        }
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub line_start: u32,
    pub execute_count: u32,
}

impl Function {
    pub fn new(name: &str, line_start: u32) -> Self {
        Self {
            name: name.to_string(),
            line_start,
            execute_count: 0,
        }
    }

    pub fn merge_with(&self, other: &Function) -> Self {
        Self {
            name: self.name.clone(),
            line_start: self.line_start,
            execute_count: self.execute_count + other.execute_count,
        }
    }
}