

fn main() {
    let mut int_bffr = CircularBuffer {start_idx: 0, end_idx: 6, max_size: 12, current_size: 7, buffer: vec![0, 1, 2, 3, 4, 5, 6]};
    
    int_bffr.push(7);
    int_bffr.push(8);
    int_bffr.push(9);
    // int_bffr.push(10);
    // int_bffr.push(11);
    // int_bffr.push(12);

    println!("Buffer : {}", int_bffr.buffer[6]);

    let access = int_bffr.binarySearch(8);
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
    fn binarySearch(&self, target:T) -> Result<T, String>
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
            search_result = self.binarySearchRecurse(target.clone(), search_min, search_max);
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
    //TODO - implement
    fn binarySearchRecurse(&self, target:T, search_min:usize, search_max:usize) -> Result<T, String>
    {
        let search_idx:usize = ((search_max - search_min)/2) + 1;
        let mut search_result:Result<T, String> = self.at(search_idx);
        let search_val = search_result.clone().unwrap();

        //find the closest match to target in the buffer
        if search_val == target || search_idx == search_min
        {
            //TODO - remove unnecessary
            search_result = self.at(search_idx);
        }
        else if target == self.at(search_max).unwrap()              // if target is highest, return highest
        {            
            search_result = self.at(self.end_idx);                  // will result in search biasing upward
        } 
        else if target == self.at(search_min).unwrap()              // if target is lowest, return lowest
        {   
            search_result = self.at(self.start_idx);
        }
        else if search_val < target               // when target is greater than search_result, search between search_idx and end_idx 
        { 
            // search the upper half of the search range
            search_result = self.binarySearchRecurse(target.clone(), search_idx, search_max);
        } 
        else if target < search_val
        {
            // search the upper half of the search range
            search_result = self.binarySearchRecurse(target.clone(), search_min, search_idx);
        }

        match search_result {
            Ok(result) => {
                println!("Min : {}", search_min);
                println!("Idx : {}", search_idx);
                println!("Max : {}", search_max);
                Ok(result) 
            },
            Err(e) => {
                return Err(String::from("Could not find target: ".to_owned() + &String::from(e)));
            }
        }
    }
}
