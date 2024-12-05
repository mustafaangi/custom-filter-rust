// Define the filter condition struct
struct FilterCondition<T> {
    value: T,
}

// Implement the matching logic
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        self.value == *item
    }
}

// Define the filtering function
fn custom_filter<T: PartialEq>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T> 
where T: Clone {
    let mut result = Vec::new();
    
    for item in collection {
        if condition.is_match(item) {
            result.push(item.clone());
        }
    }
    
    result
}

fn main() {
    // Test with integers
    let numbers = vec![1, 2, 3, 4, 5, 2, 3, 2];
    let condition = FilterCondition { value: 2 };
    let filtered_numbers = custom_filter(&numbers, &condition);
    println!("Filtered numbers: {:?}", filtered_numbers);

    // Test with strings
    let words = vec!["apple", "banana", "apple", "cherry", "date", "apple"];
    let string_condition = FilterCondition { value: "apple" };
    let filtered_words = custom_filter(&words, &string_condition);
    println!("Filtered words: {:?}", filtered_words);
}