#![feature(type_alias_impl_trait)]

fn main() {
    gcell::new!(core::mem::size_of::<ctor::Opaque>());
    //~^ error: failed to resolve: use of undeclared crate or module `foo`

    let (mut cell, mut token) = gcell::new!(10_u32);
    let (mut cell2, mut token2) = gcell::new!(10_u32);
    cell.get_mut(&mut token);
    cell2.get_mut(&mut token2);
    cell.get_ref(&token);
    cell2.get_ref(&token2);

    cell.get_mut(&mut token2);
    //~^ error: mismatched types
    //  expected mutable reference `&mut Bar<impl Trait>`
    //  found mutable reference: `&mut Bar<impl Trait>`
    cell.get_ref(&token2);
    //~^ error: mismatched types
    //  expected reference `&Bar<impl Trait>`
    //  found reference: `&Bar<impl Trait>`
    cell2.get_mut(&mut token);
    //~^ error: mismatched types
    //  expected mutable reference `&mut Bar<impl Trait>`
    //  found mutable reference: `&mut Bar<impl Trait>`
    cell2.get_ref(&token);
    //~^ error: mismatched types
    //  expected reference `&Bar<impl Trait>`
    //  found reference: `&Bar<impl Trait>`

    cell = cell2;
    //~^ error: mismatched types
    //  expected struct: `Foo<impl Trait>`
    //  found struct: `Foo<impl Trait>`
    cell2 = cell;
    //~^ error: mismatched types
    //  expected struct: `Foo<impl Trait>`
    //  found struct: `Foo<impl Trait>`
    token = token2;
    //~^ error: mismatched types
    //  expected struct: `Bar<impl Trait>`
    //  found struct: `Bar<impl Trait>`
    token2 = token;
    //~^ error: mismatched types
    //  expected struct: `Bar<impl Trait>`
    //  found struct: `Bar<impl Trait>`
}
