use std::time::Instant;

#[allow(dead_code)]
pub fn timeit(f: fn() -> (), times: u32) -> String {
    let now = Instant::now();
    for _ in 0..times {
        f();
    }
    format!(
        "time for {} runs was {} millis.",
        times,
        now.elapsed().as_millis()
    )
}
