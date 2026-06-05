# Domain Model and Architecture

## Bounded Contexts

### Tournament

The tournament context handles everything that is directly related to a tournament. It includes use cases like tournament creation, player registration, and filtered overview. It also includes reactions to table events such as ranking players, table balance, and blind control.

#### Domain Model

* Tournament
* TournamentSpecification
* TournamentPlayer
* TournamentTable


### Table

The table context handles everything that is directly related to a poker table. It includes uses cases like act on table, sit out, and table observation. It also includes reactions to tournament events such as discontinuation of a tables and reseating.

#### Domain Model

* Table
* TableSpecification
* TablePlayer
