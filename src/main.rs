// Define the FilterCondition struct with the desired type for filtering.
struct FilterCondition<T> {
    limit: T,
}

// Implement the is_match method on the FilterCondition struct.
impl<T: PartialOrd> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        *item > self.limit
    }
}

// Define the custom_filter function to filter elements based on the condition.
fn custom_filter<T>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T>
    where
        T: Clone + PartialOrd, // Add PartialOrd here
{
    collection.into_iter().filter(|item| condition.is_match(item)).collect()
}

fn main() {
    // Create a collection and a FilterCondition object.
    let numbers = vec![5, 10, 15, 20, 25];
    let filter_condition = FilterCondition { limit: 10 };

    // Call the custom_filter function and store the result.
    let filtered_numbers = custom_filter(numbers, &filter_condition);

    // Print the filtered result to the console.
    println!("Filtered numbers: {:?}", filtered_numbers);
}
