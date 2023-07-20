use llm::models::Llama;
use llm::KnownModel;
use llm::Model;
use std::path::PathBuf;


struct Message{
    user: bool,
    text: String
}

struct Conversation{
    messages: Vec<Message>
}

impl Conversation{
    fn new() -> Conversation {
        Conversation {
            messages: Vec::new()
        }
    }
}

fn main() {
    let conversation = Conversation::new();
    
    let mut context_dialog = format!(
        "### Robot: It's me robo-buddy, how can I help you today?
        ### Human: How big is the sun?
        ### Robot: Sorry, I'm a robot, I can't see the sun.
        "
        );

    // To-do: Add Dummy messages to parse and add to dialog

    for message in conversation.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("Human: {msg}\n")
        } else {
            format!("Robot: {msg}\n")
        };
        context_dialog.push_str(&curr_line);
    }
    
    let model_path = "Wizard-Vicuna-7B-Uncensored.ggmlv3.q4_K_M.bin";
    // load a GGML model from disk
    let llama = llm::load::<Llama>(
        // path to GGML file
        &PathBuf::from(&model_path),
        llm::TokenizerSource::Embedded,
        
        Default::default(),
        // load progress callback
        llm::load_progress_callback_stdout
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

    // use the model to generate text from a prompt
    // let mut session = llama.start_session(Default::default());
    let session = Model::start_session(&llama, Default::default());

}

