/******* ITERATIVE VERSION ******/
/*pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mut array_1 = array;
    let len = array.len();
    let mut mid = len / 2;
    let mut offset = 0;

    if key > *array.get(len-1).unwrap(){
        return None;
    }
    if key < *array.get(0).unwrap(){
        return None;
    }

    if *array.get(mid).unwrap() == key {
        return Some(mid)
    }
    loop {
        if offset >= array.len() {
            return None;
        }
        if let Some(current) = array_1.get(mid) {
            if *current == key {
                return Some(mid + offset)
            } else if key > *current {
                if mid == 0 {return None;}
                let (_low, high) = array_1.split_at(mid);
                array_1 = high;
                offset += mid;
                mid = high.len() / 2;
                continue;
            } else if key < *current {
                let (low, _high) = array_1.split_at(mid);
                mid = low.len() / 2;
                array_1 = low;
                continue;
            }
        }
    }
}
*/

/* ITERATIVE VERSION WITH INDEX
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = array.len() as i32 - 1;

    loop{
        if start > end {
            return None;
        }
        let mid = (start+end)/2;
        let v = array [mid as usize];
        if v == key {
            return Some (mid as usize);
        } else if v > key {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
}
*/



/******* RECURSIVE VERSION ******/
/*
pub fn find_recurs(array: &[i32], key: i32) -> Option<usize> {
    let len = array.len();
    let mid = len / 2;

    if array.is_empty() {
        return None;
    }
    if key > *array.get(len-1).unwrap(){
        return None;
    }
    if key < *array.get(0).unwrap(){
        return None;
    }

    if *array.get(mid).unwrap() == key {
        return Some(mid);
    }
    else {
        let (low,high) = array.split_at(mid);
        if key > *array.get(mid).unwrap(){
            let ret = find(high,key);
            if ret != None {
                return Some (ret.unwrap() + mid);
            }
            else {
                return None;
            }
        }
        if key < *array.get(mid).unwrap(){
            return find(low,key);
        }
        None
    }
}
*/

