use rust_showcase::{Analyzer, ParallelTextAnalyzer};

fn main() {
    let input = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    if input.is_empty() {
        eprintln!("Usage: cargo run -- \"your text here\"");
        return;
    }

    let analyzer = ParallelTextAnalyzer;
    match analyzer.analyze(&input) {
        Ok(report) => {
            println!("chars      : {}", report.chars);
            println!("words      : {}", report.words);
            println!("lines      : {}", report.line_count);
            println!("top words  :");

            let mut pairs: Vec<_> = report.frequencies.into_iter().collect();
            pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            for (word, count) in pairs.into_iter().take(5) {
                println!("  - {word}: {count}");
            }
        }
        Err(error) => eprintln!("error: {error}"),
    }
}
