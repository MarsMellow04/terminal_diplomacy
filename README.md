Connection Manager, calls this when connection and messaged are parsed.

The connection Manager has services, {Auth, Game, Move,}

The all have Reposoritire swhcih can talk to the DB (GameRepo, UserRepo, MovesRepo)

Then there is the Game Reigsutry this Maps the UUID of a game to the GameHandler, this holds a GameInstance 

The Game instance will have a listener for when Moves are added to the Game Instance.

There is a Statemanchine for teh user to handle the creation of orders.

There is also an order collector to to verify orders and collect them

There is session store that stores teh mutliple different sessions.