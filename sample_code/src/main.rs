
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("1 ~ 101 사이의 숫자 맞추기");

    let secret_number = rand::thread_rng().gen_range(1 .. 101);
    println!("정답이라고 생각하는 숫자를 입력하세요!");



    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("입력한 값을 읽지 못했습니다.");
        let guess_number:u32 = guess.trim().parse().expect("입력한 값이 숫자가 아닙니다.");
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다."),
            Ordering::Greater => println!("입력한 숫자가 큽니다."),
            Ordering::Equal =>{
                println!("정답");
                break;
            } ,
        }
    }

}

