# Rust_bootcamp_project2
 The task is about creating a custom filtering function in Rust. The purpose of this function is to filter out elements from a collection (like a list of numbers or a list of strings) based on a specific condition. 

 1.	Open the main.rs file in a text editor.
2.	Define a struct called FilterCondition with a single field of the desired type for filtering.
3.	Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.
4.	Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
5.	In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.
6.	Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
7.	Print the filtered result to the console.
8.	Compile and run the program to test its functionality.
   
FilterCondition struct: This is a custom data type you're creating. It will hold the condition for filtering. For example, if you're working with a list of integers, the FilterCondition might hold a number, say 10. The purpose of this number is to act as a benchmark for filtering the list.

is_match method: This is a method you'll define on the FilterCondition struct. It will take an item from the list and check if it matches the condition. For example, if the FilterCondition is 10, the is_match method might check if the item is greater than 10. If it is, the method will return true; otherwise, it will return false.
custom_filter function: This is the main function you'll create. It will take a list and a FilterCondition as input. It will go through each item in the list and use the is_match method to check if the item should be included in the output. If is_match returns true, the item will be added to the output list. If is_match returns false, the item will be ignored.
Here's an example to illustrate the functionality:
Suppose you have a list of numbers: [5, 10, 15, 20, 25]. You want to filter out all numbers that are greater than 10. Here's how you'd use the structures and functions you're creating:
You'd create a FilterCondition with the value 10. This represents the condition for filtering.
You'd call the custom_filter function with the list of numbers and the FilterCondition you created.
The custom_filter function would go through each number in the list. For each number, it would call the is_match method on the FilterCondition.
The is_match method would check if the number is greater than the FilterCondition (which is 10). If the number is greater than 10, is_match would return true.
If is_match returns true, the custom_filter function would add the number to the output list.
After going through all the numbers, the custom_filter function would return the output list.
So, if you started with the list [5, 10, 15, 20, 25] and a FilterCondition of 10, the custom_filter function would return [15, 20, 25], because those are the numbers that are greater than 10.

