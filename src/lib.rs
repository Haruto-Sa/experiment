use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::thread;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub chars: usize,
    pub words: usize,
    pub line_count: usize,
    pub frequencies: HashMap<String, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalyzeError {
    EmptyInput,
}

impl Display for AnalyzeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "input is empty"),
        }
    }
}

impl std::error::Error for AnalyzeError {}

pub trait Analyzer {
    fn analyze(&self, input: &str) -> Result<Report, AnalyzeError>;
}

#[derive(Debug, Default)]
pub struct ParallelTextAnalyzer;

impl Analyzer for ParallelTextAnalyzer {
    fn analyze(&self, input: &str) -> Result<Report, AnalyzeError> {
        if input.trim().is_empty() {
            return Err(AnalyzeError::EmptyInput);
        }

        thread::scope(|scope| {
            let chars_handle = scope.spawn(|| input.chars().count());
            let words_handle = scope.spawn(|| input.split_whitespace().count());
            let lines_handle = scope.spawn(|| input.lines().count());
            let freq_handle = scope.spawn(|| {
                input
                    .split_whitespace()
                    .map(normalize)
                    .filter(|w| !w.is_empty())
                    .fold(HashMap::new(), |mut acc, word| {
                        *acc.entry(word).or_insert(0) += 1;
                        acc
                    })
            });

            Report {
                chars: chars_handle.join().unwrap_or_default(),
                words: words_handle.join().unwrap_or_default(),
                line_count: lines_handle.join().unwrap_or_default(),
                frequencies: freq_handle.join().unwrap_or_default(),
            }
        })
        .pipe(Ok)
    }
}

fn normalize(token: &str) -> String {
    token
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}

#[cfg(test)]
mod tests {
    use super::{Analyzer, AnalyzeError, ParallelTextAnalyzer};

    #[test]
    fn counts_text_and_words() {
        let analyzer = ParallelTextAnalyzer;
        let report = analyzer
            .analyze("Rust is fast. Rust is safe.")
            .expect("valid analysis");

        assert_eq!(report.words, 6);
        assert_eq!(report.frequencies.get("rust"), Some(&2));
        assert_eq!(report.frequencies.get("is"), Some(&2));
    }

    #[test]
    fn rejects_empty_input() {
        let analyzer = ParallelTextAnalyzer;
        let error = analyzer.analyze("   ").expect_err("empty must fail");
        assert_eq!(error, AnalyzeError::EmptyInput);
    }
}
