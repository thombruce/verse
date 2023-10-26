use bevy::prelude::*;

// TODO: This is actually just some arbitrary value we wanted to be able to change.
//       It was originally created for credits scrolling and ported from there,
//       and is now also being used in the ui/damage.rs system.
//       Both systems wanted to be able to change the "top" value of a UI
//       element, set as a Val::[Px, Percent, VMax, VMin, VH, VW](f32) which
//       it seems we can't otherwise perform calculations on.
//       So what this is, really, is a wrapper around some f32 intended to be used
//       in some other UI element.
//
//       We might therefore think about renaming it to something like Uif32 so that it
//       could be used more generally for Top, Bottom, Left, Right, Margin, Padding, etc.
//
//       But what if we wanted to change two or more such values? An entity can't have
//       more than one of a single component, can it?
//
//       No, but... we should create more of these and determine what's common about them,
//       see if we can implement some kind of inheritence, and also generalise the system(s)
//       that are responsible for converting these values into Val::* ones.
//
//       Though, I'm not certain how possible that is... Are these values always in a
//       Style component? And how do we determine what kind of Val to set?
//
//       Perhaps 'Top'/'Uif32Top'/* should have a secondary value indicating type?
//       Following this, the conversions could be handled in a single system here.
#[derive(Component)]
pub struct Top(pub f32);
