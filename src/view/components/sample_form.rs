use leptos::*;
use web_sys::{Event, HtmlInputElement};
use wasm_bindgen::JsCast;

fn is_valid_file_extension(file_name: &str) -> bool {
    let extension = file_name.rsplit('.').next();
    matches!(extension, Some("md") | Some("html"))
}

#[component]
pub fn WritingForm() -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (file, set_file) = create_signal(None::<HtmlInputElement>);
    let (file_name, set_file_name) = create_signal(String::new());
    let (error_message, set_error_message) = create_signal(String::new());

    let handle_file_change = move |event: Event| {
        let event = Event::new("change").unwrap();
        let target = event.target().unwrap();
        let input = target.dyn_into::<HtmlInputElement>().unwrap();
        set_file.set(Some(input.clone()));

        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let name = file.name();
                set_file_name.set(name);

                if is_valid_file_extension(&name) {
                    set_error_message.set(String::new());
                } else {
                    set_error_message.set("Invalid file type. Only .md and .html files are allowed.".to_string());
                }
            }
        }
    };
    
    view! {
        <div>
            <h1>"Sample Form"</h1>
            <div class="form-container">
                <form action="/submit-writing" method="post" enctype="multipart/form-data">
                    <div>
                      <label for="title">Title (required):</label>
                      <input type="text" id="title" name="title" required/>
                    </div>
                    
                    <div>
                      <label for="description">Description (required):</label>
                      <textarea id="description" name="description" required></textarea>
                    </div>
                    
                    <div>
                      <label for="content">Content (file):</label>
                      <input type="file" id="content" name="content" onchange= move |_| handle_file_change.clone()/>
                      <p>{file_name.get()}</p>
                    </div>
                    
                    <div>
                      <button type="submit">Submit</button>
                    </div>
                  </form>
            </div>
        </div>
    }
}