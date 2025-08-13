// closure - like function that are not named

// use strd::thread;
// use::std::time::Duration;

// fn simulated_expensive_calculation(intensity:u32) -> u32{
//     println!("Calculating Slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// fn main(){
//     let _simulated_intensity = 10;
//     let _simulated_random_number = 7;

//     generate_workout(_simulated_intensity,_simulated_random_number);

// }

// fn generate_workout(intensity:u32, random_number:u32){
//     // let expensive_result = simulated_expensive_calculation(intensity);

//     let expensive_closure = |num| {
//         println!("Calculating Slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };

//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_result
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_result
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated");
//         } else {

//         }

//     }
// }


