import socket  # Import the socket library for network connections.
import hashlib  # Import the hashlib library for cryptographic hashing.
import random  # Import the random library to generate random numbers.
import time  # Import the time library to manage delays.

# Constants used in the zero-knowledge proof protocol
G = 5  # Generator value, part of the public parameters in cryptographic protocols.
P = 65521  # Prime modulus, another part of the public parameters.
PASSWORD = "ProofOfKnowledge" #The password i am trying to prove knowledge of without revealing it

# Function to perform modular exponentiation efficiently
def mod_pow(base, exponent, modulus):
    result = 1
    base = base % modulus
    while exponent > 0:
        if exponent % 2 == 1:
            result = (result * base) % modulus
        exponent >>= 1 # divide by 2 or shifting to the right by 1 bit
        base = (base * base) % modulus
    return result

def solve_challenge(host, port):
    # Hash the password and convert the first 8 bytes to an integer for cryptographic use.
    password_hash = hashlib.sha256(PASSWORD.encode()).digest()
    password_num = int.from_bytes(password_hash[:8], 'big')


    # Here we are establishing  a TCP connection to the server.
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port)) # Connect to the server.
        print(f"Connected to {host}:{port}")

        # Receive initial message(this works perfectly when i run the script)
        initial_msg = s.recv(4096).decode()
        print("Initial message received:")
        print(initial_msg)

        # Extract public key(Also works fine)
        public_key_line = [line for line in initial_msg.split('\n') if 'Public Key:' in line][0]
        public_key = int(public_key_line.split(':')[1].strip())
        print(f"Extracted Public Key: {public_key}")

        attempts = 0
        while attempts < 3:
            #Now this is where everything goes wrong welpp!
            # Generate random r
            r = random.randint(1, P - 1)
            
            # Calculate y = G^r mod P
            y = mod_pow(G, r, P)

            # Receive challenge
            challenge_msg = s.recv(4096).decode()
            print("Challenge received:")
            print(challenge_msg)

            challenge_line = [line for line in challenge_msg.split('\n') if 'Challenge:' in line][0]
            challenge = int(challenge_line.split(':')[1].strip())
            print(f"Extracted Challenge: {challenge}")

            # Calculate response: (r + password_num * challenge) mod (P - 1) // someone help me!!
            # i learnt this challenge is using something called Schnorr Digital Signature. They say its a pretty easy concept but i am yet to wrap my head around it
            response = (r + password_num * challenge) % P #or is it response = (r + password_num * challenge) % (P-1)

            # Send response
            print(f"Sending response: {response}")
            s.sendall(f"{response}\n".encode())

            # Receive result
            result = s.recv(4096).decode()
            print("Server response:")
            print(result)

            if "Correct" in result:
                print("Challenge solved!")
                break
            else:
                print("Incorrect. Trying again...")
                attempts += 1

        if attempts >= 3:
            print("Max attempts reached. Exiting.")

if __name__ == "__main__":
    solve_challenge('localhost', 5555)