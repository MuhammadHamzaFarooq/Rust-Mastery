fn main() {
    //declear and initiation for subrarray like matrix 
    let arr = [[1,2,3],[4,5,6],[7,8,9]];
    let val = arr[0];//acess value with index no
    let val1 = arr[1];//acess value with index no
    let val2 = arr[2];//acess value with index no
    
  // declear and initiation mutlitple variable with zero
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    // declear and initiation empty vector
    let mut v: Vec<i32> = Vec::new();
   
  //while loop 1
    while(a <=2){
      let mut j :i32 = val[a];
      v.push(j);
      a +=1;
    }
    //while loop 2
    while(b <=2){
      let mut k :i32 = val1[b];
      v.push(k);
      b +=1;
      
    }
    //while loop 3
    while(c <=2){
      let mut l :i32 = val2[c];
      v.push(l);
      c +=1;
    }

    //printing the whole vector 
    println!("{:?}",v);
}
  