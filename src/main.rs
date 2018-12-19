pub mod dom;

fn main() {
  let tag = String::from("div");
  let mut attrs = dom::AttrMap::new();
  attrs.insert(String::from("class"), String::from(".box"));


  let one = dom::text(String::from("one"));
  let two = dom::text(String::from("two"));
  let three = dom::text(String::from("three"));

  let el = dom::elem(tag, attrs, vec![one, two, three]);

  el.print();
}
