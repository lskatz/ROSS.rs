extern crate getopts;
extern crate fasten;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;

use fasten::fasten_base_options;
use fasten::io::fastq;
use fasten::io::seq::Cleanable;
use fasten::io::seq::Seq;
use fasten::logmsg;

use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = fasten_base_options();
    //script-specific flags
    opts.optflag("d","deshuffle","Deshuffle reads from stdin");
    opts.optopt("1","","Forward reads. If deshuffling, reads are written to this file.","1.fastq");
    opts.optopt("2","","Forward reads. If deshuffling, reads are written to this file.","2.fastq");

    let matches = opts.parse(&args[1..]).expect("Error: could not parse parameters");
    if matches.opt_present("help") {
        println!("Interleaves reads from either stdin or file parameters.\n{}", opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }
    if matches.opt_present("paired-end") {
        logmsg("WARNING: --paired-end was supplied but it is assumed for this script anyway");
    }

    if matches.opt_present("deshuffle") {
        deshuffle(&matches);
    } else {
        shuffle(&matches);
    }
}

fn deshuffle(matches: &getopts::Matches) -> () {
    
    // Where are we reading to?  Get those filenames.
    let r1_filename = if matches.opt_present("1") {
        matches.opt_str("1").unwrap()
    } else {
        "/dev/stdout".to_string()
    };
    let r2_filename = if matches.opt_present("2") {
        matches.opt_str("2").unwrap()
    } else {
        "/dev/stdout".to_string()
    };

    let mut file1 = File::create(r1_filename).expect("ERROR: could not write to file");
    let mut file2 = File::create(r2_filename).expect("ERROR: could not write to file");

    // read stdin
    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new(my_buffer);
    let mut read_counter=0;
    for seq in fastq_reader {
        
        // print to file 1 and to file 2, alternating each Seq
        if read_counter % 2 == 0 {
            write!(file1,"{}\n",seq.to_string()).unwrap();
        } else {
            write!(file2,"{}\n",seq.to_string()).unwrap();
        }
        read_counter+=1;
    }

}

fn shuffle(matches: &getopts::Matches) -> () {

    // Where are we reading from?  Get those filenames.
    let r1_filename = if matches.opt_present("1") {
        matches.opt_str("1").unwrap()
    } else {
        "/dev/stdin".to_string()
    };
    let r2_filename = if matches.opt_present("2") {
        matches.opt_str("2").unwrap()
    } else {
        "/dev/stdin".to_string()
    };

    // Read 1 first, and read 2 is halfway down.
    // Unfortunately this means that it all goes into ram.
    let     seqs1 = read_seqs(&r1_filename);
    let mut seqs2 = read_seqs(&r2_filename);
    let mut num_pairs = seqs1.len();

    // If reading R1 from stdin, it is possible that seqs2 
    // is empty. If so, redistribute half the reads from 
    // seqs1 into seqs2.
    if seqs2.len() == 0 {
        num_pairs = seqs1.len()/2;
        for seq in seqs1[seqs1.len()/2..seqs1.len()].iter() {
            seqs2.push(seq.clone());
        }
    }

    for i in  0..num_pairs  {
        seqs1[i].print();
        seqs2[i].print();
    }

}

fn read_seqs(filename: &String) -> Vec<Seq> {

    let my_file = File::open(&filename).expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let fastq_reader=fastq::FastqReader::new(my_buffer);
    let seqs :Vec<Seq> = fastq_reader.collect();
    return seqs;
}
