//converting seconds to hh::mm:ss
use std::io;

fn main() {
    let total_seconds = get_user_input();
    println!("Time in 24 hours format is : {}", sec_to_hhmmss(total_seconds));
}

fn sec_to_hhmmss(total_seconds: u32) -> String {
    if total_seconds > 86399{
        panic!("Your input should be between 0 and 86399"); 
    }
    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;
    let time_msg = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    return time_msg
}

fn read_from_stdin() -> String{
    let mut input = String::new();
    println!("Enter time in seconds (0-86399): ");
    io::stdin().read_line(&mut input).expect("failed to read line");
    input
}

fn parse_string_as_u32(input: String) -> u32{
    let total_seconds: u32 = input.trim().parse().expect("Input number only without any sign");
    total_seconds
} 

fn get_user_input() -> u32{
    parse_string_as_u32(read_from_stdin())
}

#[cfg(test)]
mod test{
    mod time_format{
        //1. when total_seconds is 0, function must return 00:00:00
        #[test]
        fn test_when_total_seconds_is_zero(){
            assert_eq!("00:00:00", super::super::sec_to_hhmmss(0)); //super = for parent
        }
        //2. when total_seconds is 86400, function must panic
        #[test]
        #[should_panic(expected = "Your input should be between 0 and 86399")]
        fn test_when_total_seconds_is_86400(){
            super::super::sec_to_hhmmss(86400);
        }
    }
    mod parse_user_input{
        // 1) user_enters zero, function must return 0 
        #[test]
        fn test_user_enters_zero_must_return_0(){
            assert_eq!(0, super::super::parse_string_as_u32("0\n\r".to_string()));
        }
    }
}