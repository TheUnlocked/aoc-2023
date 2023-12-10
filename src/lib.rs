pub trait AocResult {
    fn from_aoc(&self) -> Option<String>;
}

impl AocResult for String { fn from_aoc(&self) -> Option<String> { Some(self.into()) } }
impl AocResult for &str { fn from_aoc(&self) -> Option<String> { Some(self.to_string()) } }
impl AocResult for str { fn from_aoc(&self) -> Option<String> { Some(self.to_string()) } }
impl AocResult for i32 { fn from_aoc(&self) -> Option<String> { Some(ToString::to_string(self)) } }
impl AocResult for u32 { fn from_aoc(&self) -> Option<String> { Some(ToString::to_string(self)) } }
impl AocResult for () { fn from_aoc(&self) -> Option<String> { None } }

#[macro_export]
macro_rules! aoc {
    {
        use $inputPath:expr;
        $($tail:tt)*
    } => {
        use $crate::AocResult;

        fluid_let::fluid_let!(static DEBUG: bool = false);

        fn main() {
            $crate::run_top_level_stmts!($($tail)*);

            let path = $inputPath;
            println!("Reading {}...", path);
            let input = std::fs::read_to_string(std::path::Path::new(path));
            
            match input {
                Err(err) => {
                    println!("Failed to read {}: {}", path, err);
                }
                Ok(input) => {
                    let input = input.as_str();
                    $crate::run_parts!(input, $($tail)*);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! run_top_level_stmts {
    () => {};
    ($body:expr; $($tail:tt)*) => {
        $body;
        $crate::run_top_level_stmts!($($tail)*)
    };
    (
        fn $partn:ident($param:ident) $body:block
        $($tail:tt)* 
    ) => {
        fn $partn($param: &str) -> impl AocResult + $body
        $crate::run_top_level_stmts!($($tail)*);
    };
}

#[macro_export]
macro_rules! run_parts {
    ($input:expr,) => {};
    ($input:expr, $_:expr; $($tail:tt)*) => { $crate::run_parts!($input, $($tail)*) };
    (
        $input:expr,
        fn $partn:ident($_1:ident) $_2:block
        $($tail:tt)*
    ) => {

        let result = $partn($input).from_aoc();
        match result {
            None => {
                println!("[{}] Not implemented.", stringify!($partn));
            }
            Some(result) => {
                println!("[{}] SOLUTION: {}", stringify!($partn), result);
            }
        }

        $crate::run_parts!($input, $($tail)*)
    };
}

#[macro_export]
macro_rules! IF {
    ($fluid:expr, $then:block) => {
        $fluid.get(|current| {
            if let Some(true) = current $then
        })
    };
}

#[macro_export]
macro_rules! example {
    ($part:ident( $input:tt ) == $expected:expr) => {
        let input = indoc::indoc! { $input };
        let expected = $expected.to_string();

        let actual = DEBUG.set(true, || {
            $part(input.into()).from_aoc()
        });
        
        match actual {
            None => println!("[{}] EX: Not implemented, Skipping.", stringify!($part)),
            Some(actual) => if actual == expected {
                println!("[{}] EX: Passed.", stringify!($part));
            }
            else {
                println!("[{}] EX: Expected {} but got {}.", stringify!($part), expected, actual);
            }
        }
    };
}

