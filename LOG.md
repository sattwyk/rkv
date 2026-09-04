# Task 1

To setup the cli to read the arguments and run it on two different modes

it needs two flags
- primary / p
- replica / r

rkv --primary
rkv --replica
rkv -p
rkv -r

if no flag received it will default to primary and MAX 1 argument


- i need a enum for Mode with primary and replica
- i need to use std::env to collect the arguments 

# Task 2

Create single threaded tcp server based on the mode they're in

- Primary - port 7000
- Replica - port 7001

both can parse text bytes coming to their stream



