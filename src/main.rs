/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: blackrider <blackrider@student.42.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/09 14:13:14 by blackrider        #+#    #+#             */
/*   Updated: 2024/03/09 15:19:34 by blackrider       ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
	let		secret_num = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number");
	loop {
		let mut guess = String::new();
	    println!("Please, enter your number:");
		io::stdin().read_line(&mut guess)
			.expect("Could not read the line");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		match guess.cmp(&secret_num)
		{
			Ordering::Less => println!("Number is less"),
			Ordering::Greater => println!("Number is grate then"),
			Ordering::Equal => {
				println!("Number is equal!!!");
				break ;
			}
		}	
	}
}
