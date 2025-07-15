
use lettre::{message::header::ContentType, Message, SendmailTransport, Transport};


//#![cfg(feature = "sendmail-transport")]


fn main() {
    
    // here the email is built
    let my_email = Message::builder()
        .from("ozog <ozog@cua.edu>".parse().expect("parse failed"))
        .to("ozog <ozog@cua.edu>".parse().expect("parse failed"))
        .subject("Email Sent From Rust")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("It worked!")).expect("failed to build");
    // here the sender is init
    let sender = SendmailTransport::new();
    // send the email and match to handle errors
    match sender.send(&my_email){
        Ok(stuff) => println!("{:?}", stuff),
        Err(e) => println!("{:?}", e),
    };
    // could send a lot of emails this way
    for _ in 0..10 {
        let sender = SendmailTransport::new();
        match sender.send(&my_email){
            Ok(stuff) => println!("{:?}", stuff),
            Err(e) => println!("{:?}", e),
        };
    }
}
