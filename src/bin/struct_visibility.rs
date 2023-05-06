mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}
fn main() {
    let open_box = my::OpenBox {
        contents: "publice infomation",
    };

    println!("The open box contains : {}", open_box.contents);

    let _closed_box = my::ClosedBox::new("classified infomation");
}
