use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;
use js_sys::Function;
use web_sys::{ScrollIntoViewOptions, ScrollBehavior};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct CodeAnalysis {
    code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct DetectedAlgorithm {
    name: String,
    category: String,
    complexity: String,
    description: String,
    wikipedia_link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct DetectedDataStructure {
    name: String,
    category: String,
    complexity: String,
    description: String,
    wikipedia_link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct AnalysisResult {
    patterns: Vec<String>,
    algorithms: Vec<String>,
    detailed_algorithms: Vec<DetectedAlgorithm>,
    data_structures: Vec<String>,
    detailed_data_structures: Vec<DetectedDataStructure>,
    complexity: String,
    recommendations: Vec<String>,
}

// Add the entry point for the WASM module
#[wasm_bindgen(start)]
pub fn main() {
    // Instead of rendering a Yew component that replaces the HTML, we'll just initialize our event handlers
    initialize_app();
}

fn initialize_app() {
    // Check API status
    let window = web_sys::window().unwrap();
    let _document = window.document().unwrap();
    
    // Set a timeout to simulate API connection and update status
    let callback = Closure::once_into_js(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        if let Some(status_element) = document.get_element_by_id("api-status") {
            // Check if we're running on GitHub Pages
            let location = window.location();
            let host = location.host().unwrap_or_default();
            
            if host.contains("github.io") {
                status_element.set_inner_html(
                    "<i class=\"fas fa-info-circle\"></i>
                    <span>Demo Mode - Sample Results</span>"
                );
            } else {
                status_element.set_inner_html(
                    "<i class=\"fas fa-check-circle\"></i>
                    <span>Connected to analysis engine</span>"
                );
            }
        }
        
        // Add event listener to the analyze button
        if let Some(btn) = document.get_element_by_id("analyze-btn") {
            let btn: &web_sys::HtmlElement = btn.dyn_ref().unwrap();
            
            let analyze_callback = Closure::wrap(Box::new(move || {
                analyze_code();
            }) as Box<dyn FnMut()>);
            
            let func: &Function = analyze_callback.as_ref().unchecked_ref();
            let _ = btn.add_event_listener_with_callback("click", &func.clone());
            
            // Keep the callback alive by forgetting it
            analyze_callback.forget();
        }
    });
    
    let func: &Function = callback.as_ref().unchecked_ref();
    window.set_timeout_with_callback_and_timeout_and_arguments_0(&func.clone(), 1500).unwrap();
}

// Function to analyze code when the button is clicked
fn analyze_code() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Get the code from the textarea
    let code_input = document.get_element_by_id("code-input").unwrap();
    let code_input: web_sys::HtmlTextAreaElement = code_input.dyn_into().unwrap();
    let code = code_input.value();
    
    // Show loading state
    if let Some(btn) = document.get_element_by_id("analyze-btn") {
        let btn: web_sys::HtmlElement = btn.dyn_into().unwrap();
        btn.set_inner_html("<i class=\"fas fa-spinner fa-spin\"></i> Analyzing...");
        btn.set_attribute("disabled", "").unwrap();
    }
    
    // Check if we're running on GitHub Pages
    let location = window.location();
    let host = location.host().unwrap_or_default();
    let is_github_pages = host.contains("github.io");
    
    if is_github_pages {
        // Show demo results
        show_demo_results(&code);
    } else {
        // Make actual API call
        let future = async move {
            let result = call_analyze_api(&code).await;
            update_ui_with_results(result);
        };
        wasm_bindgen_futures::spawn_local(future);
    }
}

// Function to show demo results for GitHub Pages
fn show_demo_results(code: &str) {
    let window = web_sys::window().unwrap();
    let _document = window.document().unwrap();
    
    // Simulate analysis delay
    let callback = Closure::once_into_js(move || {
        let result = create_demo_result(code);
        update_ui_with_results(Ok(result));
    });
    
    let func: &Function = callback.as_ref().unchecked_ref();
    window.set_timeout_with_callback_and_timeout_and_arguments_0(&func.clone(), 1500).unwrap();
}

// Function to create demo results based on the code
fn create_demo_result(_code: &str) -> AnalysisResult {
    AnalysisResult {
        patterns: vec![
            "Nested loop pattern".to_string(),
            "Array traversal".to_string(),
            "Element swapping".to_string(),
            "Conditional comparison".to_string()
        ],
        algorithms: vec![
            "Bubble Sort".to_string(),
            "Array Printing".to_string()
        ],
        detailed_algorithms: vec![
            DetectedAlgorithm {
                name: "Bubble Sort".to_string(),
                category: "Sorting".to_string(),
                complexity: "O(n²)".to_string(),
                description: "A simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order.".to_string(),
                wikipedia_link: "https://en.wikipedia.org/wiki/Bubble_sort".to_string()
            }
        ],
        data_structures: vec![
            "Vector (Dynamic Array)".to_string(),
            "Primitive Types (i32)".to_string()
        ],
        detailed_data_structures: vec![
            DetectedDataStructure {
                name: "Vector".to_string(),
                category: "Dynamic Array".to_string(),
                complexity: "O(1) access, O(n) insertion".to_string(),
                description: "A contiguous growable array type with heap-allocated contents.".to_string(),
                wikipedia_link: "https://en.wikipedia.org/wiki/Dynamic_array".to_string()
            }
        ],
        complexity: "Medium".to_string(),
        recommendations: vec![
            "Consider using more efficient sorting algorithms like Quick Sort or Merge Sort for larger datasets".to_string(),
            "The current implementation has O(n²) time complexity".to_string(),
            "For better performance with large arrays, consider using the built-in sort() method".to_string(),
            "To analyze your own code: Follow the setup instructions to run ICALDS locally".to_string()
        ]
    }
}

// Function to update UI with results
fn update_ui_with_results(result: Result<AnalysisResult, String>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    match result {
        Ok(analysis) => {
            // Show results container
            if let Some(container) = document.get_element_by_id("results-container") {
                container.set_attribute("style", "display: block;").unwrap();
            }
            
            // Update complexity badge
            if let Some(badge) = document.get_element_by_id("complexity-badge") {
                let class = match analysis.complexity.as_str() {
                    "High" => "badge badge-danger",
                    "Medium" => "badge badge-warning",
                    "Low" => "badge badge-success",
                    _ => "badge badge-success"
                };
                badge.set_attribute("class", class).unwrap();
                badge.set_inner_html(&format!("Complexity: {}", analysis.complexity));
            }
            
            // Update patterns list
            if let Some(list) = document.get_element_by_id("patterns-list") {
                let mut html = String::new();
                for pattern in &analysis.patterns {
                    html.push_str(&format!("<li>{}</li>", pattern));
                }
                list.set_inner_html(&html);
            }
            
            // Update algorithms list
            if let Some(list) = document.get_element_by_id("algorithms-list") {
                let mut html = String::new();
                for algorithm in &analysis.algorithms {
                    html.push_str(&format!("<li>{}</li>", algorithm));
                }
                list.set_inner_html(&html);
            }
            
            // Update detailed algorithms list
            if let Some(list) = document.get_element_by_id("detailed-algorithms-list") {
                let mut html = String::new();
                for algorithm in &analysis.detailed_algorithms {
                    html.push_str(&format!(
                        "<li><strong>{}</strong> ({})<br><small>{}</small><br><a href=\"{}\" target=\"_blank\">Learn more on Wikipedia</a></li>",
                        algorithm.name, algorithm.category, algorithm.description, algorithm.wikipedia_link
                    ));
                }
                list.set_inner_html(&html);
            }
            
            // Update data structures list
            if let Some(list) = document.get_element_by_id("data-structures-list") {
                let mut html = String::new();
                for data_structure in &analysis.data_structures {
                    html.push_str(&format!("<li>{}</li>", data_structure));
                }
                list.set_inner_html(&html);
            }
            
            // Update detailed data structures list
            if let Some(list) = document.get_element_by_id("detailed-data-structures-list") {
                let mut html = String::new();
                for data_structure in &analysis.detailed_data_structures {
                    html.push_str(&format!(
                        "<li><strong>{}</strong> ({})<br><small>{}</small><br><a href=\"{}\" target=\"_blank\">Learn more on Wikipedia</a></li>",
                        data_structure.name, data_structure.category, data_structure.description, data_structure.wikipedia_link
                    ));
                }
                list.set_inner_html(&html);
            }
            
            // Update recommendations list
            if let Some(list) = document.get_element_by_id("recommendations-list") {
                let mut html = String::new();
                for recommendation in &analysis.recommendations {
                    html.push_str(&format!("<li>{}</li>", recommendation));
                }
                list.set_inner_html(&html);
            }
        }
        Err(e) => {
            // Show error in recommendations
            if let Some(list) = document.get_element_by_id("recommendations-list") {
                list.set_inner_html(&format!("<li>Error: {}</li>", e));
            }
            if let Some(container) = document.get_element_by_id("results-container") {
                container.set_attribute("style", "display: block;").unwrap();
            }
        }
    }
    
    // Reset button
    if let Some(btn) = document.get_element_by_id("analyze-btn") {
        let btn: web_sys::HtmlElement = btn.dyn_into().unwrap();
        btn.set_inner_html("<i class=\"fas fa-search\"></i> Analyze Code");
        btn.remove_attribute("disabled").unwrap();
    }
    
    // Scroll to results
    if let Some(container) = document.get_element_by_id("results-container") {
        let options = ScrollIntoViewOptions::new();
        options.set_behavior(ScrollBehavior::Smooth);
        let _ = container.scroll_into_view_with_scroll_into_view_options(&options);
    }
}

// Function to call the backend API
async fn call_analyze_api(code: &str) -> Result<AnalysisResult, String> {
    let window = web_sys::window().ok_or("Failed to get window object")?;
    let location = window.location();
    
    // Try to get the origin, fallback to localhost if not available
    let origin = location.origin().unwrap_or("http://localhost:8081".to_string());
    
    // Replace the port to point to the API server (assuming it runs on 8081)
    let api_url = if origin.contains(":8080") {
        origin.replace(":8080", ":8081") + "/analyze"
    } else {
        "http://localhost:8081/analyze".to_string()
    };
    
    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(web_sys::RequestMode::Cors);
    
    let analysis_data = CodeAnalysis { code: code.to_string() };
    let json_data = serde_json::to_string(&analysis_data).map_err(|e| e.to_string())?;
    
    let js_value = wasm_bindgen::JsValue::from_str(&json_data);
    opts.set_body(&js_value);
    
    let request = web_sys::Request::new_with_str_and_init(&api_url, &opts)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;
    
    request.headers().set("Content-Type", "application/json")
        .map_err(|e| format!("Failed to set headers: {:?}", e))?;
    
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Failed to fetch: {:?}", e))?;
    
    let resp: web_sys::Response = resp_value.dyn_into().map_err(|_| "Failed to cast response")?;
    
    let text_promise = resp.text()
        .map_err(|e| format!("Failed to get text promise: {:?}", e))?;
    
    let text_js_value = JsFuture::from(text_promise)
        .await
        .map_err(|e| format!("Failed to get response text: {:?}", e))?;
    
    let text = text_js_value.as_string()
        .ok_or("Failed to convert response to string")?;
    
    if resp.ok() {
        let result: AnalysisResult = serde_json::from_str(&text)
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(result)
    } else {
        Err(format!("API Error ({}): {}", resp.status(), text))
    }
}