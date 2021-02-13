mod def;
mod player;
mod table;

pub(crate) use def::{ActionPolicy, Concept, DealtResult, Seat, TileDealingSpec};
pub(crate) use table::TableOld;

#[cfg(test)]
mod test {
    use crate::game::def::Action;
    use crate::game::{ActionPolicy, Concept, DealtResult, Seat, TableOld, TileDealingSpec};
    use std::rc::Rc;

    struct MockConcept;

    impl Concept for MockConcept {
        type Tile = char;
        type Meld = ();
    }

    struct MockTileDealingSpec;

    impl TileDealingSpec<MockConcept> for MockTileDealingSpec {
        fn deal(&self) -> DealtResult<MockConcept> {
            DealtResult::new(
                vec!['a', 'b'],
                vec!['c'],
                vec!['d'],
                [
                    (vec!['e'], Seat::East),
                    (vec!['f'], Seat::South),
                    (vec!['g'], Seat::West),
                    (vec!['h'], Seat::North),
                ],
            )
        }
    }

    struct MockActionPolicy;

    impl ActionPolicy<MockConcept> for MockActionPolicy {
        fn action_after_draw(&self, drawn_tile: char) -> Action<MockConcept> {
            Action::Discard(drawn_tile)
        }
    }

    #[test]
    fn points_should_not_change_after_exhaustive_drawn_with_no_ready_all() {
        let tile_dealing_spec = {
            let spec: Box<dyn TileDealingSpec<MockConcept>> = Box::new(MockTileDealingSpec {});
            Rc::new(spec)
        };

        let mut table = TableOld::new(tile_dealing_spec);

        let action_policy = {
            let policy: Box<dyn ActionPolicy<MockConcept>> = Box::new(MockActionPolicy {});
            Rc::new(policy)
        };

        let mock_user_seeds = [
            (action_policy.clone(), Seat::East),
            (action_policy.clone(), Seat::South),
            (action_policy.clone(), Seat::West),
            (action_policy.clone(), Seat::North),
        ];

        table.join_users(mock_user_seeds);
        table.start_game(1000);
        table.do_hand(); // イテレータで止めながらやりたい

        assert_eq!(table.player_at(Seat::East).point(), 1000);
        assert_eq!(table.player_at(Seat::South).point(), 1000);
        assert_eq!(table.player_at(Seat::West).point(), 1000);
        assert_eq!(table.player_at(Seat::North).point(), 1000);
    }
}
