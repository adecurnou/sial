#![allow(dead_code)]

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::error::Error;
use std::default::Default;

mod tokenizer;      // Tokenizes files into words, sentences, and paragraphs.
mod calculator;     // Does all the calculations for stats

#[derive(Default, Copy, Clone)]
struct Metadata {
    word_total: f32,    // Intermediary for calculator.
    wordlen: f32,
    sentlen: f32,
    paralen: i32,
    comma: i32,
    semicolon: i32,
    quote: i32,
    bangs: i32,
    dashes: i32,
    ands: i32,
    buts: i32,
    however: i32,
    condition: i32,		// Prevent confusion with if.
    thats: i32,
    more: i32,
    musts: i32,
    mights: i32,
    thises: i32,
    very: i32,
}

impl Metadata {
    fn words(&mut self, s: Vec<&str>) {
        self.word_total = calculator::num_words(&s);
        self.wordlen = calculator::word_mean(&s);

        // Let me throw all elegance out the window.
        let (ands, buts, however, condition, thats, more,
            musts, mights, thises, very) = calculator::word_freq(&s);
        
        self.ands = ands;
        self.buts = buts;
        self.however = however;
        self.condition = condition;
        self.thats = thats;
        self.more = more;
        self.musts = musts;
        self.mights = mights;
        self.thises = thises;
        self.very = very;
    }

    fn sents(&mut self, s: Vec<&str>) {
        self.sentlen = calculator::sent_mean(&self.word_total, &s);
    }
}

fn main() {
    
    let mut off = Metadata {..Default::default()};
    let mut pseu = Metadata {..Default::default()};

    let args: Vec<String> = env::args().collect();
    
    let mut file1 = match File::open(&args[1]) {
        Err(why) => panic!("Couldn't open {}: {}", &args[1], Error::description(&why)),
        Ok(file1) => file1,
    };
    
    let mut file2 = match File::open(&args[2]) {
        Err(why) => panic!("Couldn't open {}: {}", &args[2], Error::description(&why)),
        Ok(file2) => file2,
    };

    let (mut s1, mut s2) = (String::new(), String::new());

    match file1.read_to_string(&mut s1) {
        Err(why) => panic!("Couldn't read file1: {}", Error::description(&why)),
        Ok(_) => print!(""),
    };
    
    match file2.read_to_string(&mut s2) {
        Err(why) => panic!("Couldn't read file2: {}", Error::description(&why)),
        Ok(_) => print!(""),
    };

    let s1_token: Vec<&str> = tokenizer::word_token(&s1);
    let s2_token: Vec<&str> = tokenizer::word_token(&s2);
    let s1_sent: Vec<&str> = tokenizer::sent_token(&s1);
    let s2_sent: Vec<&str> = tokenizer::sent_token(&s2);
    
    off.words(s1_token);
    pseu.words(s2_token);
    off.sents(s1_sent);
    pseu.sents(s2_sent);
    print_data(off, pseu);

}

fn print_data (st1: Metadata, st2: Metadata) {
    println!("{0: <30} {1: <20} {2: <10}", "", "Official", "Pseudonym");
    println!("{0: <30} {1: <40}", "", "---------------------------------");
    println!("{0: <30} {1: <20} {2:<10}", "Word total", st1.word_total, st2.word_total);
    println!("{0: <30} {1: <20} {2:<10}", "Mean word length", st1.wordlen, st2.wordlen);
    println!("{0: <30} {1: <20} {2:<10}", "Mean sentence length", st1.sentlen, st2.sentlen);
    //println!("{0: <30} {1: <40}", "", "---------------------------------");
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'and'", st1.ands, st2.ands);
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'but'", st1.buts, st2.buts);
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'however'", st1.however, st2.however);
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'if'", st1.condition, st2.condition);
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'that'", st1.thats, st2.thats);
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'more'", st1.more, st2.more);
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'must'", st1.musts, st2.musts);
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'might'", st1.mights, st2.mights);
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'this'", st1.thises, st2.thises);
    println!("{0: <30} {1: <20} {2:<10}", "Frequency of 'very'", st1.very, st2.very);
}