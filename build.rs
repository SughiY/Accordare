extern crate gcc;

fn main() {
        gcc::Config::new().file("src/Acc/Yin.c").compile("libYin.a");
}
