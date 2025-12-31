pub fn balance(parentheses: Vec<char>) -> bool {
    let mut open = 0;

    for p in parentheses {
        match p {
            '(' => open += 1,
            ')' => {
                open -= 1;
                if open < 0 {
                    return false;
                }
            }
            _ => (),
        }
    }

    open == 0
}
