## Random Bugs:

- Postgres: Despite not running, postgres from homebrew still redirecting requests to the non-running verstion instead of the docker container. 
    - Solution:  `brew services stop postgresql@14`
- Crates: Remember you have to define it at the top of the crate, mianrs or lib.rd. Only then will teh ide se eit.