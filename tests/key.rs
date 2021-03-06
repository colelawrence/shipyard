use shipyard::prelude::*;

#[test]
fn key_equality() {
    let world = World::default();

    //create 3 entities
    let (e0, e1, e2) =
        world.run::<(EntitiesMut, &mut usize), _, _>(|(mut entities, mut usizes)| {
            (
                entities.add_entity(&mut usizes, 0),
                entities.add_entity(&mut usizes, 1),
                entities.add_entity(&mut usizes, 2),
            )
        });

    //add a component to e1
    world.run::<(EntitiesMut, &mut u32), _, _>(|(ref mut entities, ref mut u32s)| {
        entities.add_component(u32s, 42, e1);
    });

    //confirm that the entity keys have not changed for usizes storage
    world.run::<&usize, _, _>(|usizes| {
        //sanity check
        assert_eq!((&usizes).iter().with_id().count(), 3);

        let keys: Vec<EntityId> =
            (&usizes)
                .iter()
                .with_id()
                .map(|(entity, _)| entity)
                .fold(Vec::new(), |mut vec, x| {
                    vec.push(x);
                    vec
                });

        assert_eq!(keys, vec![e0, e1, e2]);
    });

    //confirm that the entity id for (usize) is the same as (usize, u32)
    //in other words that the entity itself did not somehow change from adding a component
    world.run::<(&usize, &u32), _, _>(|(usizes, u32s)| {
        //sanity check
        assert_eq!((&usizes, &u32s).iter().with_id().count(), 1);

        let (entity, (_, _)) = (&usizes, &u32s).iter().with_id().find(|_| true).unwrap();
        assert_eq!(entity, e1);
    });
}
