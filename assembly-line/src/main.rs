fn main() {
    let prod_time_slow = production_time_in_minutes(1);
    let prod_time_med = production_time_in_minutes(5);
    let prod_time_fast = production_time_in_minutes(10);

    println!("Production Times: {} {} {}", prod_time_slow, prod_time_med, prod_time_fast);

    let work_time_slow = working_items_per_minute(1);
    let work_time_med = working_items_per_minute(5);
    let work_time_fast = working_items_per_minute(10);
    
    println!("Working Times: {} {} {}", work_time_slow, work_time_med, work_time_fast)
}

fn production_time_in_minutes(speed: u8) -> f64 {
    if speed >= 1 && speed <= 4 {
	221.0 * speed as f64
    } else if speed >= 5 && speed <= 8 {
	198.9 * speed as f64
    } else {
	170.17 * speed as f64
    }
}

fn working_items_per_minute(speed: u8) -> u32 {
    production_time_in_minutes(speed) as u32 / 60
}
