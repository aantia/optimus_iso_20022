// Import common types and traits in the prelude
use iso_20022_sdk::prelude::*;
//use iso_20022_sdk::documents::Document;
//use iso_20022_sdk::documents::DocumentType;

use backtrace::Backtrace;

fn main() {
    let bt = Backtrace::new();
    //let xml_message = std::env::args().nth(1).expect("missing 1st command line arg");
    let message_type = std::env::args().nth(2).expect("missing 2nd command line arg");

    // expect message_type to be "caaa.001.001.11" (AuthorizationRequest)
    if message_type != "caaa.001.001.11" {
        panic!("message_type (the 2nd argument) must be caaa.001.001.11");
    }

    let out = Message::builder().set_document(Document::from_namespace(&message_type));

    let astring = std::any::type_name_of_val(&out);
    println!("Type of experimental empty document: {}",astring);
    // let mut var1 = xml_message.clone();
    // let mut var2 = message_type.clone();

    // from docs https://emergentfinancial.github.io/iso-20022/message_envelope.html
    // let msg = Message::<_>::builder()
    //     .set_document(Document::from_namespace(message_type.as_str()));

    //let msg: Result<Message<BizMsgEnvlp>, Error> = Message::<BizMsgEnvlp>::from_xml(xml_message.as_str());
    
    //let msg: Result<Message<_>, Error> = Message::from_xml(&xml_message);

    //println!("pattern: {:?}, path: {:?}", xml_message, message_type)
    //let doc_type = Document::from_namespace(&message_type);
    //let myclone = doc_type.clone(); // WHY??? is this needed to print doc_type?

    // now I'll try to create a message from the xml string
    //let msg = Message::from_xml(&xml_message);
    //let mut msg = Message::builder().set_document(doc_type);
    //let msg_result: Result<Message<Document>, Error> = Message::from_xml(&xml_message);


    // if let Ok(test_message) = std::fs::read_to_string("test_message.xml") {
    //     let msg_result: Result<Message<Document>, Error> = Message::from_xml(&test_message);
    
    //     println!("Message: {:?}", msg_result);
    // } else {
    //     println!("Error reading test_message.xml:");
    // }

    let test_message = std::fs::read_to_string("test_input.xml");
    let binding = test_message.unwrap();
    let msg_result: Result<Message<Document>, Error> = Message::from_xml(&binding);
    //let myerr = msg_result.unwrap_err();
    let my_result = msg_result.as_ref().unwrap();

    println!("Message: {:?}", my_result);
    println!("Backtrace:\n{bt:?}");


}
