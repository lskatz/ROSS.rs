extern crate ross;
extern crate getopts;
extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

use std::env;
use std::f32;

use ross::ross_base_options;

const  TEN: f32 = 10.0;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut opts = ross_base_options();

    let matches = opts.parse(&args[1..]).expect("ERROR: could not parse parameters");

    if matches.opt_present("h") {
        println!("Emma: collapse identical reads into single reads, recalculating quality values. If paired end, then each set of reads must be identical. Rachel's daughter Emma was played by twins, essentially collapsing two individuals into one character!");
        println!("{}",opts.usage(&opts.short_usage(&args[0])));
        std::process::exit(0);
    }

    let lines_per_read={
        if matches.opt_present("paired-end") {
            8
        }else{
            4
        }
    };

    let mut entries :HashMap<String,u32>= HashMap::new();

    let my_file = File::open("/dev/stdin").expect("Could not open file");
    let my_buffer=BufReader::new(my_file);
    let mut line_counter =0;
    let mut entry = String::new();
    for line in my_buffer.lines() {
        // unwrap the line here and shadow-set the variable.
        let line=line.expect("ERROR: did not get a line");
        line_counter+=1;
        entry.push_str(&line);
        entry.push_str("\n");

        // Action if we have a full entry when mod 0
        if line_counter % lines_per_read == 0 {
            // increment the counter
            let count = entries.entry(entry).or_insert(0);
            *count += 1;
            // reset the entry string
            entry = String::new();
        }
    }

    for (entry,count) in entries {
        //println!("{} => {}",count,entry);
        let mut lines = entry.lines();
        let mut id=  lines.next().expect("Could not parse for the ID line").to_string();
        let     seq= lines.next().expect("Could not parse for the seq line");
                     lines.next().expect("Could not parse for the + line");
        let     qual=lines.next().expect("Could not parse for the qual line");
        id.push_str(" ");
        id.push_str(&count.to_string());

        println!("{} {} {}",id,seq,recalculate_qual(qual,count));
        // TODO edit the count 
        // TODO edit the qual
        // TODO paired end
    }
}

fn recalculate_qual(qual_str: &str, count: u32) -> String {
    let mut qual_out = String::new();

    let qual = qual_str.to_string();
    for qual_char in qual.chars() {
        let qual_int = qual_char as u8 as f32 - 33.0;
        //let ten:f32=10.0;
        let p :f32 = TEN.powf((-1.0 * qual_int)/TEN);
        let p_recalc :f32 = p.powi(count as i32);
        let qual_recalc = -TEN * (p_recalc).log(TEN)+33.0;
        let mut qual_recalc_char = qual_recalc.floor() as u8 as char;
        if qual_recalc_char as u8 > 'I' as u8 {
            qual_recalc_char = 'I';
        }
        qual_out.push(qual_recalc_char);
    }

    return qual_out;
}




