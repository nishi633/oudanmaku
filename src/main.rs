fn main() {
    // 標準入力から入力値取得
    // https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
    let key = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_right().to_owned() // 改行コードが末尾にくっついてくるので削る
    };

    messages(&key);
    messages2();
}

fn messages(key: &str) {
    // https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html#%E3%83%8F%E3%83%83%E3%82%B7%E3%83%A5%E3%83%9E%E3%83%83%E3%83%97%E3%82%92%E6%9B%B4%E6%96%B0%E3%81%99%E3%82%8B
    use std::collections::HashMap;
    let mut map = HashMap::new();

    // https://qiita.com/smicle/items/29a4d5d1d14ad7f77f60
    map.insert("烏野",   "飛べ");
    map.insert("音駒",   "繋げ");
    map.insert("青城",   "コー トを制す");
    map.insert("白鳥沢", "強者 であれ");
    map.insert("伊達工", "鉄壁");

    //assert_eq!(map.get(&key), Some(&"a"));
    // getはSomeに包まれたOption<&V>を返却するため
    match map.get(key) {
        Some(v) => println!("【{}】", v),
        None => println!("none value"),
    };

    // HashMapの中身を表示
    //for (key, value) in &map {
    //    println!("{}: {}", key, value);
    //}
}

fn messages2() {
    // https://doc.rust-lang.org/std/primitive.array.html
    let a = [
        "飛べ",
        "繋げ",
        "コートを制す",
        "強者であれ",
        "鉄壁"
    ];

    // https://rust-random.github.io/rand/rand/trait.Rng.html#method.gen_range
    use rand::{thread_rng, Rng};
    let mut rng = rand::thread_rng();
    let max = if a.len() > 0  { a.len() - 1 } else {0};
    println!("{}", a[rng.gen_range(0..max)]);
}
