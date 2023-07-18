use llm::models::Llama;
use llm::KnownModel;



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
    let data: Llama;
    // data.into_inner()

    let conversation = Conversation::new();
}

