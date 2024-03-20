use leptos::{html::Input, *};
use web_sys::{Event, HtmlInputElement, SubmitEvent};
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
        let target = event.target().expect("no event target");
        let input = target.dyn_into::<HtmlInputElement>().expect("failed to cast target to HtmlInputElement");
        set_file.set(Some(input.clone()));

        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let name = file.name();
                set_file_name.set(name.clone());

                if is_valid_file_extension(&name.clone()) {
                    set_error_message.set(String::new());
                } else {
                    set_error_message.set("Invalid file type. Only .md and .html files are allowed.".to_string());
                }
            }
        }
    };
    
    view! {
        <div class="writing-form">
            <h1>"Sample Form"</h1>
            <div class="form-container">
                <form method="post" enctype="multipart/form-data">
                    <div class="form-input-title">
                      <label for="title">Title (required):</label>
                      <input type="text" id="title" name="title" required on:input=move |ev|{set_title.set(event_target_value(&ev))} prop:value=title/>
                    </div>
                    
                    <div class="form-input-description">
                      <label for="description">Description (required):</label>
                      <textarea id="description" name="description" required on:input=move |ev|{set_description.set(event_target_value(&ev))} prop:value=description></textarea>
                    </div>
                    
                    <div class="form-input-file">
                      <label for="content">Content (file):</label>
                      <input type="file" id="content" name="content" on:change= move |event|{handle_file_change(event)}/>
                      <p>{move || file_name.get()}</p>
                      <p>{move || error_message.get()}</p>
                    </div>
                    
                    <div>
                      <button type="submit">Submit</button>
                    </div>
                  </form>
            </div>
        </div>
    }
}