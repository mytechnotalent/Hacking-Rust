fn another_function(arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: i32, arg7: i32, arg8: i32, arg9: i32, arg10: i32, arg11: i32, arg12: i32) -> String {
    let result = format!("You passed: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12);

    result
}

fn main() {
    let returned_value = another_function(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("The function returned: {}", returned_value);
}