use chess::{Board, Game};
use juniper::EmptyMutation;
use juniper::EmptySubscription;
use juniper::FieldResult;
use juniper::RootNode;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "Game of chess")]
struct ChessGame {
    id: String,
    can_declare_draw: bool,
    board: ChessBoard,
}

#[derive(GraphQLObject)]
#[graphql(description = "Representation of a chessboard")]
struct ChessBoard {
    fen: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn game(id: String) -> FieldResult<ChessGame> {
        let game = Game::new();
        assert_eq!(game.current_position(), Board::default());
        Ok(ChessGame {
            id: id.to_owned(),
            can_declare_draw: game.can_declare_draw(),
            board: ChessBoard {
                fen: format!("{}", game.current_position()),
            },
        })
    }
}

/*

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}

*/

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
