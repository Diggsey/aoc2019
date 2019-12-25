const INPUT: &str = include_str!("../../inputs/day8.txt");
const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const IMAGE_SIZE: usize = WIDTH*HEIGHT;

fn main() {
    let img: Vec<_> = INPUT.chars()
        .filter(|&c| c >= '0' && c <= '9')
        .collect();
    
    let mut res = vec!['2'; IMAGE_SIZE];

    let num_layers = img.len() / IMAGE_SIZE;
    for layer in 0..num_layers {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let layer_index = y*WIDTH + x;
                if res[layer_index] == '2' {
                    let index = layer*IMAGE_SIZE + layer_index;
                    res[layer_index] = img[index];
                }
            }
        }
    }
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let layer_index = y*WIDTH + x;
            let c = match res[layer_index] {
                '0' => ' ',
                '1' => '#',
                _ => ' ',
            };
            print!("{}", c);
        }
        println!();
    }
}
