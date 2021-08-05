use rand::Rng;
use std::fs;
use std::io;
use std::io::Stdin;
fn main() {
    let stdin = io::stdin();
    let mut cached_content = Vec::new();
    let mut cached_key = Vec::new();

    loop {
        let mut buffer = String::new();

        println!("1. Load Message from file");
        println!("2. Turn Message to bytes");
        println!("3. Generate key");
        println!("4. Cipher Loaded Message");
        println!("5. Decipher Loaded Message");
        println!("6. Check compatibility of 2 files using key");
        println!("7. Readme");
        println!("8. Clean cache");
        println!("0. Zakończ");

        stdin
            .read_line(&mut buffer)
            .expect("Error: Problems with input");
        let trimmed = buffer.trim().parse::<u32>();
        let mut option = -1;

        if let Ok(i) = trimmed {
            option = i as i32;
        }

        match option {
            -1 => println!("Zły input"),
            0 => return,
            1 => cached_content = load(&stdin, "Message").unwrap_or_default(),
            //1 => cipher(&stdin),
            2 => println!("{}", turn_into_bytes(&cached_content)),
            3 => cached_key = generate_key(&cached_content).unwrap_or_default(),
            4 => cipher(&stdin, &mut cached_content, &mut cached_key),
            5 => decipher(&stdin, &mut cached_content, &mut cached_key),
            6 => check_compatibility(&stdin, &mut cached_key),
            7 => print_readme(),
            8 => {
                cached_content = Vec::new();
                cached_key = Vec::new();
            }
            _ => println!("Zły input"),
        }
    }
}

fn cipher(stdin: &Stdin, content: &mut Vec<u8>, key: &mut Vec<u8>) {
    if content.is_empty() {
        println!("There is no message currently loaded");
        *content = load(stdin, "Raw Message").unwrap_or_default();
    }
    if key.is_empty() {
        println!("There is no message currently loaded");
        *key = load(stdin, "Key").unwrap_or_default();
    }
    if content.is_empty() || key.is_empty() {
        println!("Couldn't find key or message");
        return;
    }
    println!("Type filename of 'ciphred_file'.");
    let filenames = get_filenames(stdin);
    if filenames.0.is_none() {
        println!("Wrong file names");
        return;
    }
    let ciphred_filename = filenames.0.unwrap();

    let mut ciphred = Vec::new();
    for iteration in 0..content.len() {
        let key_byte = key[iteration];
        let content_byte = content[iteration];
        ciphred.push(content_byte ^ key_byte);
    }
    fs::write(ciphred_filename, ciphred).expect("Writting to file error");
}

fn decipher(stdin: &Stdin, content: &mut Vec<u8>, key: &mut Vec<u8>) {
    if content.is_empty() {
        println!("There is no message currently loaded");
        *content = load(stdin, "Ciphred Message").unwrap_or_default();
    }
    if key.is_empty() {
        println!("There is no message currently loaded");
        *key = load(stdin, "Key").unwrap_or_default();
    }
    if content.is_empty() || key.is_empty() {
        println!("Couldn't find key or message");
        return;
    }
    println!("Type filename of 'deciphred_file'.");
    let filenames = get_filenames(stdin);
    if filenames.0.is_none() {
        println!("Wrong file names");
        return;
    }
    let deciphred_filename = filenames.0.unwrap();

    let mut deciphred_message = Vec::new();

    if key.len() < content.len() {
        println!("Error: key and ciphred file have diffrent sizes.");
        return;
    }
    for index in 0..content.len() {
        let key_byte = key[index];
        let cipher_byte = content[index];

        deciphred_message.push(cipher_byte ^ key_byte);
    }
    let message = String::from_utf8(deciphred_message);
    match message {
        Ok(msg) => {
            println!("{}", msg);
            fs::write(deciphred_filename, msg).expect("Writting to file error");
        },
        Err(error) => {
            println!("{}", error);
        }
    }
}

fn check_compatibility(stdin: &Stdin, key: &mut Vec<u8>) {
    if key.is_empty() {
        println!("There is no message currently loaded");
        *key = load(stdin, "Key").unwrap_or_default();
    }
    println!("Type filenames of 'ciphred_file deciphred_file'.");
    let filenames = get_filenames(stdin);
    if filenames.0.is_none() || filenames.1.is_none() {
        println!("Wrong file names");
        return;
    }

    let ciphred_conent =
        fs::read(filenames.0.unwrap()).expect("file is not present in this folder");
    let deciphred_conent =
        fs::read(filenames.1.unwrap()).expect("file is not present in this folder");

    if ciphred_conent.is_empty() || key.is_empty() || deciphred_conent.is_empty() {
        println!("Could't find ciphred file, deciphred file or key");
        return;
    }
    let mut files_match = false;
    if ciphred_conent.len() != key.len() {
        println!("Key don't match ciphred_message");
        files_match = true;
    }
    if deciphred_conent.len() != key.len() {
        println!("Key don't match deciphred_message");
        files_match = true;
    }
    if files_match {
        println!("Files don't match");
        return;
    }

    for index in 0..ciphred_conent.len() {
        let ciphred_byte = ciphred_conent[index];
        let deciphred_byte = deciphred_conent[index];
        let key_byte = key[index];
        if deciphred_byte != (ciphred_byte ^ key_byte) {
            println!("Files don't match");
            return;
        }
    }
    println!("Files are matching");
}

fn get_filenames(stdin: &Stdin) -> (Option<String>, Option<String>, Option<String>) {
    let mut buffer = String::new();
    stdin
        .read_line(&mut buffer)
        .expect("Error: Problems with input");
    let mut filename_iter = buffer.split_whitespace();

    (
        match filename_iter.next() {
            Some(x) => Some(String::from(x)),
            None => None,
        },
        match filename_iter.next() {
            Some(x) => Some(String::from(x)),
            None => None,
        },
        match filename_iter.next() {
            Some(x) => Some(String::from(x)),
            None => None,
        },
    )
}

fn load(stdin: &Stdin, file_to_load_name: &str) -> Option<Vec<u8>> {
    println!("Type filename of '{}'.", file_to_load_name);
    let filenames = get_filenames(stdin);
    if filenames.0.is_none() {
        println!("Wrong input fromat");
        return None;
    }
    let file_to_load = filenames.0.unwrap();
    let content = fs::read(file_to_load).expect("file is not present in this folder");
    let message = String::from_utf8_lossy(&content);

    println!("{} is {}", file_to_load_name, message);

    Some(content)
}

fn turn_into_bytes(content: &[u8]) -> String {
    let content_vec = content;

    content_vec
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
}

fn generate_key(content: &[u8]) -> Option<Vec<u8>> {
    println!("Generating Key");
    let mut key = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..content.len() {
        let random_byte: u8 = rng.gen::<u8>();
        key.push(random_byte);
    }

    let key_str = key
        .clone()
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>();
    println!("Key is '{}' ", key_str);
    fs::write("key", key.clone()).expect("Writting to file error");
    Some(key)
}

fn print_readme() {
    let readme = r#"                          ___Options___
    1. Load Message from file ->  load content of given file and hold it in memory untill programs shutdown or read another file
    2. Turn Message to bytes ->  simply print binary representation of content loaded from file using option 1.
    3. Generate key ->  generate key for currently loaded content, print it and save to 'key' file
    4. Cipher Loaded Message -> Using cached key and loaded content(raw message) it ciphers content and save it to file
    5. Decipher Loaded Message -> Using cached key and loaded content(ciphred message) it deciphers it into raw message and saves to file.
    6. Check compatibility of 2 files using key -> using cached key it load content of both files provided by client and checks if they have same content after deciphering.
    8. Clean cache -> This will clear cached key and content of file
                                             ___Importatnt Note___
    If key or Message are not loaded client will be asked for filename and they will be loaded and cached."#;
    println!("{}", readme);
}
