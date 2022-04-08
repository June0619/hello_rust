fn main() {

    // 변수 불변
    // let x = 5; Error : mut 키워드가 없으면 변수 값은 불변이다.
    let mut var1 = 5;
    println!("x의 값 : {}", var1);

    var1 = 6;
    println!("x의 값 : {}", var1);

    // 상수
    // 1. RUST 상수는 대문자 표현이 일반적
    // 2. 숫자 리터럴 선언에 가독성을 위해 '_' 사용 가능
    // 3. mut 키워드가 없더라도 상시 불변
    const MAX_POINTS: u32 = 100_000;
    println!("MAX POINTS 의 값 : {}", MAX_POINTS);

    // 변수 가리기(shadow)
    // 1. 할당 된 변수를 재 할당하여 가리는 것이 가능하다.
    // 2. 타입을 변경하여 가리는 것이 가능하다.
    let var2 = 5;
    let var2 = var2 + 5;
    let var2 = var2 * 2;
    println!("var2 의 값: {}", var2);

    let spaces = "                        ";
    let spaces = spaces.len();
    println!("spaces 의 값 : {}", spaces);

    // Error : mutable 변수라도 데이터 타입을 변경할 수는 없다.
    // let mut spaces = "                        ";
    // spaces = spaces.len();



}
