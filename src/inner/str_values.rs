pub trait StrValues {
    fn join_strings(&self, separator: &str) -> String;
    fn string_count(&self) -> usize;
    fn string_at(&self, index: usize) -> &str;
}

impl StrValues for &[&str] {
    fn join_strings(&self, separator: &str) -> String {
        self.join(separator)
    }

    fn string_count(&self) -> usize {
        self.len()
    }

    fn string_at(&self, index: usize) -> &str {
        self[index]
    }
}

impl StrValues for Vec<String> {
    fn join_strings(&self, separator: &str) -> String {
        self.join(separator)
    }

    fn string_count(&self) -> usize {
        self.len()
    }

    fn string_at(&self, index: usize) -> &str {
        self[index].as_str()
    }
}

impl StrValues for Vec<&str> {
    fn join_strings(&self, separator: &str) -> String {
        self.join(separator)
    }

    fn string_count(&self) -> usize {
        self.len()
    }

    fn string_at(&self, index: usize) -> &str {
        self[index]
    }
}

impl StrValues for String {
    fn join_strings(&self, _separator: &str) -> String {
        self.clone()
    }

    fn string_count(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            1
        }
    }

    fn string_at(&self, _index: usize) -> &str {
        self.as_str()
    }
}

impl StrValues for &String {
    fn join_strings(&self, separator: &str) -> String {
        (*self).join_strings(separator)
    }

    fn string_count(&self) -> usize {
        (*self).string_count()
    }

    fn string_at(&self, index: usize) -> &str {
        (*self).string_at(index)
    }
}

impl StrValues for &str {
    fn join_strings(&self, _separator: &str) -> String {
        self.to_string()
    }

    fn string_count(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            1
        }
    }

    fn string_at(&self, _index: usize) -> &str {
        self
    }
}
