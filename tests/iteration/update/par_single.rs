use rayon::prelude::*;
use shipyard::prelude::*;

#[test]
fn filter() {
    let world = World::new();
    let (mut entities, mut u32s) = world.borrow::<(EntitiesMut, &mut u32)>();

    u32s.update_pack();
    entities.add_entity(&mut u32s, 0);
    entities.add_entity(&mut u32s, 1);
    entities.add_entity(&mut u32s, 2);
    entities.add_entity(&mut u32s, 3);
    entities.add_entity(&mut u32s, 4);
    entities.add_entity(&mut u32s, 5);
    u32s.clear_inserted();

    let im_vec;
    let m_vec;
    let mod_vec;

    let iter = u32s.par_iter();
    assert_eq!(iter.opt_len(), Some(6));
    im_vec = iter.filter(|&&x| x % 2 == 0).collect::<Vec<_>>();
    assert_eq!(im_vec, vec![&0, &2, &4]);
    drop(im_vec);

    m_vec = (&mut u32s)
        .par_iter()
        .filter(|&&mut x| x % 2 != 0)
        .collect::<Vec<_>>();
    assert_eq!(m_vec, vec![&mut 1, &mut 3, &mut 5]);
    mod_vec = u32s.modified().par_iter().collect::<Vec<_>>();

    assert_eq!(mod_vec, vec![&0, &1, &2, &3, &4, &5]);
}
