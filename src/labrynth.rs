use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use rand::Rng;

use crate::cupcake::Cupcake;

// types of messages that can be sent to threads
#[derive(PartialEq)]
enum Message {
    None,
    Invite,
    STOP,
    AllInvited,
}

// simulates labrynth game. Check the README to find a full explanation of the solution
pub fn labrynth_game() {
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

    println!("");

    // cupcake state that will be shared among threads
    let cupcake_state = Arc::new(Mutex::new(Cupcake::new()));

    // a message queue so main thread can send messages to worker threads
    let mut messages: Vec<Arc<Mutex<VecDeque<Message>>>> = vec![];

    // message queue so worker threads can send messages to main thread
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // each message is a queue so thread can pop message from queue
    for _ in 0..num_threads {
        let mes = Arc::new(Mutex::new(VecDeque::new()));
        messages.push(mes);
    }

    // add all handles
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..num_threads {
        let message = messages[i].clone();
        let cupcake_state = cupcake_state.clone();
        let tx = tx.clone();
        let handle;
        if i == 0 {
            handle =
                thread::spawn(move || guest_run(message, cupcake_state, tx, num_threads, true));
        } else {
            handle =
                thread::spawn(move || guest_run(message, cupcake_state, tx, num_threads, false));
        }
        handles.push(handle);
    }

    // while main thread has not received a message from any worker threads
    while rx.try_iter().next().is_none() {
        // select random guest to invite
        let mut rng = rand::thread_rng();
        let guest = rng.gen_range(0..num_threads);
        println!("sending invite to guest {guest}");
        {
            // lock message and send INVITE
            let mut message = messages[guest].lock().unwrap();
            message.push_back(Message::Invite);
        }
    }

    // send stop message to each thread
    for i in 0..num_threads {
        let mut message = messages[i].lock().unwrap();
        message.push_back(Message::STOP);
    }

    // wait till threads are finished
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All guests invited ot labrynth! :)")
}

fn guest_run(
    message: Arc<Mutex<VecDeque<Message>>>,
    cupcake: Arc<Mutex<Cupcake>>,
    tx: Sender<Message>,
    num_guests: usize,
    leader: bool,
) {
    // thread needs to remember if it has eaten a cupcake
    let mut num_seen_empty = 0;
    let mut has_eaten = false;
    loop {
        let mut action = Message::None;
        {
            let mut message = message.lock().unwrap();

            // if there exists an element pop it off
            if !message.is_empty() {
                action = message.pop_front().unwrap();
            }
        }
        // we've recieved the signal from the main thread to stop
        if action == Message::STOP {
            break;

        // we have recieved an invite so enter labrynth
        } else if action == Message::Invite {
            let is_empty = !enter_labrynth(cupcake.clone(), has_eaten, leader);
            if !is_empty {
                has_eaten = true;
            }
            if leader && is_empty {
                num_seen_empty += 1;
                println!("seen: {}", num_seen_empty);
            }
        }
        if leader && num_seen_empty == num_guests {
            tx.send(Message::AllInvited).unwrap();
        }
    }
}

// returns true if cupcake was there, false if it was not there
fn enter_labrynth(cupcake: Arc<Mutex<Cupcake>>, has_entered: bool, leader: bool) -> bool {

    let mut cupcake = cupcake.lock().unwrap();

    // need to know if cupcake was out or not
    let ret = cupcake.is_out();

    // leader always requests cupcake
    if leader && !cupcake.is_out() {
        if !cupcake.is_out() {
            cupcake.request();
        }
    }

    if cupcake.is_out() && !has_entered {
        cupcake.eat();
    }

    return ret;
}
