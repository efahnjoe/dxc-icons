include!(concat!(env!("OUT_DIR"), "/mod.rs"));
use dioxus::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icons_collection() {
        // print the number of icons
        println!("Found {} icons", ICONS_COLLECTION.len());
        
        // Ensure that there is at least one icon
        assert!(!ICONS_COLLECTION.is_empty(), "No icons were generated!");
        
        // Check for duplicates
        let mut set = std::collections::HashSet::new();
        for name in ICONS_COLLECTION {
            assert!(set.insert(*name), "Duplicate icon name: {}", name);
        }
    }

    #[test]
    fn test_spawn_icon() {
        let mut success_count = 0;
        let mut fail_count = 0;

        for name in ICONS_COLLECTION.iter() {
            match spawn_icon(name) {
                Some(_icon) => {
                    success_count += 1;
                }
                None => {
                    fail_count += 1;
                    println!("Failed to load icon: {}", name);
                }
            }
        }

        println!("Icons loaded: {} success, {} failed", success_count, fail_count);
        assert_eq!(success_count, ICONS_COLLECTION.len());
        assert_eq!(fail_count, 0);
    }

    #[test]
    fn test_spawn_icon_invalid_name() {
        // Test invalid names return None
        assert!(spawn_icon("NonExistentIcon123").is_none());
        assert!(spawn_icon("").is_none());
    }

    #[test]
    fn test_all_icons_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for &name in ICONS_COLLECTION {
            assert!(seen.insert(name), "Duplicate icon name found: {}", name);
        }
    }
}