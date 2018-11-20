use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
  //We create the HashSet that will contain all our final tuplets (x, y, z)

  let mut set = HashSet::new();

    //Due to the condition a<b<c, it's not usefull to iterate through 1 to sum, 
    //so we iterate through 1 to sum / 2.
    for a in 1u32..sum/2 {

      //Same reason, we can assume that b can only be greater than a, and lesser than sum minus a.
      for b in (a+1)..(sum-a) {

        //Having compute a and b, we just compute c by doing a substraction, it's more effective than implementing a third for loop.
        let mut c = sum-a-b;

        //We check the condition that our new found triplet (a,b,c) corresponding to a+b+c = sum,
        //also match the condition that it is a pythagorean tuplets.
        if a.pow(2)+b.pow(2) == c.pow(2){

          //We just add the pythagorean tuplets to the above created HashSet, name set.
          set.insert([a, b, c]);
        }
      }
    }
  return set
} 
