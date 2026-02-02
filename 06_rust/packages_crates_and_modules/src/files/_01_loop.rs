pub fn loop_task(){
    let mut index :i32 = 0;
    println!("PRINT ALL ODD NUMBERS FROM 1 TO 10");
    loop{
        index += 1;
        if index == 10 {
            break;
        }
        if index % 2 == 0 {
            continue;
        }
        print!("{index} ");
    }

    println!();
    println!("PRINT ALL EVEN NUMBERS FROM 1 TO 10");

    for i in 1..=1000{
        if i % 2 != 0 {
            continue;
        }
        if i > 10 {
            break;
        }
        print!("{i} ")
    }

    println!();
    println!("PRINT ALL ODD NUMBERS FROM 1 TO 10");

    let mut index = 0;
    while index < 1000 {
        index += 1;
        if index > 10 {
            break;
        }
        if index % 2 == 0 {
            continue;
        }
        print!("{index} ");
    }
}