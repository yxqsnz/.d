pub fn input(question: impl ToString) -> String {
    use std::io::{stdin, stdout, Write};
    let q = question.to_string();
    print!("{q}");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let buf = buf.trim().to_string();
    buf
}
macro_rules! mlang {
    ([let $i:tt] $x:tt :: $e:expr; $($tt:tt)*) => {
        let $i = [$x, $e].concat();
        mlang!($($tt)*);
    };
    ([let $i:tt as $ty:ty] $e:expr; $($tt:tt)*) => {
        let $i:$ty = $e;
        mlang!($($tt)*);

    };
    ([let $i:tt] $e:expr; $($tt:tt)*) => {
        let $i = $e;
        mlang!($($tt)*);
    };
    (
        [for $it:tt in $list:ident]
        $(
            [$i:tt $($o:tt)?] $($arg:tt)?$(.)?
        );+
        [end];
        $(
            $tt2:tt
        )*
    ) => {
        for $it in $list {
            $(
                mlang!([$i $($o)?] $($arg)?;);
            )*
        }
        mlang!($($tt2)*);
    };
    (
        [rust]
        $rust_code:block
        [end];
        $(
            $tt:tt
        )*
    ) => {
        $rust_code;
        mlang!($($tt)*);
    };
    ([writeln out] $($var:tt),+; $($tt:tt)*) => {
        {
            let fmt = format!($($var)*);
            println!("{fmt}");
        }
        mlang!($($tt)*);
    };
    ([write out] $($var:tt),+; $($tt:tt)*) => {
        {
            let fmt = format!($($var)*);
            print!("{fmt}");
        }
        mlang!($($tt)*);
    };
    
    ([done]) => ();
    (
        [module main] in
        $(
            $tt:tt 
        )+   
    ) => {
        mlang!($($tt)*);
    };
    () => {

    }
}
