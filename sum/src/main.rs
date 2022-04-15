
fn sum( list: &[u32] )  -> Option<u32> {
    let mut total:u32 = 0;
    for x in list {
        let res = total.checked_add( *x );
        match res {
            Some(v) => total = v,
            None => return None,
        }
        
    }
    return Some(total)
}



fn main() {
    let para = &[1,2,3];
    let para_outbond = &[u32::MAX, 1 , 2];
    let res1 = sum(para);
    let res2 = sum(para_outbond);

    println!("sum of {:?} is: {:?}", para, res1);
    println!("sum of {:?} is: {:?}", para_outbond, res2);
}
