fn share_data_between_threads_and_mutate() {
    use std::sync::Arc; // atomic reference counter(smart pointer)
    use std::sync::Mutex; // mutex -> mutual exclusive
    // smart pointer which guarantess that only one thread with lock
    // acquired will be able to mutate the value inside
    
    println!("Intro to Concurrency");
    let steps =  Arc::new(Mutex::new(5));
    let thread = {
        let steps = steps.clone();
        std::thread::spawn(move ||{
            while *steps.lock().unwrap() > 0{
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Thread step {}",steps.lock().unwrap());
                *steps.lock().unwrap() -=1 ;
            }
            "Goodbye!" // important thread could return values
        })
    };

    println!("Spawned a thread!");

    // Very important moment to understand closure captures
    // the environment
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Main thread slept for 3 seconds");
    
    let result = thread.join().unwrap(); 
    println!("Thread returned: {:?}", result);
}



fn fortune_cookie() {
    extern crate rand;
    use rand::Rng;
    use std::thread;
    // multiproducer, single consumer
    use std::sync::mpsc::channel;
    
    use std::time;

    let ten_millis = time::Duration::from_millis(1000);
    
    const DISCONNECT: &str = "Come back tomorrow!";
    
    let (sender,reciever) = channel();
    
    
    
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            
            let msg = match rng.gen_range(0..5)  {
                0 => "Fortune favors the brave.",
                1 => DISCONNECT,
                2 => "You will travel to many exotic places in your lifetime.",
                3 => "You can make your own happiness.",
                4 => "You are very talented in many ways.",
                _ => unreachable!(),
            };
            
            println!("Sending cookie: {}",msg);
            //thread::sleep(ten_millis);
            sender.send(msg).unwrap();
            if msg == DISCONNECT {
                break;
            }
        }
    });
    
    for recieved_msg in reciever {
        println!("What a day. Your fortune cookie : {}",recieved_msg);
        thread::sleep(ten_millis);
        
    }
    
}

fn main(){
    //share_data_between_threads_and_mutate();
    fortune_cookie();
}





