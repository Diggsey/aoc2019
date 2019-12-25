const INPUT: (i32, i32) = (128392, 643281);

fn main() {
    let mut candidate = INPUT.0;
    let mut count = 0;
    while candidate < INPUT.1 {
        let mut divisor = 100000;
        let mut prev_digit = 0;
        let mut has_double = false;
        for _ in 0..6 {
            let mut digit = (candidate / divisor) % 10;
            while digit < prev_digit {
                candidate -= candidate % divisor;
                candidate += divisor;
                digit += 1;
            }
            if digit == prev_digit {
                has_double = true;
            }
            prev_digit = digit;
            divisor /= 10;
        }

        if has_double && candidate < INPUT.1 {
            count += 1;
            println!("{}", candidate);
        }
        candidate += 1;
    }
    println!("{}", count);
}
