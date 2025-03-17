use yew::prelude::*; // Import necessary parts of Yew
use wasm_bindgen::prelude::*;
use web_sys::{File,  HtmlInputElement}; // Import the necessary web_sys types

#[function_component]
pub fn App() -> Html {
    let file = use_state(|| None); // State to store the uploaded file

    // Handle file input change
    let onfile_change = {
        let file = file.clone();
        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                if let Some(f) = files.get(0) {
                    file.set(Some(f)); // Store the selected file in state
                }
            }
        }
    };

    // Handle file upload when the upload button is clicked
    let onupload_click = {
        let file = file.clone();
        move |_| {
            if let Some(f) = (*file).clone() {
                upload_file_to_server(f); // Send the file to the server
            } else {
                log::warn!("No file selected.");
            }
        }
    };

    html! {
        <div>
            <h3>{"Upload an Excel file"}</h3>
            <input type="file" accept=".xlsx" onchange={onfile_change} />
            <button onclick={onupload_click}>{"Upload"}</button>  // Pass closure directly to onclick
        </div>
    }
}

// Function to upload the file to the server
fn upload_file_to_server(_file: File) {
    // let form_data = FormData::new().unwrap();
    // form_data.append_with_blob("file", &file).unwrap();

    // // Send a POST request with the file data
    // let request = web_sys::window().unwrap().fetch_with_str_and_init(
    //     "/api/upload", // API endpoint
    //     &web_sys::RequestInit::new()
    //         .method("POST")
    //         .body(Some(&form_data))
    // );

    // // Handle the server response
    // let _ = request.then(|response| {
    //     if response.ok() {
    //         log::info!("File uploaded successfully");
    //     } else {
    //         log::error!("Failed to upload file");
    //     }
    //     Ok(())
    // });
    log::info!("File uploaded successfully");
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Info).expect("error initializing log");
    log::info!("App has started!"); // Log a message for debugging
    yew::Renderer::<App>::new().render();  
}
