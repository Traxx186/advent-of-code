pub fn pop_multiple<T>(vec: &mut Vec<T>, amount: usize) -> Option<Vec<T>> {
    let mut out = Vec::new();
    for _ in 0..amount {
        let item = vec.pop();
        match item {
            Some(el) => out.push(el),
            None => return None
        }
    }

    Some(out)
}

pub fn push_multiple<T>(out: &mut Vec<T>, items: Vec<T>) {
    for item in items {
        out.push(item)
    }
}

