use std::collections::HashMap;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Instant;

mod example_text;
use example_text::TEXT;

// A helper function used to measure execution time in milliseconds.
fn measure_exec_time_ms(start_time: Instant) -> f64 {
    let elapsed = start_time.elapsed();
    (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000.0)
}

// Number of threads to use in our parallel computations.
const THREADS: usize = 4;

fn main() {
    // Start the timer to measure execution time.
    let start = Instant::now();

    // Create our target (master) map of words.
    let mut master_map: HashMap<String, u32> = HashMap::new();

    // Next, split the input text into vector of words, and wrap it into Arc as well (to safely
    // reference it from many threads). Also, calculate the number of words that should go in
    // chunk when the vector is divided into n, smaller vectors, where n == THREADS.
    let words: Arc<Vec<&str>> = Arc::new(TEXT.split_whitespace().collect());
    let len = words.len();
    let num_words_in_chunk = len / THREADS;

    // Create multi-producer, single consumer channel.
    let (tx, rx) = mpsc::channel();

    for i in 0..THREADS {
        // Create local version of channel's sender.
        let local_tx = tx.clone();

        // Clone the Arc pointer to the words vector.
        let words = words.clone();
        thread::spawn(move || {
            // Create local map first. Put all computations done in that thread in this map.
            let mut local_map = HashMap::new();

            // Iterate over words in adequate chunk.
            for &word in words[(i * num_words_in_chunk)..((i + 1) * num_words_in_chunk)].iter() {
                // Cast all characters to lowercase, get rid of non-alphanumeric characters (like punctuation).
                let word = word.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();

                // Increment count of given word within local map. The pattern below is safe, because
                // if given word (key) does not exist in the map yet, it will we be inserted first with value of 0
                // and then the value will be incremented.
                let count = local_map.entry(String::from(word)).or_insert(0);
                *count += 1;
            }

            // Pass the result of that thread's computations over the channel.
            local_tx.send(local_map).unwrap();
        });
    }

    // Expect as many "messages" over the channel, as there are THREADS.
    for _ in 0..THREADS {
        // Receive a message, unwrap it.
        let sub_map = rx.recv().unwrap();

        // For each record in sub_map, update the master count in our target map.
        for (word, number) in sub_map {
            let count = master_map.entry(word).or_insert(0);
            *count += number;
        }
    }

    // Finally, print out the results in the master map!
    for (word, number) in master_map.iter() {
        println!("{}: {}", word, number);
    }

    // Note, that for example four threads vs one thread is notably faster only when
    // the input text is literally hundreds of thousands of words long.
    // With small texts, there is practically no difference.
    println!("(finished in {} ms)", measure_exec_time_ms(start));
}
