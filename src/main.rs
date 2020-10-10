/*
Simulation Phantom Traffic Jam

To do
- Construct system which is a product space of lattice and 1D circuit
- Construct self propelled cars with its own speed limit (determined by global speed limit)
- Construct a closed linked list of cars produce insert, delete function for any position
- Add interaction between cars - interaction to backward affects longer than forward interaction
- Lane change of cars
- probabilistic decision of lane change
- Record the trajectory of cars, mean speed computation
- simulate phantom traffic jam according to density of cars and lane change probability
- Is it universal for dimensionality of lattice that the phantom traffic jam caused by lane change?
- Verify the prisoner's dilemma in the traffic - if almost of cars don't change lane without reason, then frequent lane change is better strategy for individual to maximize its mean speed
- Evolution scheme of cars to maximize their mean speed by change their lane change probability
- Is it possible that the mean speed increases in spite of the general speed limit decreases, by optimizing their lane change strategy

*/



pub mod system;

fn main() {
    println!("Hello, world!");
}
