use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool{
    let magazine_map = 
    magazine
    .iter()
    .fold(HashMap::new(), |mut words, str| {
        *(words.entry(*str).or_insert(0)) += 1;
        words    
    });

    let note_map = 
    note
    .iter()
    .fold(HashMap::new(), |mut words, str| {
        *(words.entry(*str).or_insert(0)) += 1;
        words    
    });

    note_map
    .iter()
    .all(&|(w, count)| magazine_map.get(w).unwrap_or(&0) >= count)
}

pub fn can_construct_note2(magazine: &[&str], note: &[&str]) -> bool {
    if note.len() > magazine.len() {return false};
    
    let mut note_map = HashMap::new();
    for word in note {
        let count = note_map.entry(word).or_insert(0);
        *count += 1;
    }
    let mut magazine_map = HashMap::new();
    for word in magazine {
        let count = magazine_map.entry(word).or_insert(0);
        *count += 1;
    }
    for word in note{
        match magazine_map.get(word){
            Some(res) => {
                if note_map[word] > *res {return false;}
            },
            None => return false
        }
    }
    return true;
}

fn test_func(){
    let temp = vec!["a", "b", "C", "D", "e"];
    let mut map = HashMap::new();
    temp.iter().enumerate().for_each(|(index, val)| {map.insert(val, index);});
    let res = map.entry(&"a").or_insert(5);
    *res += 10;
    // do dòng trên trả về &mut con trỏ tại giá trị đó, nên mình tăng giá trị nó cũng tăng giá trị ngay trong map luôn
    println!("{:?}", map);
}
// fold tương tự như reduce nhưng fold mình có thể init giá trị khởi tạo
// hàm entry dùng cho kiểu Hashmap => truyền vào key và trả về Entry
// => nếu key tồn tại trả về Entry(OccupiedEntry {key: .., value: ..})
// => nếu key không tồn tại trả về Entry(VacantEntry(key))
// hàm or_insert dùng kèm với entry thì nó sẽ kiểm tra nếu tồn tại key thì thôi, còn key chưa có thì nó chèn giá trị đi kèm với key đó

// Nguyên dòng này words.entry(*str).or_insert(0) => return &value => deref thì nhận được value và cộng nó thêm 1
// trả về words là kiểu hashmap