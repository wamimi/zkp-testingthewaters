// SPDX-License-Identifier: GPL-3.0
// @author: konfushon(https://x.com/konfushon)

use rand::Rng;
use sha2::{Digest, Sha256};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

// CONSTANTS
const G: u64 = 5; // generator
const P: u64 = 65521; // largest 16-bit prime number. In a real-world scenario, we'd use a much larger prime(Mersenne prime number)

// This struct implements a zero knowledge proof (zkp) system for password verification
struct ZKPasswordVerifier {
    password_hash: [u8; 32],
    public_key: u64,
    r: u64,
    challenge: u64,
}

impl ZKPasswordVerifier {
    fn new(password: &str) -> Self {
        let password_hash = Sha256::digest(password.as_bytes());
        let password_num = u64::from_be_bytes(password_hash[0..8].try_into().unwrap());

        // Generate the public key: g^password mod P
        let public_key = mod_pow(G, password_num, P);
        ZKPasswordVerifier {
            password_hash: password_hash.into(),
            public_key,
            r: 0,
            challenge: 0,
        }
    }

    // Generate a random challenge for the proof
    fn generate_challenge(&mut self) -> u64 {
        let mut rng = rand::thread_rng();
        self.r = rng.gen_range(1000..10000);
        self.challenge = mod_pow(G, self.r, P);
        self.challenge
    }

    // Verify the response to the challenge
    fn verify(&self, response: u64) -> bool {

        // Calculate g^response mod P
        let expected = mod_pow(G, response, P);

        // Calculate (((g^r) mod P) * (public_key^challenge mod P)) mod P
        let verification =
            (mod_pow(G, self.r, P) * mod_pow(self.public_key, self.challenge, P)) % P;

        // If the prover knows the password, these values should be equal    
        expected == verification
    }
}

// A helper function for efficient modular exponentiation
fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exponent;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn handle_client(stream: TcpStream) {

    // Initialize the zero knowledge proof verifier with the password that you are to prove knowledge  of
    let mut verifier = ZKPasswordVerifier::new("ProofOfKnowledge"); // this is the password you are to prove knowledge of
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut lines = reader.lines();

    writeln!(writer, "\n\t\t\t**********Welcome to the zero knowledge proof challenge!**********\n\n\n").unwrap();
    writeln!(writer, 
        "\t\t\tYou have entered the realm of cryptographic challenges,\n\
         \t\t\twhere knowledge is power but revealing your secret will lead to your own peril!\n\
         \t\t\tIn this digital landscape, I am the guardian of a treasure known only to those who possess\n\
         \t\t\tthe secret password. But fear not! You shall not need to disclose this password to prove your worth.\n\
         \t\t\tInstead, you will engage in a test of wits and cunning — an ancient ritual known as zero knowledge proofs.\n\
         \t\t\tHere, you will convince me that you know the password without ever uttering it aloud.\n\
         \t\t\tTo begin, I will present you with a challenge.\n\
         \t\t\tYou must solve it using your knowledge of the password while keeping it hidden from prying eyes.\n\
         \t\t\tOnly then shall you earn the coveted flag that signifies your success!\n\
         \t\t\tAre you ready to embark on this quest?\n").unwrap();
    writeln!(writer, "PROVE YOU KNOW THE PASSWORD WITHOUT REVEALING IT.\n").unwrap();
    writeln!(writer, "\t\tPublic Key: {}", verifier.public_key).unwrap();
    writer.flush().unwrap();

    let mut attempts = 0;
    while attempts < 3 {
        let challenge = verifier.generate_challenge();
        writeln!(writer, "\t\tChallenge: {}\n\n", challenge).unwrap();
        write!(writer, "Your response: ").unwrap();
        writer.flush().unwrap();

        if let Some(Ok(response)) = lines.next() {
            if let Ok(user_response) = response.trim().parse::<u64>() {
                if verifier.verify(user_response) {

                    // If verified, create and send the flag
                    let flag = format!("flag{{zk_{}}}", hex::encode(&verifier.password_hash[..6]));
                    writeln!(writer, "Correct! Took you a while.\n. Flag: {}\n", flag).unwrap();
                    return;
                } else {
                    writeln!(writer, "Incorrect. Try again.\n\n").unwrap();
                    attempts += 1;
                }
            } else {
                writeln!(writer, "Your solution is definitely numbers. I don't know what that was you're giving me!\n\n").unwrap();
            }
        } else {
            println!("Client disconnected.");
            return;
        }
        writer.flush().unwrap();
    }
    writeln!(writer, "Too many incorrect attempts. Seems like this challenge is too smart for you. Goodbye!").unwrap();
    writer.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:5555").unwrap();
    println!("Connect to the challenge using: nc localhost 5555");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
