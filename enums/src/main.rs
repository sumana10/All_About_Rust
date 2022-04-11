enum Direction{
    Up,
    #[allow(dead_code)]
    Down,
    #[allow(dead_code)]
    Left,
    #[allow(dead_code)]
    Right
}

fn main() {
    
    let player_direction:Direction = Direction::Up;

    match player_direction {

        Direction::Up => println!("You are Up"),
        Direction::Down => println!("You are Down"),
        Direction::Left => println!("You are Left"),
        Direction::Right => println!("You are Right"),
    }
}
