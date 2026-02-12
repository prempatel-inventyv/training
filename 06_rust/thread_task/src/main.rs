use std::{sync::{Arc, Mutex, MutexGuard, atomic::{AtomicI32, Ordering}}, thread};
use chrono::{DateTime, Local, Utc};
use std::time::Duration;


#[derive(Debug)]
#[allow(dead_code)]
struct MultiThread {
    id: i32,
    recordAddedTime: String,
    threadId: String, // randomly generated id
}

fn record_creator(counter: &AtomicI32, shared_data : &Arc<Mutex<Vec<MultiThread>>>){
    loop {
        let curr_number = counter.load(Ordering::SeqCst);
        let now = Local::now();
        let record = MultiThread{
            id:curr_number,
            recordAddedTime: now.to_string(),
            threadId: format!("{:?}", thread::current().id()),
        };
        {
            let mut data = shared_data.lock().unwrap();
            data.push(record);
        }
        counter.fetch_add(1, Ordering::SeqCst);
        thread::sleep(Duration::from_secs(10));
    }
}

fn printer(shared_data: &Arc<Mutex<Vec<MultiThread>>>){
   loop {
    {
        let data = shared_data.lock().unwrap();
        println!("All Threads : {:#?}",data);
    }
    thread::sleep(Duration::from_secs(2));
   }
}

fn remove_even_record(shared_data: &Arc<Mutex<Vec<MultiThread>>>){
    loop {
        {
            let mut data: MutexGuard<'_, Vec<MultiThread>>;
            {
                data = shared_data.lock().unwrap();
            }
            let now = Utc::now().timestamp();
            
            data.retain(|x| {
                let is_even = x.id % 2 == 0;
                let record_time = DateTime::parse_from_str(
                    &x.recordAddedTime,
                    "%Y-%m-%d %H:%M:%S%.f %:z",
                )
                .unwrap()
                .timestamp();

                let age = now - record_time;
                !(is_even && age > 20)
            });
            println!("After even id removed : {:#?}",data);
        }
        thread::sleep(Duration::from_secs(2));
    }
}

fn remove_odd_record(shared_data: &Arc<Mutex<Vec<MultiThread>>>) {
    loop {
        {   
            let mut data: MutexGuard<'_, Vec<MultiThread>>;
            {
                data = shared_data.lock().unwrap();
            }
            let now = Utc::now().timestamp();
            
            data.retain(|x| {
                let is_odd = x.id % 2 != 0;
                let record_time = DateTime::parse_from_str(
                    &x.recordAddedTime,
                    "%Y-%m-%d %H:%M:%S%.f %:z",
                )
                .unwrap()
                .timestamp();

                let age = now - record_time;
                !(is_odd && age > 20)
            });
            println!("After odd + old records removed: {:#?}", data);
        }
        thread::sleep(Duration::from_secs(2));
    }
}

fn even_count(shared_data: &Arc<Mutex<Vec<MultiThread>>>){
   loop {
        let mut even_counter = 0;
        {
            let data: MutexGuard<'_, Vec<MultiThread>>;
            {
                data = shared_data.lock().unwrap();
            }
            for i in data.iter(){
                if i.id % 2 == 0{
                    even_counter += 1;
                }
            }
        }
        println!("Even id threads are  : {}",even_counter);
        thread::sleep(Duration::from_secs(2));
   }
}

fn odd_count(shared_data: &Arc<Mutex<Vec<MultiThread>>>){
   loop {
        let mut odd_counter = 0;
        {
            let data: MutexGuard<'_, Vec<MultiThread>>;
            {
                data = shared_data.lock().unwrap();
            }
            for i in data.iter(){
                if i.id % 2 != 0{
                    odd_counter += 1;
                }
            }
        }
        println!("Even id threads are  : {}", odd_counter);
        thread::sleep(Duration::from_secs(2));
   }
}

fn main(){
    let counter = Arc::new(AtomicI32::new(0));
    let shared_data = Arc::new(Mutex::new(Vec::<MultiThread>::new()));

    let mut handles = Vec::new();

    let handle1 = thread::spawn({
        let shared_data = Arc::clone(&shared_data);
        let counter = Arc::clone(&counter);
        move || record_creator(&counter, &shared_data)
    });
    handles.push(handle1);
    
    let handle2 = thread::spawn({
        let shared_data = Arc::clone(&shared_data);
        move || printer(&shared_data)
    });
    
    handles.push(handle2);   

    let handle3 = thread::spawn({
        let shared_data = Arc::clone(&shared_data);
        move || remove_even_record(&shared_data)
    });
    handles.push(handle3);

    let handle4 = thread::spawn({
        let shared_data = Arc::clone(&shared_data);
        move || remove_odd_record(&shared_data)
    });
    handles.push(handle4);

    let handle5 = thread::spawn({
        let shared_data = Arc::clone(&shared_data);
        move || even_count(&shared_data)
    });
    handles.push(handle5);

    let handle6 = thread::spawn({
        let shared_data = Arc::clone(&shared_data);
        move || odd_count(&shared_data)
    });
    handles.push(handle6);

    for t in handles{
        t.join().unwrap();
        
    }
}