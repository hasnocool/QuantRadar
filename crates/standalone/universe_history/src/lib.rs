//! universe_history stub crate.
pub struct UniverseHistory { pub members: Vec<(String,u64)> }
impl UniverseHistory { pub fn new()->Self{Self{members:vec![]}} pub fn add(&mut self,symbol:String,ts:u64){self.members.push((symbol,ts));} pub fn at_time(&self,t:u64)->Vec<String>{self.members.iter().filter(|(_,ts)|*ts<=t).map(|(s,_)|s.clone()).collect()} }
