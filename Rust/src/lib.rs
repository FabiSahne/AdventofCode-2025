#[macro_export]
macro_rules! time {
    ($($input:tt)*) => {{
        let begin = line!();
        let start = ::std::time::Instant::now();
        let result = $($input)*;
        let duration = start.elapsed();
        println!("{}:{} took {:?}.", file!(), begin, duration);
        result
    }};
}
