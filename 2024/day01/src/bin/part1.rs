fn main() {
    let contents = include_str!("input.txt");
    let lines = contents.lines();
    let mut list1 = Vec::<u32>::new();
    let mut list2 = Vec::<u32>::new();
    for line in lines {
        let list_parts = line.split_whitespace().collect::<Vec<&str>>();
        let num1 = list_parts[0].parse::<u32>().unwrap();
        let num2 = list_parts[1].parse::<u32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }
    list1.sort();
    list2.sort();

    let mut sum = 0;
    for i in 0..list1.len() {
        sum += list1[i].abs_diff(list2[i]);
    }
    println!("Sum of differences: {}", sum);
}
