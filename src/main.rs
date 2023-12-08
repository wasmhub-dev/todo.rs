use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Window, Storage};
use web_sys::{HtmlInputElement, HtmlButtonElement, HtmlElement};
use gloo_events::EventListener;

mod utils;

#[derive(Clone, Debug, PartialEq, Eq)]
struct TodoApp {
    window: Window,
    document: Document,
    storage: Storage,
    input_box: HtmlInputElement,
    button: HtmlButtonElement,
    list: HtmlElement,
}

impl TodoApp {
    fn new() -> Self {
        let window: Window = window().expect_throw("no global `window` exists");
        let document = window.document().expect_throw("should have a document on window");

        let input_box: HtmlInputElement = document.get_element_by_id("input-box").unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let button = document.query_selector("button").unwrap().unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        let list = document.get_element_by_id("list-container").unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        let storage = window.local_storage().unwrap().unwrap();

        Self {
            window, 
            document,
            storage,
            input_box,
            button,
            list
        }
    }

    fn add_task(&self) {
        if self.input_box.value().is_empty() {
            let _ = self.window.alert_with_message("Please enter a task");
        } else {
            let li = self.document.create_element("li").unwrap();
            li.set_inner_html(&self.input_box.value());
            self.list.append_child(&li).unwrap();
            self.input_box.set_value("");
            let span = self.document.create_element("span").unwrap();
            span.set_inner_html("&times;");
            li.append_child(&span).unwrap();
        }
        self.save_data();
    }

    fn save_data(&self) {
        self.storage.set_item("todo_list", &self.list.inner_html()).unwrap();
    }

    fn show_task(&self) {
        let data = self.storage.get_item("todo_list");
        match data {
            Ok(Some(data)) => {
                self.list.set_inner_html(&data);
            },
            _ => {}
        }
    }
}


#[wasm_bindgen(main)]
pub fn main() {
    log!("Welcome to the wasm world!");
    let todo_app = Rc::new(TodoApp::new());

    { 
        let todo_app_clone = todo_app.clone();
        EventListener::new(&todo_app.list, "click", move |event| {
            let target = event.target().unwrap().dyn_into::<HtmlElement>().unwrap();
            if target.tag_name() == "LI" {
                let _ = target.class_list().toggle_with_force("checked", true);
                let _ = todo_app_clone.save_data();
            } else if target.tag_name() == "SPAN" {
                target.parent_element().unwrap().remove();
                let _ = todo_app_clone.save_data();
            }
        }).forget();
    }

    {
        let todo_app_clone = todo_app.clone();
        EventListener::new(&todo_app.button, "click", move |_| {
            let _ = todo_app_clone.add_task();
        }).forget();
    }

    todo_app.show_task();
}