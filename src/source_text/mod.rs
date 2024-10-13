pub struct SourceText {
    input: String,
}

impl SourceText {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn from_file(filename: &str) -> Self {
        match std::fs::read_to_string(filename) {
            Ok(input) => Self { input },
            _ => Self {
                input: "".to_string(),
            },
        }
    }

    pub fn get_location(&self, index: usize) -> (String, usize) {
        let line_number = self.get_linenumber(index);
        (self.get_line(line_number - 1), self.get_column(index))
    }

    pub fn get_column(&self, index: usize) -> usize {
        if self.input[0..index].lines().count() > 0 {
            self.input[0..index].lines().last().unwrap().len()
        } else {
            index
        }
    }

    pub fn get_linenumber(&self, index: usize) -> usize {
        if index == 0 {
            return 1;
        }
        self.input[0..index].lines().count()
    }

    pub fn get_line(&self, row: usize) -> String {
        if self.input.lines().count() > 0 {
            self.input.lines().nth(row).unwrap().to_string()
        } else {
            self.input.clone()
        }
    }
}
