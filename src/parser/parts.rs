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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_empty_functions() {
        let function1 = Function::new("function1", 3);
        let function2 = Function::new("function1", 3);

        let merged = function1.merge_with(&function2);

        assert!(merged.name == "function1");
        assert!(merged.line_start == 3);
        assert!(merged.execute_count == 0);
    }

    #[test]
    fn merge_populated_functions() {
        let function1 = Function {
            name: "function1".into(),
            line_start: 22,
            execute_count: 3,
        };

        let function2 = Function {
            name: "function1".into(),
            line_start: 22,
            execute_count: 7,
        };

        let merged = function1.merge_with(&function2);

        assert!(merged.name == "function1");
        assert!(merged.line_start == 22);
        assert!(merged.execute_count == 10);
    }

    #[test]
    fn merge_empty_lines() {
        let line1 = Line::new(10, 0);
        let line2 = Line::new(10, 0);

        let merged = line1.merge_with(&line2);

        assert!(merged.number == 10);
        assert!(merged.execute_count == 0);
    }

    #[test]
    fn merge_populated_lines() {
        let line1 = Line::new(10, 3);
        let line2 = Line::new(10, 7);

        let merged = line1.merge_with(&line2);

        assert!(merged.number == 10);
        assert!(merged.execute_count == 10);
    }
}
