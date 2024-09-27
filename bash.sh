#!/bin/bash

PASSWORD="ProofOfKnowledge"

# Generate the SHA256 hash of the password
PASSWORD_HASH=$(echo -n "$PASSWORD" | sha256sum | awk '{print $1}')

# Take the first 8 bytes (16 characters) of the hash and convert to decimal
PASSWORD_NUM=$(echo "ibase=16; ${PASSWORD_HASH:0:16}" | bc)

# Constants
G=5
P=65521

# Generate public key (G^PASSWORD_NUM mod P)
PUBLIC_KEY=$(echo "($G^$PASSWORD_NUM) % $P" | bc)

# Public key
echo "Public Key: $PUBLIC_KEY"

# Connect to the server
HOST="localhost"
PORT=5555

# Create a TCP connection to the server
exec 3<>/dev/tcp/$HOST/$PORT

# Read the initial message from the server
initial_msg=$(cat <&3)
echo "Initial message from server:"
echo "$initial_msg"

# Send the public key to the server
echo "$PUBLIC_KEY" >&3
echo "Sent public key: $PUBLIC_KEY"

# Loop for the zero-knowledge proof protocol
for attempt in {1..3}; do
    echo "Attempt $attempt/3"

    # Generate random r
    r=$(( RANDOM % (P - 1) + 1 ))

    # Calculate y = G^r mod P
    y=$(echo "($G^$r) % $P" | bc)

    # Send y to the server
    echo "$y" >&3
    echo "Sent y: $y"

    # Read the challenge from the server
    challenge_msg=$(cat <&3)
    echo "Received challenge from server:"
    echo "$challenge_msg"

    # Extract the challenge
    challenge=$(echo "$challenge_msg" | grep "Challenge:" | cut -d ':' -f2 | tr -d ' ')
    echo "Extracted Challenge: $challenge"

    # Calculate the response
    response=$(( (r + PASSWORD_NUM * challenge) % (P - 1) ))

    # Send the response to the server
    echo "$response" >&3
    echo "Sent response: $response"

    # Read the result from the server
    result=$(cat <&3)
    echo "Server response:"
    echo "$result"

    if [[ "$result" == *"Correct"* ]]; then
        echo "Challenge solved!"
        break
    elif [[ "$result" == *"Incorrect"* ]]; then
        echo "Incorrect response. Trying again..."
    fi
done

# Close the connection
exec 3>&-