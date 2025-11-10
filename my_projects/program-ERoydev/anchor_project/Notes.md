
# Development Brainstorming
1. Probably i should implement Escrow functionalities when people use liquidity pool




# Issues 
So if your instruction is:

create mint

create ATA for that mint

Anchor can’t safely do step 2 inside the same instruction if you’re passing the ATA as an account, because at the time of instruction execution, the ATA doesn’t exist yet — and the runtime will fail with the kind of access violation you’ve been seeing.