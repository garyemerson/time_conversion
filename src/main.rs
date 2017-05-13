use std::io;

fn main() {
    let time = get_line();
    let time = time.trim();
    let mut hour = time.chars().take(2).collect::<String>().parse::<i32>().unwrap();
    let am_pm = time.chars().skip(8).collect::<String>();

    if am_pm == "PM" && hour != 12 {
        hour += 12;
    } else if am_pm == "AM" && hour == 12 {
        hour = 0;
    }
    
    let hour_str: String = format!("{:02}", hour);
    let military_time = hour_str + &time.chars().skip(2).take(6).collect::<String>();
    println!("{}", military_time);
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
