fn main() {
    
    let mut counter : u8 = 0;
    
    'out_loop: loop{
        
        println!("out: {}",counter);
        
        if counter == 10{
            break 'out_loop;
        }
        
        let mut inner_counter :u8 = 0;
        'inner_loop: loop{
            if inner_counter == counter + 1{
                break 'inner_loop;
            }

            println!("inner: {}",inner_counter);
            inner_counter+=1;
        }



        counter+=1;
    }


}
