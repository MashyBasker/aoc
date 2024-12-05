pub mod part1 {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    /// This function reads the input from the file `day1.in`
    /// Returns: Tuple(Array, Array)
    /// An array can be used here because the size of the file is known before compile time(1000 lines)
    /// This makes the code more efficient and fast. However, for the situation when the size
    /// of the file is unknown, a vector or any other dynamic data structure should be used.
    ///
    /// After reading the line, the function gets the numbers and converts them each to u16 and put them
    /// in their respective arrays.
    /// As we need to keep finding the next smallest element of the list of the location IDs, we will be
    /// sorting the arrays in ascending order.
    pub fn preprocess_input(path: &str) -> io::Result<([i32; 1000], [i32; 1000])> {
        let file = File::open(&path)?;
        let file_reader = BufReader::new(file);
        let mut left_array: [i32; 1000] = [0; 1000];
        let mut right_array: [i32; 1000] = [0; 1000];

        for (index, line) in file_reader.lines().enumerate() {
            let line = line?;
            let line_info: Vec<&str> = line.split_whitespace().collect();

            if line_info.len() >= 2 {
                left_array[index] = line_info[0].parse::<i32>().unwrap_or(0);
                right_array[index] = line_info[1].parse::<i32>().unwrap_or(0);
            } else {
                eprintln!("Warning! Malformed line at line {}: {:?}", index + 1, line);
            }
        }
        left_array.sort();
        right_array.sort();
        Ok((left_array, right_array))
    }

    /// This function takes the 2 sorted arrays recieved from the function `preprocess_input`
    /// The function uses an accumulator pattern to accumulate the sums of the absolute differences of each element
    /// of the arrays. Since the arrays are pre-sorted we do not have to worry about finding the next smallest elements
    /// as iterating the array is guranteed to get the next smallest array
    ///
    /// Similar to,
    /// loop {
    /// 	acc = acc + abs(left_array_sorted[i] - right_array_sorted[i])
    /// }
    pub fn distance_sum(left_array: [i32; 1000], right_array: [i32; 1000]) -> i32 {
        left_array
            .iter()
            .zip(right_array.iter())
            .fold(0, |acc, (l, r)| acc + (l - r).abs())
    }

    /// Driver function for Day 1 Part 1 of AoC '24
    pub fn driver(input: &str) -> i32 {
        let (left, right) = preprocess_input(input).unwrap();
        distance_sum(left, right)
    }
}


pub mod part2 {
    use crate::day1::sol::part1::preprocess_input;

    pub fn binary_search_frequency(element: i32, array: [i32; 1000]) -> i32 {
        let lower_bound = array.partition_point(|&x| x < element);
        let upper_bound = array.partition_point(|&x| x <= element);
        if lower_bound == upper_bound {
            0
        } else {
            (upper_bound - lower_bound) as i32
        }
    }

    pub fn find_similarity_score(left_array: [i32; 1000], right_array: [i32; 1000]) -> i32 {
        let mut score = 0;
        for ele in left_array {
            let freq = binary_search_frequency(ele, right_array);
            score += ele * freq;
        }
        score
    }

    pub fn driver(input:&str) -> i32 {
        let (left, right) = preprocess_input(input).unwrap();
        find_similarity_score(left, right)
    }

}