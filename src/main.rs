pub mod dom;
pub mod html;

fn main() {
  let source = "
    <head></head>
    <body>
      <div>hello</div>
      <div>one</div>
      <div>two</div>
      <div>three</div>
    </body>
  "
  .to_string();

  let nodes = html::parse(source);

  nodes.print();
}
