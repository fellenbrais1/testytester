// This is a child module in this other module file
// Make that the path to the function you want to call respects this nesting
pub mod name_helpers {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{} {}", first, last);
        // let full_name = first.to_owned() + " " + &last;
        full_name
    }
}

pub mod data_base {
    pub fn display_data() -> String {
        let array = ["Bob", "Bub", "Bab", "Bib", "Beb"];
        let string = String::from(array[2]);
        string
    }
}