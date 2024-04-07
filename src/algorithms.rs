pub fn busca_sequencial(data: &Vec<i32>, element: i32) -> isize {
    for (index, &value) in data.iter().enumerate() {
        if value == element {
            return index as isize;
        }
    }
    return -1;
}

pub fn busca_sequencial_otimizada(data: &Vec<i32>, element: i32) -> isize {
    for (index, &value) in data.iter().enumerate() {
        if value > element {
            return -1;
        } else if value == element {
            return index as isize;
        }
    }
    return -1;
}

pub fn busca_binaria(data: &Vec<i32>, element: i32) -> isize {
    let (mut start, mut end) = (0, data.len() - 1);
    while start < end {
        let mid = start + (end - start) / 2;
        if data[mid] == element {
            return mid as isize;
        }
        if data[mid] < element {
            start = mid + 1;
        } else {
            end = mid; // Adjust end index when element is greater
        }
    }
    return -1;
}

pub fn busca_binaria_recursiva(data: &Vec<i32>, element: i32, start: usize, end: usize) -> isize {
    if start >= end {
        return -1;
    }
    let mid: usize = (start + end) / 2;
    if data[mid] == element {
        return mid as isize;
    } else if data[mid] > element {
        return busca_binaria_recursiva(data, element, start, mid);
    } else {
        return busca_binaria_recursiva(data, element, mid + 1, end);
    }
}