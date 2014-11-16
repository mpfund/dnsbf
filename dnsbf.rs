extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use std::io;

struct AppSettings {
	dictionary:String,
	hostname: Option<String>,
	help: bool
}

fn main(){
	let args = os::args();
	let settings = read_arguments(&args);
	if settings.help {
		return;
	}
	if settings.hostname==None{
		println!("no host name argument.");
		return;
	}

	let mut cd = os::getcwd();
	cd.push(settings.dictionary);

	let dictionary_content = io::File::open(&cd).read_to_string().unwrap();
	let dictionary : Vec<&str> = dictionary_content.as_slice().split_str("\n").collect();
	let domain = settings.hostname.unwrap();
	query_hostnames_from_dicitonary(dictionary,domain.as_slice());
}

fn query_hostnames_from_dicitonary(dict:Vec<&str>,domain:&str){
	let outputfilename = "output.txt";
	let mut output = io::File::create(&Path::new(outputfilename)).unwrap();

	for subdomain in dict.iter() {
		let fulldomain = format!("{}.{}",subdomain.trim(),domain);
		let result = std::io::net::addrinfo::get_host_addresses(fulldomain.as_slice());
		match result{
			Ok(ips)=>{
				println!("found domain: {}" ,fulldomain);
				output.write_str(fulldomain.as_slice());
				output.write_str("\n");
				for ip in ips.iter() {
					output.write_str(format!("   {}\n",ip).as_slice());
				}
			}
			_=>{}
		}
	}
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("Brute force sub domain names from a given domain with a dictionary.");
    println!("Output file name is output.txt.");
    println!("");
    println!("-n\t\thost / domain name, e.g. google.com");
    println!("-d\t\tdictionary file name. (default is wordlist.wl)");
    println!("-h --help\tshow help.");
}


fn read_arguments(args:&Vec<String>)->AppSettings{
	let program = args[0].clone();

	let opts = [
		optopt("n", "host name", "target host or domain name", "NAME"),
		optopt("d", "dictionary", "input dictionary", "NAME"),
        optflag("h", "help", "print this help menu")
	];

	let mut settings = AppSettings {
		dictionary : "wordlist.wl".to_string(),
		hostname : None,
		help: false
	};

	let matches = match getopts(args.tail(),opts){
		Ok(m)=>{m}
		Err(f)=>{panic!(f.to_string())}
	};

	if matches.opt_present("h") {
		print_usage(program.as_slice(),opts);
		settings.help = true;
	} else{
		let output = matches.opt_str("d");
		if output != None {
			settings.dictionary = output.unwrap().to_string();
		}
		let hostname = matches.opt_str("n");
		if hostname != None {
			settings.hostname = Some(hostname.unwrap().to_string());
		}
	}
	settings
}