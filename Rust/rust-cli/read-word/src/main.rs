use std::io::Read;

use regex::RegexSet;
use dotext::{Docx, MsDoc};

fn main(){
let required_skills = vec![
        "Supply Chain",
        "Project(s|^s)",  //# (s|^s) ending with or without `s`
        "Control(?=[^s])", //# (?=[^s]) not ending with s
        "Plicy|Policies",  // either singular or plural
        "Procurement", "Contracting", "Analysis", "optimization", "working capital",
        "utilization", "Planning", "Forecasting", "Sourcing", "Purchasing", "Material", "Inventory",
        "Production", "Logistics", "Transportation", "Customer Service", "ERP", "Compliance",
        "Procedure(s|^s)", "Synergy", "cash"
    ]; //  to be scanned in the given CV

    let searches =required_skills.iter().map(|s| format!("(?i){}", s)).
        collect::<Vec<String>>();

    let set = RegexSet::new(&searches).unwrap();

 //   let mut file = std::fs::File::open("CV.docx").expect("Cannot open file");
    let mut file = Docx::open("HasanResume.docx").expect("Cannot open file");
    let mut isi = String::new();
    let _ = file.read_to_string(&mut isi);

    if set.is_match(&isi){
        let matches = set.matches(&isi).into_iter().collect::<Vec<_>>();
        let exact_matches = matches.iter().
                                    map(|s| searches[*s].as_str()).collect::<Vec<&str>>();
        println!("Out of {} requirements, the below {} matches had been found:",
                set.len(), matches.len());
        for x in exact_matches {
                println!("{}", x );
        }
    } else {
            println!("No matches found");
    }
}