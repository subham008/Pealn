use criterion::{Criterion, criterion_group, criterion_main};

fn bench_pealn(c: &mut Criterion) {
    use pealn::pealn;
    c.bench_function("pealn", |b| {
        b.iter(|| {
            pealn!("[red](error) : [yellow](Invalid argument at) [blue](line 42) : [green](This is a test message)");
        })
    });
}

fn bench_colored(c: &mut Criterion) {
    use colored::*;
    c.bench_function("colored", |b| {
        b.iter(|| {
            println!(
                "{} : {} {} : {}",
                "error".red(),
                "Invalid argument at".yellow(),
                "line 42".blue(),
                "This is a test message".green()
            );
        })
    });
}

fn bench_crossterm(c: &mut Criterion) {
    use crossterm::style::{Color, Stylize};
    c.bench_function("crossterm", |b| {
        b.iter(|| {
            println!(
                "{} : {} {} : {}",
                "error".with(Color::Red),
                "Invalid argument at".with(Color::Yellow),
                "line 42".with(Color::Blue),
                "This is a test message".with(Color::Green)
            );
        })
    });
}

fn bench_termcolor(c: &mut Criterion) {
    use std::io::Write;
    use termcolor::{Color as TCColor, ColorChoice, ColorSpec, StandardStream, WriteColor};
    c.bench_function("termcolor", |b| {
        b.iter(|| {
            let mut buf = StandardStream::stdout(ColorChoice::Always);
            let mut spec = ColorSpec::new();
            spec.set_fg(Some(TCColor::Red));
            buf.set_color(&spec).unwrap();
            write!(&mut buf, "error").unwrap();

            spec.set_fg(Some(TCColor::Yellow));
            buf.set_color(&spec).unwrap();
            write!(&mut buf, " : Invalid argument at").unwrap();

            spec.set_fg(Some(TCColor::Blue));
            buf.set_color(&spec).unwrap();
            write!(&mut buf, " line 42").unwrap();

            spec.set_fg(Some(TCColor::Green));
            buf.set_color(&spec).unwrap();
            write!(&mut buf, " : This is a test message").unwrap();

            buf.reset().unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_pealn,
    bench_colored,
    bench_crossterm,
    bench_termcolor
);
criterion_main!(benches);
