use llm::models::Llama;
use llm::Model;
use std::path::PathBuf;
use std::convert::Infallible;
use std::io;


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


fn inference_callback<'a>(
    stop_sequence: String,
    buf: &'a mut String,
    out_str: &'a mut String,
) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
    use llm::InferenceFeedback::Halt;
    use llm::InferenceFeedback::Continue;
    move |resp| match resp {
        llm::InferenceResponse::InferredToken(t) => {
            let mut reverse_buf = buf.clone();
                reverse_buf.push_str(t.as_str());
            if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                buf.clear();
                return Ok::<llm::InferenceFeedback, Infallible>(Halt);
            } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                buf.push_str(t.as_str());
                return Ok(Continue);
            }

            if buf.is_empty() {
                out_str.push_str(&t);
            } else {
                out_str.push_str(&reverse_buf);
            }

            Ok(Continue)
        }
        llm::InferenceResponse::EotToken => Ok(Halt),
         _ => Ok(Continue),
    }
}

fn ask_chatbot(chat: String) {
    let conversation = Conversation::new();
    
    let chatbot_persona = "A chat between a human and an assistant\n";
    let mut context_dialog = format!(
        "### Robot: What questions do you have today??
        ### Human: What is the capital of Australia?
        ### Robot: The capital city of Australia is Canberra.
        "
        );

    context_dialog.push_str(format!("### Human: {chat}").as_str());

    // To-do: Add Dummy messages to parse and add to dialog

    for message in conversation.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("### Human: {msg}\n")
        } else {
            format!("### Robot: {msg}\n")
        };
        context_dialog.push_str(&curr_line);
    }
    
    let model_path = "Wizard-Vicuna-7B-Uncensored.ggmlv3.q4_K_M.bin";
    // load a GGML model from disk
    let mut llama = llm::load::<Llama>(
        // path to GGML file
        &PathBuf::from(&model_path),
        llm::TokenizerSource::Embedded,
        
        Default::default(),
        // load progress callback
        llm::load_progress_callback_stdout
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
    
    let prompt_text = format!("{chatbot_persona}\n{context_dialog}\n### Robot:").to_string();
    let prompt = llm::Prompt::Text(&prompt_text); 


    let mut request = llm::InferenceRequest {
        prompt: prompt,
        parameters: &llm::InferenceParameters::default(),
        play_back_previous_tokens: false,
        maximum_token_count: None
    };
    

    let mut buf = String::new();
    let mut res = String::new();
    let inference_callback = inference_callback(
        String::from("### Human:"), &mut buf, &mut res
    );

    let mut session = Model::start_session(&llama, Default::default());
    let mut rng = rand::thread_rng();

    session.infer(
        &mut llama,
        &mut rng,
        &mut request,
        &mut Default::default(),
        inference_callback
        ).unwrap_or_else(|e| panic!("{e}"));

    println!("{}",res);

}

fn main(){
    
    println!("What would you like to ask the chatbot?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    ask_chatbot(input);

}
