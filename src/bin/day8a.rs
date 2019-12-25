const INPUT: &str = include_str!("../../inputs/day8.txt");
const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const IMAGE_SIZE: usize = WIDTH*HEIGHT;

fn main() {
    let img: Vec<_> = INPUT.chars()
        .filter(|&c| c >= '0' && c <= '9')
        .collect();
    
    let num_layers = img.len() / IMAGE_SIZE;
    let (_, ones, twos) = (0..num_layers).map(|layer| {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = layer*IMAGE_SIZE + y*WIDTH + x;
                match img[index] {
                    '0' => zeros += 1,
                    '1' => ones += 1,
                    '2' => twos += 1,
                    _ => {},
                }
            }
        }
        (zeros, ones, twos)
    }).min_by_key(|&(zeros, _, _)| zeros).unwrap();

    println!("{}", ones*twos);
}
