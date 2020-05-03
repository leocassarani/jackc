use std::collections::HashMap;

pub struct Labeller {
    labels: HashMap<&'static str, u16>,
}

impl Labeller {
    pub fn new() -> Self {
        Labeller {
            labels: HashMap::new(),
        }
    }

    pub fn generate(&mut self, prefix: &'static str) -> String {
        let count = self.labels.entry(prefix).or_insert(0);
        let label = format!("{}{}", prefix, *count);
        *count += 1;
        label
    }

    pub fn reset(&mut self) {
        self.labels.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let mut labels = Labeller::new();
        assert_eq!(labels.generate("LOOP_"), "LOOP_0");
        assert_eq!(labels.generate("LOOP_"), "LOOP_1");
        assert_eq!(labels.generate("RETURN_"), "RETURN_0");
        assert_eq!(labels.generate("LOOP_"), "LOOP_2");
        assert_eq!(labels.generate("RETURN_"), "RETURN_1");
    }

    #[test]
    fn test_reset() {
        let mut labels = Labeller::new();
        for _ in 0..10 {
            labels.generate("LABEL_");
        }

        assert_eq!(labels.generate("LABEL_"), "LABEL_10");

        labels.reset();
        assert_eq!(labels.generate("LABEL_"), "LABEL_0");
        assert_eq!(labels.generate("LABEL_"), "LABEL_1");
    }
}
