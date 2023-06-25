use helpers::read_file;

fn form_groups(contents: String) -> Vec<Vec<u32>> {
    let mut groups = Vec::new();
    for group in contents.split("\n\n") {
        let mut group_numbers = Vec::new();
        for num in group.split("\n") {
            let parsed_num = num.parse::<u32>();
            if parsed_num.is_err() {
                continue;
            }
            group_numbers.push(parsed_num.unwrap());
        }
        groups.push(group_numbers);
    }
    groups
}

fn sum_groups(groups: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut sums = Vec::new();
    for group in groups {
        let mut sum = 0;
        for num in group {
            sum += num;
        }
        sums.push(sum);
    }
    sums
}

fn main() {
    let input = read_file("./day_1/input.txt");
    let groups = form_groups(input);
    let mut sums = sum_groups(&groups);
    sums.sort();
    sums.reverse();
    let max_three_values = sums.get(0..3).unwrap().to_vec();
    let total_sum = max_three_values.iter().sum::<u32>();
    println!("The three largest values summed are: {:?}", total_sum);
}
