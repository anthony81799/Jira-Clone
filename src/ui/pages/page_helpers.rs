use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    let len = text.len();

    match len.cmp(&width) {
        std::cmp::Ordering::Equal => text.to_owned(),
        std::cmp::Ordering::Less => {
            let left_over = width - len;
            let mut column_string = text.to_owned();

            for _ in 0..left_over {
                column_string.push(' ');
            }

            column_string
        }
        std::cmp::Ordering::Greater => {
            if width == 0 {
                return "".to_owned();
            } else if width == 1 {
                return ".".to_owned();
            } else if width == 2 {
                return "..".to_owned();
            } else if width == 3 {
                return "...".to_owned();
            }
            let result = text.truncate_ellipse(width - 3);
            result.to_string()
        }
    }
}
