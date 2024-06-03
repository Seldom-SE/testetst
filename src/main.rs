use bevy::{
    ecs::{
        query::QueryFilter,
        system::{SystemParam, SystemParamItem},
    },
    prelude::*,
};

fn check_system<Out, Func: Send + Sync + 'static, F0: SystemParam, F1: SystemParam>(_f: Func)
where
    for<'a> &'a mut Func:
        FnMut(F0, F1) -> Out + FnMut(SystemParamItem<F0>, SystemParamItem<F1>) -> Out,
    Out: 'static,
{
}

fn main() {
    check_system(align_to_screen::<(), &Transform>);
    check_system(align_to_screen_2);

    App::new()
        .add_systems(Update, align_to_screen::<(), &Transform>)
        .run();
}

pub(crate) trait SystemGet<'a: 'w + 's, 'w, 's, M: 'static>: 'a + Sized {
    type Param: SystemParam;
    type Filter: QueryFilter;

    fn get(entity: Entity, param: &'a Self::Param) -> Option<Self>;
}

impl<'a: 'w + 's, 'w, 's, T: Component> SystemGet<'a, 'w, 's, ()> for &'a T {
    type Param = Query<'w, 's, &'static T>;
    type Filter = With<T>;

    fn get(entity: Entity, param: &'a Self::Param) -> Option<Self> {
        Some(param.get(entity).unwrap())
    }
}

fn align_to_screen<M, T>(
    ts: Query<Entity, <T as SystemGet<'_, '_, '_, M>>::Filter>,
    param: <T as SystemGet<'_, '_, '_, M>>::Param,
) where
    for<'a: 'w + 's, 'w, 's> T: SystemGet<'a, 'w, 's, M>,
{
    for t in &ts {
        if T::get(t, &param).is_some() {
            info!("Wow, I got a `T`")
        }
    }
}

fn align_to_screen_2<'a, 'w, 's>(
    _ts: Query<Entity, With<Transform>>,
    _param: Query<'w, 's, &'static Transform>,
) {
}

// pub(crate) trait SystemGet<'a, M>: 'a + Sized {
//     type Param<'w, 's>: SystemParam
//     where
//         'w: 'a,
//         's: 'a;
//     type Filter: QueryFilter;
//
//     fn get<'w: 'a, 's: 'a>(entity: Entity, param: &'a Self::Param<'w, 's>) -> Option<Self>;
// }
//
// impl<'a, T: Component> SystemGet<'a, ()> for &'a T {
//     type Param<'w, 's> = Query<'w, 's, &'static T> where 'w: 'a, 's: 'a;
//     type Filter = With<T>;
//
//     fn get<'w: 'a, 's: 'a>(entity: Entity, param: &'a Self::Param<'w, 's>) -> Option<Self> {
//         Some(param.get(entity).unwrap())
//     }
// }

// pub(crate) trait SystemGet<'a, 'w, 's, M: 'static>: 'a + Sized {
//     type Param: SystemParam;
//     type Filter: QueryFilter;
//
//     fn get(entity: Entity, param: &'a Self::Param) -> Option<Self>;
// }
//
// impl<'a, 'w, 's, T: Component> SystemGet<'a, 'w, 's, ()> for &'a T {
//     type Param = Query<'w, 's, &'static T>;
//     type Filter = With<T>;
//
//     fn get(entity: Entity, param: &'a Self::Param) -> Option<Self> {
//         Some(param.get(entity).unwrap())
//     }
// }
