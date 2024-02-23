use std::sync::*;
use std::thread;
use std::thread::*;
use std::time::Duration;
use std::time::Instant;

#[derive(PartialEq)]
enum Sign {
    Busy,
    Available,
}

// number of guests is 10
const NUM_THREADS: usize = 10;
// duration of the party is 5 seconds
const PARTY_DURATION: Duration = Duration::from_secs(5);

// simulates the vase game
pub fn vase_sim() {
    let num_threads = NUM_THREADS;

    // sign which can be available or busy
    let sign = Arc::new(RwLock::new(Sign::Available));

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // get current time to specify duration of the party
    let party_start = Instant::now();

    for i in 0..num_threads {
        let sign = sign.clone();
        let handle = thread::spawn(move || guest_run(sign, i, party_start));
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

fn guest_run(sign: Arc<RwLock<Sign>>, num: usize, party_start: Instant) {
    // while the party has not ended
    while party_start.elapsed() < PARTY_DURATION {
        let mut state = Sign::Busy;
        {
            // check if sign is available
            let r = sign.read().unwrap();
            if *r == Sign::Available {
                state = Sign::Available;
            }
        }

        // go to vase if available
        if state == Sign::Available {
            let mut view = false;
            {
                let mut w = sign.write().unwrap();

                // in case it was changed from when we first read it
                if *w == Sign::Available {
                    *w = Sign::Busy;
                    view = true;
                }
            }
            if view {
                println!("Guest {} viewing vace", num);
                view_vace();
            }
            let mut w = sign.write().unwrap();
            // make sign available after we are done
            *w = Sign::Available;
        }

        // if sign not available have fun at the party
        have_fun();
    }
}

// simulates viewing vace by sleeping for 100 milliseconds
fn view_vace() {
    thread::sleep(Duration::from_millis(100));
}

// simulates having fun at party by sleeping for 50 milliseconds
fn have_fun() {
    thread::sleep(Duration::from_millis(50));
}
