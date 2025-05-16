use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant, SystemTime},
};

use reqwest::blocking::Client;

#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    action_status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

#[derive(Debug)]
struct Config {
    file_path: Option<String>,
    positional_urls: Vec<String>,
    workers: usize,
    timeout: u64,
    retries: usize,
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut file_path = None;
    let mut positional_urls = Vec::new();
    let mut workers = num_cpus::get(); // default
    let mut timeout = 5;
    let mut retries = 0;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i >= args.len() {
                    return Err("Expected a file path after --file".into());
                }
                file_path = Some(args[i].clone());
            }
            "--workers" => {
                i += 1;
                workers = args.get(i).ok_or("Missing value for --workers")?
                    .parse()
                    .map_err(|_| "Invalid number for --workers")?;
            }
            "--timeout" => {
                i += 1;
                timeout = args.get(i).ok_or("Missing value for --timeout")?
                    .parse()
                    .map_err(|_| "Invalid number for --timeout")?;
            }
            "--retries" => {
                i += 1;
                retries = args.get(i).ok_or("Missing value for --retries")?
                    .parse()
                    .map_err(|_| "Invalid number for --retries")?;
            }
            other if other.starts_with("--") => {
                return Err(format!("Unknown option: {}", other));
            }
            url => {
                positional_urls.push(url.to_string());
            }
        }
        i += 1;
    }

    if file_path.is_none() && positional_urls.is_empty() {
        return Err("Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S] [--retries N]".into());
    }

    Ok(Config {
        file_path,
        positional_urls,
        workers,
        timeout,
        retries,
    })
}

fn load_urls(config: &Config) -> Result<Vec<String>, String> {
    let mut urls = Vec::new();

    if let Some(file_path) = &config.file_path {
        let contents = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;
        for line in contents.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                urls.push(trimmed.to_string());
            }
        }
    }

    urls.extend(config.positional_urls.iter().cloned());

    if urls.is_empty() {
        return Err("No URLs provided via --file or positional arguments.".to_string());
    }

    Ok(urls)
}

fn check_website(client: &Client, url: &str, timeout: Duration, retries: usize) -> WebsiteStatus {
    let mut attempts = 0;
    let start = Instant::now();
    let mut result;

    loop {
        let attempt_start = Instant::now();
        result = client
            .get(url)
            .timeout(timeout)
            .send()
            .map(|res| res.status().as_u16())
            .map_err(|e| e.to_string());

        attempts += 1;
        if result.is_ok() || attempts > retries {
            break;
        }

        let elapsed = attempt_start.elapsed();
        if elapsed < Duration::from_millis(100) {
            std::thread::sleep(Duration::from_millis(100) - elapsed);
        }
    }

    WebsiteStatus {
        url: url.to_string(),
        action_status: result,
        response_time: start.elapsed(),
        timestamp: SystemTime::now(),
    }
}

fn write_json(results: &[WebsiteStatus]) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("status.json")?;

    writeln!(file, "[")?;
    for (i, result) in results.iter().enumerate() {
        let status_str = match &result.action_status {
            Ok(code) => format!("{{\"ok\": {}}}", code),
            Err(e) => format!("{{\"err\": \"{}\"}}", e.replace('"', "'")),
        };
        writeln!(
            file,
            "  {{\"url\": \"{}\", \"status\": {}, \"response_time_ms\": {}, \"timestamp\": \"{:?}\"}}{}",
            result.url,
            status_str,
            result.response_time.as_millis(),
            result.timestamp,
            if i + 1 < results.len() { "," } else { "" }
        )?;
    }
    writeln!(file, "]")?;
    Ok(())
}

fn main() {
    let config = match parse_args() {
        Ok(cfg) => cfg,
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(2);
        }
    };

    let urls = match load_urls(&config) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(2);
        }
    };

    let client = Arc::new(Client::builder()
        .timeout(Duration::from_secs(config.timeout))
        .build()
        .expect("Failed to build HTTP client"));

    let (job_sender, job_receiver) = mpsc::channel::<String>();
    let (result_sender, result_receiver) = mpsc::channel::<WebsiteStatus>();
    let job_receiver = Arc::new(Mutex::new(job_receiver));

    for _ in 0..config.workers {
        let job_receiver = Arc::clone(&job_receiver);
        let result_sender = result_sender.clone();
        let client = Arc::clone(&client);
        let timeout = Duration::from_secs(config.timeout);
        let retries = config.retries;

        thread::spawn(move || {
            while let Ok(url) = job_receiver.lock().unwrap().recv() {
                let status = check_website(&client, &url, timeout, retries);
                println!("{} -> {:?}", status.url, status.action_status);
                result_sender.send(status).unwrap();
            }
        });
    }

    for url in &urls {
        job_sender.send(url.clone()).unwrap();
    }
    drop(job_sender);

    let mut results = Vec::new();
    for _ in 0..urls.len() {
        if let Ok(result) = result_receiver.recv() {
            results.push(result);
        }
    }

    if let Err(e) = write_json(&results) {
        eprintln!("Failed to write status.json: {}", e);
    }
}