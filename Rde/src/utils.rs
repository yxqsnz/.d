

/// catch a error
/// ```rs,no_run
/// catch!({
///    // some code
/// } then(err) => {
///   println!("a error: {}", err)
/// })
/// ```
#[macro_export]
macro_rules! catch {
    ($try_block:block then($err:ident) => $err_block:block) => {{
        #[allow(unused_mut)]
        let mut block = || -> Result<(),  Box<dyn std::error::Error>> { 
            $try_block
            Ok(())
        };
        match block() {
            Ok(_) => (),
            Err($err) => {
                $err_block
            }
        }
    }};
}

#[macro_export]
macro_rules! popup {
    ($display:ident, title = $title:expr, $($arg:tt)*) => {
        {
            let err = format!($($arg)*);
            $display.add_layer(
                cursive::views::Dialog::new()
                    .title($title)
                    .content(cursive::views::TextView::new(err))
                    .button("Ok", |s| { let _ = s.pop_layer(); }),
            );
        }
    };
    ($display:ident, $($arg:tt)*) => {
        {
            let err = format!($($arg)*);
            $display.add_layer(
                cursive::views::Dialog::new()
                    .title("Zyrmo Rde")
                    .content(cursive::views::TextView::new(err))
                    .button("Ok", |s| { let _ = s.pop_layer(); }),
            );
        }
    };
    
}