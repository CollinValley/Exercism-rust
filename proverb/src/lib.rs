pub fn build_proverb(list: Vec<&str>) -> String {
    match list.len() {
        0 => {
            String::new()
        }
        1 => {
            format!("And all for the want of a {}.", list[0])
        }
        _ => {
            let mut output = String::new();
            for (i,thing) in list.iter().enumerate() {
                match i {
                    0 => {}
                    _ => {
                        output.push_str(&format!("For want of a {} the {} was lost.\n", list[i-1], thing));
                    }
                }
            }
            output.push_str(&format!("And all for the want of a {}.", list[0]));
            output
        }
    }
}
