

fn main() {
    test_bool();
    test_int();
    test_compare_int();
    test_float();
    test_bit_shift();
    test_logical_shift();
    test_arithmetic_shift();
}


fn test_int(){
    println!("加算：{}", 1 + 2);
    println!("減算：{}", 1 - 2);
    println!("乗算：{}", 1 * 2);
    println!("除算：{}", 1 / 2);
    println!("剰余：{}", 1 % 2);
}


fn test_compare_int(){
    println!("1=2等しい：{}", 1 == 2);
    println!("1!=2:等しくない：{}", 1 != 2);
    println!("1>2:大きい：{}", 1 > 2);
    println!("1<2:小さい：{}", 1 < 2);
    println!("大きいか等しい：{}", 1 >= 2);
    println!("小さいか等しい：{}", 1 <= 2);
}

fn test_bool(){
    println!("論理積(短絡評価)：{}", true && false);
    println!("論理和(短絡評価)：{}", true || false);
    println!("排他的論理和：{}", true ^ false);
    println!("否定：{}", !true);
    println!("論理積(非短絡評価)：{}", true & false);
    println!("論理和(非短絡評価)：{}", true | false);
}

fn test_float(){
    println!("float加算：{}", 1.0 + 2.0);
    println!("float減算：{}", 1.0 - 2.0);
    println!("float乗算：{}", 1.0 * 2.0);
    println!("float除算：{}", 1.0 / 2.0);
    println!("float剰余：{}", 1.0 % 2.0);
    println!("float比較：{}", 1.0 == 2.0);
    println!("float比較：{}", 1.0 == 1.00);
    println!("float比較：{}", 1.0 != 2.0);
    println!("float比較：{}", 1.0 > 2.0);
    println!("float比較：{}", 1.0 < 2.0);
    println!("float比較：{}", 1.0 >= 2.0);
    println!("float比較：{}", 1.0 <= 2.0);
}

fn test_bit_shift(){
    println!("左シフト：{}", 1 << 2);
    println!("右シフト：{}", 1 >> 2);
}

fn test_logical_shift(){
    let n: u8 = 0b0001_1000;
    println!("0b0001_1000 2ビット左シフト：{}", n << 2);
    println!("0b0001_1000 2ビット右シフト：{}", n >> 2);
}

fn test_arithmetic_shift(){
    let p: i8 = -64;
    println!("0b0001_1000 2ビット左算術シフト：{}", p << 2);
    println!("0b0001_1000 2ビット右算術シフト：{}", p >> 2);
}

fn test_bit(){
    let player: u16 = 1 | // 毒状態
    (1 << 1) | // 攻撃力アップ状態
    (568 << 2); // 
}