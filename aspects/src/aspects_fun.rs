use std::time::{Duration, Instant};

fn chrono_fun<A, R, F>(fun: F) -> impl Fn(A) -> R 
where
    F: Fn(A) -> R,
{
    move |x| { 
        let now = Instant::now();
        let res = fun(x);
        println!("Duration {} micro sec.", now.elapsed().as_micros());
        res
     }
}