/*
// https://michael-f-bryan.github.io/cheat-sheets/Rust/emails_and_templates.html
use std::alloc::System;

#[global_allocator]
static A: System = System;

use lettre_email::{Email};
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use std::path::Path;
use std::{error};
use serde::{Serialize, Deserialize};
use tera::{Tera, Context};

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    email: String,
    name: String,
    company: String
}

type OryxResult<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() {

    let project_root = env!("CARGO_MANIFEST_DIR");
    let templates = format!("{}/*.html", project_root);
    let tera = Tera::new(&templates).unwrap();

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let domain: String = settings.get(&"domain").unwrap();
    let login: String = settings.get(&"username").unwrap();
    let pswd: String = settings.get(&"password").unwrap();
    let subj: String = settings.get(&"subject").unwrap();
    let attach: String = settings.get(&"attachment").unwrap();
/*  let html_contents = fs::read_to_string("html_contents.html")
        .expect("HTML content file is not found");
*/

    let file_path = Path::new("contacts.csv");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path).unwrap();

    let mut wtr_ok = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(Path::new("ok.csv")).unwrap();

    let mut wtr_err = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(Path::new("err.csv")).unwrap();

    for result in rdr.records() {
        let record: Record = result.unwrap().deserialize(None).unwrap();
        let user: &str = record.email.as_ref();

        let html = format_email(&tera, record.name.as_str(),
                                record.company.as_str(), "sender").unwrap();

        match SmtpClient::new_simple(&domain) {   // "smtp.gmail.com"
            Err(e) => println!("Could not connect to server: {:?}", e),
            Ok(client) => {
                let mut mailer =
                    client.credentials(Credentials::new(login.clone(), pswd.clone())
                    ).transport();

                match Email::builder()
                    .attachment_from_file(Path::new(&attach), None, &mime::APPLICATION_PDF)
                    {
                        Err(e) => println!("Error in preparing attachment {:?}", e),
                        Ok(e_builder) => {

                            let template = e_builder
                                .from(login.clone())
                                .subject(&subj)
                             //   .html(&html_contents)
                                .html(html)
                                ;

                            match template.clone().to(user).build() {
                                Err(e) => println!("error: {:?} in email: {}", e, user),
                                Ok(e) => {
                                    match mailer.send(e.into()) {
                                        Err(e) => {
                                            println!("error: {:?} in emailing: {}", e, user);
                                            wtr_err.serialize(record).unwrap();
                                        },
                                        Ok(r) => {
                                            println!("Emailing: {}, {:?}", user, r.message);
                                            wtr_ok.serialize(record).unwrap();
                                        }
                                    }
                                }
                            }

                        }
                    }
            }
        }
    }
}

fn format_email(tera: &Tera, name: &str, company: &str, sender: &str) -> OryxResult<String> {
  //  -> Result<String, Box<dyn Error>> {

    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("company", &company);
    context.insert("sender", &sender);

    match tera.render("email.html", &context) {
        Ok(template) => Ok(template),
        Err(e) => Err(e)?,
    }
}

*/