use super::super::simple::structs::*;
use super::super::simple::trait_defs::*;
use super::groups::*;
use super::param::*;
use cglue_macro::*;
use core::ffi::c_void;

#[cglue_trait]
pub trait AssociatedReturn {
    #[wrap_with(*const c_void)]
    #[return_wrap(|ret| Box::leak(Box::new(ret)) as *mut _ as *const c_void)]
    type ReturnType;

    fn ar_1(&self) -> Self::ReturnType;
}

impl AssociatedReturn for SA {
    type ReturnType = usize;

    fn ar_1(&self) -> usize {
        42
    }
}

#[cglue_trait]
pub trait ObjReturn {
    #[wrap_with_obj(TA)]
    type ReturnType: TA + 'static;

    fn or_1(&self) -> Self::ReturnType;
}

impl ObjReturn for SA {
    type ReturnType = SA;

    fn or_1(&self) -> SA {
        SA {}
    }
}

#[cglue_trait]
pub trait ObjUnboundedReturn {
    #[wrap_with_obj(TA)]
    type ReturnType: TA;

    fn our_1(&self) -> Self::ReturnType;
}

impl ObjUnboundedReturn for SA {
    type ReturnType = SB;

    fn our_1(&self) -> SB {
        SB {}
    }
}

#[cglue_trait]
pub trait GenericReturn<T: 'static> {
    #[wrap_with_obj(GenericTrait<T>)]
    type ReturnType: GenericTrait<T>;

    fn gr_1(&self) -> Self::ReturnType;
}

impl GenericReturn<usize> for SA {
    type ReturnType = SA;

    fn gr_1(&self) -> SA {
        SA {}
    }
}

// TODO: generic return where T gets automatically bounded by cglue_trait

#[cglue_trait]
pub trait GenericGroupReturn<T: 'static + Eq> {
    #[wrap_with_group(GenericGroup<T>)]
    type ReturnType: GenericTrait<T>;

    fn ggr_1(&self) -> Self::ReturnType;
}

impl GenericGroupReturn<usize> for SA {
    type ReturnType = SA;

    fn ggr_1(&self) -> SA {
        SA {}
    }
}

#[test]
fn use_assoc_return() {
    let sa = SA {};

    let obj = trait_obj!(sa as AssociatedReturn);

    let ret = obj.ar_1();

    println!("{:?}", ret);

    assert_eq!(unsafe { *(ret as *const usize) }, 42);
}

#[test]
fn use_obj_return() {
    let sa = SA {};

    let obj = trait_obj!(sa as ObjReturn);

    let ta = obj.or_1();

    assert_eq!(ta.ta_1(), 5);
}

#[test]
fn use_gen_return() {
    let sa = SA {};

    let obj = trait_obj!(sa as GenericReturn);

    let ta = obj.gr_1();

    assert_eq!(ta.gt_1(), 27);
}

#[test]
fn use_group_return() {
    let sa = SA {};

    let obj = trait_obj!(sa as GenericGroupReturn);

    let group = obj.ggr_1();

    let cast = cast!(group impl GenWithInlineClause).unwrap();

    assert!(cast.gwi_1(&cast.gt_1()));
    assert!(!cast.gwi_1(&(cast.gt_1() + 1)));
}