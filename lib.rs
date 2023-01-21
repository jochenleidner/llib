//=============================================================================
// "lib.rs"
//
// Part of LLib™ - A lightweight and Free Rust Software Library
//
// Copyright ©2023 by Jochen L. Leidner <leidner¬acm.org>. All rights reserved.
//=============================================================================

pub mod llib {

    pub fn print_copyright() {
        eprint!("Copyright c2023 by Jochen L. Leidner. All rights reserved.");
    }

    pub mod io {
    }

    pub mod string {
    }

    pub mod text {
    }

}

//-- unit tests ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::llib::print_copyright;

    #[test]
    fn it_works() {
        print_copyright();
    }
}

//=============================================================================
// end of file "lib.rs"
//=============================================================================
