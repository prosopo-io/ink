#[ink_lang::contract]
mod noop {
    use ink_lang as ink;

    #[ink::trait_definition]
    pub trait Flipper {
        // only `#[ink(message)]` and `#[ink(constructor)]` allowed in trait definition
        #[ink(message)]
        fn flip(&mut self);
    }

    #[ink(storage)]
    pub struct Noop {
        flipped: bool,
    }

    #[ink(impl)]
    #[ink(namespace = "NoopNamespace")]
    impl Noop {
        #[ink(constructor)]
        #[ink(label = "new_flipper")]
        pub fn new() -> Self {
            Self { flipped: false }
        }
    }

    #[ink(label = "CustomFlipper")]
    #[ink(namespace = "FlipperNamespace")]
    impl Flipper for Noop {
        #[ink(message)]
        #[ink(label = "lets_flip")]
        #[ink(payable)]
        #[ink(selector = "0x11223344")]
        fn flip(&mut self) {
            self.flipped = !self.flipped;
        }
    }
}

fn main() {}
