use std::process::Command;
use rand::{Rng, thread_rng};
use std::fs::{File, remove_file};

fn test(command: &str, args_list: Vec<&str>, s: Option<&str>, f: Option<&str>) {
	let file_creation = File::create(f.unwrap_or("file.txt"));

	for i in 0..100 {
        let mut args = Vec::new();
        let mut rng = thread_rng();
        let num_args = rng.gen_range(1..args_list.len());

        for _ in 0..num_args {
            let rand_index = rng.gen_range(0..args_list.len());
            let rand_arg = args_list[rand_index];
            args.push(rand_arg.replace("<S>", s.unwrap_or("', '"))
								.replace("<F>", f.unwrap_or("file.txt")));
        }

        let output = Command::new(command)
            .args(&args)
            .output()
            .expect("Error");

        if !output.status.success() {
            println!("Input {}: {:?}", i, args);
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
	if let Ok(_) = file_creation {
		remove_file(f.unwrap_or("file.txt")).expect("Error in file deletion");
	}
}

fn main() {
	test("ls", vec!["-a", "-d", "-l", "-F", "-lh", "-r", "-i", "-ltr", "-t", "-n", "-m", "-g", "-p", "-Q"], None, None);
	
	test("file", vec!["--help", "-v", "-m", "-z", "-Z", "-b", "-c", 
										"-e<F>", "--exclude-quiet=<F>", "-f<F>", "-F<S>", 
										"-i", "--apple", "--extension", "--mime-type", "--mime-encoding", 
										"-k", "-l", "-L", "-h", "-n", "-N", "-0", "-p", "-r", "-s", "-S", "-C", "-d"], 
										Some(", "), Some("file.txt"));
}
