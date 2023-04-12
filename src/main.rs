

fn main() {
    let mut int_bffr = CircularBuffer {start_idx: 0, end_idx: 6, max_size: 12, current_size: 7, buffer: vec![0, 1, 2, 3, 4, 5, 6]};
    
    int_bffr.push(7);
    int_bffr.push(8);
    int_bffr.push(9);
    // int_bffr.push(10);
    // int_bffr.push(11);
    // int_bffr.push(12);

    println!("Buffer : {}", int_bffr.buffer[6]);

    let access = int_bffr.binary_search(8);

    println!("\n First element: {}", int_bffr.get_first());
    println!("\n Last element: {}", int_bffr.get_last());

    match access {
        Ok(access)=> {
            println!("\n Searched element: {}", access);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

struct CircularBuffer<T: Clone + std::cmp::PartialEq + std::cmp::PartialOrd> {
    start_idx:usize,
    end_idx:usize,
    max_size:usize,
    current_size:usize,
    buffer: Vec<T>
    //buffer: T[]
}

impl<T: Clone + std::cmp::PartialEq + std::cmp::PartialOrd> CircularBuffer<T> {
    fn get_first(&self)-> T
    {
        self.buffer[self.start_idx].clone()
    }

    fn get_last(&self)-> T
    {
        self.buffer[self.end_idx].clone()
    }

    fn at(&self, circular_index:usize) -> Result<T, String>
    {
        if circular_index > self.max_size {
            Err(String::from("The buffer has no index that high"))
        }
        else
        {
            let access_index:usize = (self.start_idx + circular_index) % self.max_size;
            Ok(self.buffer[access_index].clone())
        }
    }

    fn push(&mut self, new_data:T)
    {
        let insert_data = new_data.clone(); 
        //add a new element of type T
        if self.max_size == 0
        {
            self.buffer.push(insert_data);
            self.current_size+=1;
        }
        else if self.current_size < self.max_size
        {
            self.buffer.push(insert_data);
            self.end_idx += 1;
            self.current_size+=1;
        }
        else //when end_idx is self.buffer.len
        {
            self.start_idx = (self.start_idx + 1) % self.max_size;
            self.end_idx = (self.end_idx + 1) % self.max_size;
            self.buffer[self.end_idx] = insert_data;
        }
    }

    // interface to recursive binary search function
    fn binary_search(&self, target:T) -> Result<T, String>
    {
        // TODO - optimize
        // when function called but buffer unsorted
        //  sort (insertion)

        let search_min:usize = self.start_idx;
        let search_max:usize = self.end_idx;
        let search_result:Result<T, String>;

        //find the closest match to target in the buffer
        if target >= self.buffer[self.end_idx] {            // if target is highest or greater, return highest
            search_result = self.at(self.end_idx);
        } 
        else if target <= self.buffer[self.start_idx]       // if target is lowest or lesser, return lowest
        { 
            search_result = self.at(self.start_idx);
        }
        else                                                //otherwise, binary search the buffer
        { 
            // TODO
            search_result = self.binary_search_recurse(target.clone(), search_min, search_max);
        }

        match search_result {
            Ok(result)=> {
                if result == target
                {
                    // return
                    Ok(result.clone())
                } else {
                    println!("Nope");
                    Ok(result.clone())
                }
            },
            Err(e) => {
                Err(String::from("Could not find target: ".to_owned() + &String::from(e)))
            }
        }
    }

    //TODO - document
    fn binary_search_recurse(&self, target:T, search_min:usize, search_max:usize) -> Result<T, String>
    {
        //err case: when begin later than end, panic
        if search_min > search_max
        {
            return Err(format!("Could not find target: {}", String::from("target")));
        }

        let search_idx:usize = (search_max + search_min) / 2;
        let search_val:T = self.at(search_idx).unwrap();

        //find the closest match to target in the buffer
        if search_val == target
        {
            Ok(search_val.clone())
        }
        else if target > search_val
        { 
            // search the upper half of the search range
            self.binary_search_recurse(target.clone(), search_idx + 1, search_max)
        } 
        else // if target < search_val
        {
            // search the lower half of the search range
            self.binary_search_recurse(target.clone(), search_min, search_idx - 1)
        }
    }
}
