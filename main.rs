#![allow(unused)]
use std::io;
use std::io::{Write};

fn main() {
    println!("------Genetic Coder------");
    print!("enter codon sequence>>>");
    io::stdout().flush();

    let mut sequence: String = String::new();
    io::stdin().read_line(&mut sequence);
    let seq: Vec<char> = sequence.chars().collect();

    let mut codon:String = String::new();
    let mut ch = 0;
    while ch < seq.len() - 2{
        codon.push(seq[ch]);
        codon.push(seq[ch + 1]);
        codon.push(seq[ch + 2]);
        read_codon(codon);
        codon = String::from("");
        ch += 3;
    }
}

fn read_codon(codon: String){
    let codon:&str = &codon;
    let amino_acid:&str = match codon{
        "GUA"|"GUG"|"GUC"|"GUU"=>"Valine",
        "GCA"|"GCG"|"GCC"|"GCU"=>"Alanine",
        "GAC"|"GAU"            =>"Aspartic acid",
        "GAA"|"GAG"            =>"Glutamic acid",
        "GGA"|"GGG"|"GGC"|"GGU"=>"Glycine",
        "UUU"|"UUC"            =>"Phenylalanine",
        "UUA"|"UUG"|
        "CUA"|"CUG"|"CUC"|"CUU"=>"Leucine",
        "UCA"|"UCG"|"UCC"|"UCU"|
        "AGC"|"AGU"            =>"Serine",
        "UAU"|"UAC"            =>"Tyrosine",
        "UAA"|"UAG"|"UGA"      =>"STOP",
        "UGU"|"UGC"            =>"Cysteine",
        "UGG"                  =>"Tryptophan",
        "CCA"|"CCG"|"CCC"|"CCU"=>"Proline",
        "CAC"|"CAU"            =>"Histidine",
        "CGA"|"CGG"|"CGC"|"GCU"=>"Arginine",
        "AUA"|"AUC"|"AUU"      =>"Isoleucine",
        "AUG"                  =>"Methionine(START)",
        "ACA"|"ACG"|"ACC"|"ACU"=>"Threonine",
        "AAC"|"AAU"            =>"Asparagine",
        "AAA"|"AAG"            =>"Lysine",
        "AGA"|"AGG"            =>"Arginine",
        _ => "xxxInvalid Codonxxx"
    };
    println!("<{}>", amino_acid);
}