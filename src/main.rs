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
    
    // To-do: "query" the LLM bellow
}

