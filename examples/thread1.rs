use std::os::unix::raw::mode_t;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use log::info;

const NUM_PRODUCER: usize = 4;

#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self {
            idx,
            value,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let (px, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCER {
        let sx = px.clone();
        thread::spawn(move ||
            producer(i, sx)
        );
    }
    // 销毁多余的px
    drop(px);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        42
    });

    let secret = consumer.join().map_err(|e| anyhow::anyhow!("Thread join err:{:?}", e))?;
    println!("secret={}", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<(Msg)>) -> anyhow::Result<()>{
    loop {
        let val = rand::random::<usize>();
        tx.send(Msg::new(idx, val))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 5 == 0 {
            println!("producer:{} stop", idx);
            break;
        }
    }
    Ok(())
}