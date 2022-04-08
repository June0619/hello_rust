fn main() {

    /** 변수 불변 **/
    // let x = 5; err : not mutable variable
    let mut x = 5;
    println!("x의 값 : {}", x);

    x = 6;
    println!("x의 값 : {}", x);

    /** 상수 **/
    // 1. RUST 상수는 대문자 표현이 일반적
    // 2. 숫자 리터럴 선언에 가독성을 위해 '_' 사용 가능
    // 3. mut 키워드가 없더라도 상시 불변
    const MAX_POINTS: u32 = 100_000;


}
