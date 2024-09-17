use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Component)]
struct Tile {
    number: u8,
}

#[derive(Component)]
struct Position {
    x: u8,
    y: u8,
}

const WIDTH: u8 = 5;
const HEIGHT: u8 = 5;

fn main() {
    App::new()
        .add_systems(Startup, (setup, print_table).chain())
        .run();
}

fn setup(mut commands: Commands) {
    let mut numbers: Vec<u8> = (1u8..WIDTH*HEIGHT+1).collect();
    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);
    let table = numbers.chunks(WIDTH as usize).collect::<Vec<_>>();

    for (y, row) in table.iter().enumerate() {
        for (x, &number) in row.iter().enumerate() {
            commands.spawn((Tile { number }, Position { x: x as u8, y: y as u8 }));
        }
    }
}

fn print_table(query: Query<(&Tile, &Position)>) {
    let mut table = [[1u8; WIDTH as usize]; HEIGHT as usize];
    for (tile, pos) in query.iter() {
        table[pos.y as usize][pos.x as usize] = tile.number;
    }

    for row in table.iter() {
        for &num in row.iter() {
            let to_print = format!("{:>2} ", num);
            print!("{}", to_print);
        }
        println!();
    }
}

