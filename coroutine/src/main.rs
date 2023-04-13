use std::ops::{AddAssign, Div};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
struct Event {
    pub _type: String,
    pub time: SystemTime,
}

fn main() {
    let count = Arc::new(AtomicU64::new(0));
    let total: Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));
    let (s, r) = mpsc::sync_channel::<Event>(200);
    let count2 = count.clone();
    let total2 = total.clone();
    thread::spawn(move || loop {
        if let Ok(res) = r.try_recv() {
            let elapsed = res.time.elapsed().ok().unwrap();
            count2.fetch_add(1, Ordering::Relaxed);
            total2.lock().unwrap().add_assign(elapsed);
        }
    });
    thread::sleep(Duration::from_secs(1));

    let max_loop = 100000;
    for _ in 0..max_loop {
        let _ = s.send(Event {
            _type: "hello".to_string(),
            time: SystemTime::now(),
        });
        thread::sleep(Duration::from_micros(1));
    }
    thread::sleep(Duration::from_secs(2));

    let total2 = total.lock().unwrap();
    println!(
        "total: {:?}, average: {:?}, {:?}/s",
        total2,
        total2.div(count.load(Ordering::Relaxed) as u32),
        count.load(Ordering::Relaxed) as f64 / total2.as_secs_f64()
    );
}
