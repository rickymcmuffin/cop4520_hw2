pub mod cupcake;
use core::time;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use rand::Rng;

use crate::cupcake::Cupcake;

// types of messages that can be sent to threads
#[derive(PartialEq)]
enum Message {
    NONE,
    INVITE,
    STOP,
}

// const num_threads: u64 = 10;

fn main() {
    labrynth_game();
}
fn labrynth_game() {
    // start and reading in input
    println!("Hello! Welcome to the minatour labrynth game!");

    println!("Enter the number of guests you would you like to particiapte: ");
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    while !line.trim().parse::<usize>().is_ok() {
        println!("{line}");
        println!("Value must be a number. Try again: ");
        line = "".to_string();
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    }
    let num_threads: usize = line.trim().parse().unwrap();

    println!("Enter the number of invitations you would like to send: ");
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    while !line.trim().parse::<usize>().is_ok() {
        println!("Value must be a number. Try again: ");
        line = "".to_string();
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    }
    let num_invitations: usize = line.trim().parse().unwrap();

    println!("");

    // cupcake state that will be shared among threads
    let cupcake_state = Arc::new(Mutex::new(Cupcake::new()));

    // a message queue so main thread can send messages to worker
    // threads
    let mut messages: Vec<Arc<Mutex<VecDeque<Message>>>> = vec![];

    // each message is a queue so thread can pop message from queue
    for _ in 0..num_threads {
        let mes = Arc::new(Mutex::new(VecDeque::new()));
        messages.push(mes);
    }

    // add all handles
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..num_threads {
        let message = messages[i as usize].clone();
        let cupcake_state = cupcake_state.clone();
        let handle = thread::spawn(move || guest_run(message, cupcake_state));
        handles.push(handle);
    }

    // send out invitations
    for _ in 0..num_invitations {
        // select random guest to invite
        let mut rng = rand::thread_rng();
        let guest = rng.gen_range(0..num_threads);
        println!("sending invite to guest {guest}");
        {
            // lock message and send INVITE
            let mut message = messages[guest as usize].lock().unwrap();
            message.push_back(Message::INVITE);
        }
        // has_invited[guest as usize] = true;

        thread::sleep(time::Duration::from_millis(10));
    }

    // send stop message to each thread
    for i in 0..num_threads {
        let mut message = messages[i as usize].lock().unwrap();
        message.push_back(Message::STOP);
    }

    // wait till threads are finished
    for handle in handles {
        handle.join().unwrap();
    }

    let cupcake_state = cupcake_state.lock().unwrap();

    println!(
        "\nNumber of cupcakes eaten: {}\n",
        cupcake_state.get_num_eaten()
    );

    if cupcake_state.get_num_eaten() != num_threads {
        println!("Not all guests invited to labrynth! :(");
    } else {
        println!("All guests invited ot labrynth! :)")
    }
}

fn guest_run(message: Arc<Mutex<VecDeque<Message>>>, cupcake: Arc<Mutex<Cupcake>>) {
    // thread needs to remember if it has eaten a cupcake
    let mut has_eaten = false;
    // println!("starting!");
    loop {
        let mut action = Message::NONE;
        {
            let mut message = message.lock().unwrap();

            if !message.is_empty() {
                action = message.pop_front().unwrap();
            }
        }
        if action == Message::STOP {
            break;
        } else if action == Message::INVITE {
            // println!("invited");
            enter_labrynth(cupcake.clone(), has_eaten);
            has_eaten = true;
        }
    }
}

fn enter_labrynth(cupcake: Arc<Mutex<Cupcake>>, has_eaten: bool) {
    // simulate going through maze
    let mut rng = rand::thread_rng();
    thread::sleep(time::Duration::from_millis(rng.gen_range(1..500)));

    let mut cupcake = cupcake.lock().unwrap();

    if !has_eaten {
        if !cupcake.is_out() {
            cupcake.request();
        }
        cupcake.eat();
    }
}
