use lettre_email::{Email};
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use std::path::Path;
use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    email: String,
    name: String,
    company: String
}

fn main() {

    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings")).unwrap();
    //  .merge(config::Environment::with_prefix("APP")).unwrap();  // e.g. APP_DEBUG=1

    let login: String = settings.get(&"username").unwrap();
    let pswd: String = settings.get(&"password").unwrap();
    let subj: String = settings.get(&"subject").unwrap();
    let attach: String = settings.get(&"attachment").unwrap();
    let email: String = settings.get(&"email").unwrap();
    let html_contents = fs::read_to_string("html_contents.html")
        .expect("HTML content file is not found");
    let text_contents = fs::read_to_string("text_contents.txt")
        .expect("TEXT content file is not found");


    let file_path = Path::new("contacts.csv");
  //  let mut rdr = csv::Reader::from_path(file_path).unwrap();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path).unwrap();

  //  let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers().unwrap().clone();

    for result in rdr.records() { // }  .deserialize() {
    //    let record: Record = result.unwrap().deserialize(Some(&headers)).unwrap();
        let record: Record = result.unwrap().deserialize(None).unwrap();
        let user: &str = record.email.as_ref();

        match SmtpClient::new_simple("smtp.gmail.com") {
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
                                .from("myEMAIL@gmail.com")
                                .subject(&subj)
                                .html(&html_contents)
                                .text(&text_contents);

                            match template.clone().to(user).build() {
                                Err(e) => println!("error: {:?} in email: {}", e, user),
                                Ok(e) => {
                                    match mailer.send(e.into()) {
                                        Err(e) => println!("error: {:?} in emailing: {}", e, user),
                                        Ok(r) => println!("Emailing: {}, {:?}", user, r.message)
                                    }
                                }
                            }

                        }
                    }
            }
        }
    }
}
