use leptos::*;
use web_sys::{Event, FormData, HtmlFormElement, HtmlInputElement, SubmitEvent};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestInit;

fn is_valid_file_extension(file_name: &str) -> bool {
    let extension = file_name.rsplit('.').next();
    matches!(extension, Some("md") | Some("html"))
}

#[component]
pub fn WritingForm() -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (_file, set_file) = create_signal(None::<HtmlInputElement>);
    let (_file_name, set_file_name) = create_signal(String::new());
    let (error_message, set_error_message) = create_signal(None::<String>);
    let (success_message, set_success_message) = create_signal(None::<String>);

    let handle_file_change = move |event: Event| {
        let target = event.target().expect("no event target");
        let input = target.dyn_into::<HtmlInputElement>().expect("failed to cast target to HtmlInputElement");
        set_file.set(Some(input.clone()));

        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let name = file.name();
                set_file_name.set(name.clone());

                if is_valid_file_extension(&name.clone()) {
                    set_error_message.set(None);
                } else {
                    set_error_message.set(Some("Invalid file type. Only .md and .html files are allowed.".to_string()));
                }
            }
        }
    };

    let handle_submit = move |event: SubmitEvent| {
        event.prevent_default();
        set_success_message.set(None);
        set_error_message.set(None);

        let form_data = FormData::new_with_form(&event.target().unwrap().dyn_into::<HtmlFormElement>().unwrap()).unwrap();

        let request = web_sys::Request::new_with_str_and_init(
            "/api/writing-sample",
            &RequestInit::new()
                .method("POST")
                .body(Some(&JsValue::from(form_data))),
        ).unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let resp_value = JsFuture::from(web_sys::window().unwrap().fetch_with_request(&request)).await;

            match resp_value {
                Ok(resp) => {
                    let resp: web_sys::Response = resp.dyn_into().unwrap();
                    let status = resp.status();

                    if status == 200 {
                        set_success_message.set(Some("Form submitted successfully!".to_string()));
                        set_title.set("".to_string());
                        set_description.set("".to_string());
                        set_file.set(None);
                        set_file_name.set("".to_string());
                    } else {
                        let text = JsFuture::from(resp.text().unwrap()).await.unwrap().as_string().unwrap();
                        set_error_message.set(Some(format!("Error: {}", text)));
                    }
                }
                Err(err) => {
                    set_error_message.set(Some(format!("Error: {:?}", err)));
                }
            }
        });
    };
    
    view! {
        <div class="writing-form">
            <h1>"Sample Form"</h1>
            <div class="form-container">
                <form on:submit=handle_submit>
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
                      <input type="file" id="content" name="content" required on:change= move |event|{handle_file_change(event)}/>
                    </div>
                    
                    <div>
                      <button type="submit">Submit</button>
                    </div>
                    <div>
                        <p>{move || error_message.get()}</p>
                        <p>{move || success_message.get()}</p>
                    </div>
                  </form>
            </div>
        </div>
    }
}