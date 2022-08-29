fn main() {
    let time_left = expected_minutes_in_oven();
    println!("Expected Time: {}", time_left);

    let remaining_mins = remaining_minutes_in_oven(15);
    println!("Remaining Time: {}", remaining_mins);

    let preparation_time = preparation_time_in_minutes(5);
    println!("Preparation Time: {}", preparation_time);

    let elapsed_time = elapsed_time_in_minutes(5, 20);
    println!("Elasped Time: {}", elapsed_time);
}

fn expected_minutes_in_oven() -> i32 {
    40
}

fn remaining_minutes_in_oven(time: i32) -> i32 {
    40-time
}

fn preparation_time_in_minutes(layers: i32) -> i32 {
    layers*2
}

fn elapsed_time_in_minutes(layers: i32, time: i32) -> i32 {
    preparation_time_in_minutes(layers)+time
}
