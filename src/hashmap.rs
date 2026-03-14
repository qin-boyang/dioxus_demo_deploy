use dioxus::prelude::*;
use crate::footer::Footer;
use crate::header::Header;

#[component]
pub fn Hashmap() -> Element {
    let struct_code_content = r#"
        import java.util.HashMap;

        Map<String, Integer> map = new HashMap<>();
        map.putIfAbsent("Apple", 1);
        map.computeIfPresent("Apple", (key, val) -> val + 1);
    "#;

    let impl_code_content = r#"
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.entry("Apple").or_insert(1);
        map.entry("Apple").and_modify(|val| *val += 1);

        or

        let mut map = HashMap::new();
        map.entry("Apple")
            .and_modify(|val| *val += 1)
            .or_insert(1);
    "#;

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto font-sans text-gray-900",
            Header {}

            div { class: "border-b pb-6",
                p { class: "text-lg text-gray-300 italic",
                    "writing a Java HashMap counting by key"
                }
            }
            pre {
                class: "bg-gray-800 text-white p-4 rounded",
                code {
                    { struct_code_content }
                }
            }

            br {}
            br {}

            div { class: "border-b pb-6",
                p { class: "text-lg text-gray-300 italic",
                    "writing a Rush HashMap counting by key"
                }
            }
            pre {
                class: "bg-gray-800 text-white p-4 rounded",
                code {
                    { impl_code_content }
                }
            }

            br {}
            br {}
            
            Footer {}
        }
    }
}