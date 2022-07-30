extern crate serde_derive;
use std::io;
use std::io::Write;

mod blockchain;


fn main() {
    let miner_addr = user_input("Input a miner address: ");
    let difficulty = user_input("Difficulty: ");
    let diff = difficulty.parse::<u32>().expect("We need an integer");
    
    println!("Generate genesis block");
    let mut chain = blockchain::Chain::new(miner_addr, diff);
    
    loop {
        let to_display = vec![
            "Menu", 
            " 1) new tx",
            " 2) Mine block",
            " 3) Change Diff.",
            " 4) Change reward",
            " 5) View block chain",
            " 0) Exit",
            ];

        for txt in &to_display {
            println!("{}", txt);
        }
        
        let choice = match user_input("Enter your choice: ").parse::<i32>() {
            Ok(c) => c,
            Err(_) => {
                println!("Please enter a valid choice! Should be an number");
                continue
            }
        };

        match choice {
            1 => {
                let sender = user_input("Sender: ");
                let reciever = user_input("Reciever: ");
                let amount = match user_input("Amount: ").parse::<f32>() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Invalid amount", );
                        continue;
                    }
                };
                chain.new_tx(sender, reciever, amount);
                println!("Send");
            },
            2 => {
                chain.gen_new_block();
            },
            3 => {
                let diff = match user_input("New difficulty: ").parse::<u32>() {
                    Ok(d) => d,
                    Err(_) => {
                        println!("Difficulty should be an int");
                        continue;
                    } 
                };
                chain.update_difficulty(diff);
                println!("Difficulty updated to {diff}");
            },
            4 => {
                let reward = match user_input("New reward: ").parse::<f32>() {
                    Ok(r) => r,
                    Err(_) => {
                        println!("Difficulty should be an int");
                        continue;
                    } 
                };
                chain.update_reward(reward);
                println!("Reward updated to {reward}");
            }
            5 => {
                println!("{:#?}", chain.chain);
            },
            0 => {
                let confirm = match user_input("Are you sure(Y/n)?").as_str() {
                    "y" => true,
                    "n"  => false,
                    _ => false
                };

                if confirm == true {
                    println!("Shut down...");
                    ::std::process::exit(0)
                }

            },
            _ => {
                println!("Please enter a valid choice");
            }
        };
        

    }
}


fn user_input(msg: &str) -> String {
    print!("{}", msg);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}