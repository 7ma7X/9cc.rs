use crate::tokenizer::*;
use crate::util::*;

#[derive(Debug)]
pub struct Node {
  ty: Tk,
  lhs: Option<Box<Node>>,
  rhs: Option<Box<Node>>
}

impl Node {
  fn new_node(ty: Tk, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Node {
    Node { ty: ty, lhs: lhs, rhs: rhs }
  }

  fn new_node_num(val: i32) -> Node {
    Node { ty: Tk::Num(val), lhs: None, rhs: None }
  }

  /**
   * expr = rel ("==" rel | "!=" rel)*
   */
  pub fn expr(tokens: &Vec<Token>, pos: &mut usize, original: &String) -> Node {
    let mut node: Node = Node::rel(tokens, pos, original);

    loop {
      if consume(Tk::Equ, tokens, pos) {
        node = Node::new_node(
          Tk::Equ, 
          Some(Box::new(node)), 
          Some(Box::new(Node::rel(tokens, pos, original)))
        )
      } else if consume(Tk::Ne, tokens, pos) {
        node = Node::new_node(
          Tk::Ne, 
          Some(Box::new(node)), 
          Some(Box::new(Node::rel(tokens, pos, original)))
        )
      } else {
        return node;
      }
    }
  }

  /**
   * rel = add ("<" add | "<=" add)*
   */
  fn rel(tokens: &Vec<Token>, pos: &mut usize, original: &String) -> Node {
    let mut node: Node = Node::add(tokens, pos, original);

    loop {
      if consume(Tk::Lt, tokens, pos) {
        node = Node::new_node(
          Tk::Lt, 
          Some(Box::new(node)), 
          Some(Box::new(Node::add(tokens, pos, original)))
        )
      } else if consume(Tk::Le, tokens, pos) {
        node = Node::new_node(
          Tk::Le, 
          Some(Box::new(node)), 
          Some(Box::new(Node::add(tokens, pos, original)))
        )
      } else {
        return node;
      }
    }
  }

  /**
   * add = mul ("+" mul | "-" mul)*
   */
  fn add(tokens: &Vec<Token>, pos: &mut usize, original: &String) -> Node {
    let mut node: Node = Node::mul(tokens, pos, original);

    loop {
      if consume(Tk::Plus, tokens, pos) {
        node = Node::new_node(
          Tk::Plus, 
          Some(Box::new(node)), 
          Some(Box::new(Node::mul(tokens, pos, original)))
        )
      } else if consume(Tk::Minus, tokens, pos) {
        node = Node::new_node(
          Tk::Minus, 
          Some(Box::new(node)), 
          Some(Box::new(Node::mul(tokens, pos, original)))
        )
      } else {
        return node;
      }
    }
  }

  /**
   * mul = unary ("*" unary | "/" unary)*
   */
  fn mul(tokens: &Vec<Token>, pos: &mut usize, original: &String) -> Node {
    let mut node: Node = Node::unary(tokens, pos, original);

    loop {
      if consume(Tk::Multi, tokens, pos) {
        node = Node::new_node(
          Tk::Multi, 
          Some(Box::new(node)), 
          Some(Box::new(Node::unary(tokens, pos, original)))
        )
      } else if consume(Tk::Div, tokens, pos) {
        node = Node::new_node(
          Tk::Div, 
          Some(Box::new(node)), 
          Some(Box::new(Node::unary(tokens, pos, original)))
        ) 
      } else {
        return node;
      }
    }
  }

  /**
   * unary = ("+" | "-")? term
   */
  fn unary(tokens: &Vec<Token>, pos: &mut usize, original: &String) -> Node {
    if consume(Tk::Plus, tokens, pos) {
      return Node::term(tokens, pos, original);
    }
    if consume(Tk::Minus, tokens, pos) {
      return Node::new_node(Tk::Minus, 
        Some(Box::new(Node::new_node_num(0))), 
        Some(Box::new(Node::term(tokens, pos, original)))
      );
    }
    
    Node::term(tokens, pos, original)
  }

  /**
   * term = "(" expr ")" | num
   */
  fn term(tokens: &Vec<Token>, pos: &mut usize, original: &String) -> Node {
    if consume(Tk::LParen, tokens, pos) {
      let node = Node::expr(tokens, pos, original);
      if !(consume(Tk::RParen, tokens, pos)) {
        error_at(&tokens[*pos].input, original, "開きカッコに対応する閉じカッコがありません");
      }
      return node;
    }

    if let Tk::Num(n) = tokens[*pos].ty {
      *pos += 1;
      return Node::new_node_num(n);
    } else {
      error_at(&tokens[*pos].input, original, "数値でも開きカッコでもないトークンです");
      panic!("トークンエラー");
    }
  }

  /**
   * 抽象構文木からアセンブリを生成
   */
  pub fn gen(&self) {
    if let Tk::Num(n) = &self.ty {
      println!("    push {}", n);
      return;    
    } else {
      if let Some(ref lnode) = &self.lhs {
        lnode.gen();
      }
      if let Some(ref rnode) = &self.rhs {
        rnode.gen();
      }

      println!("    pop rdi");
      println!("    pop rax");

      match &self.ty {
        Tk::Plus => {
          println!("    add rax, rdi");
        }
        Tk::Minus => {
          println!("    sub rax, rdi");
        }
        Tk::Multi => {
          println!("    imul rdi");
        }
        Tk::Div => {
          println!("    cqo");
          println!("    idiv rdi");
        }
        rl @ Tk::Equ | rl @ Tk::Ne | rl @ Tk::Lt | rl @ Tk::Le => {
          println!("    cmp rax, rdi");
          match rl {
            Tk::Equ => println!("    sete al"),
            Tk::Ne  => println!("    setne al"),
            Tk::Lt  => println!("    setl al"),
            Tk::Le  => println!("    setle al"),
            _ => {}
          }
          println!("    movzb rax, al");          
        }
        _ => {}
      }

      println!("    push rax");
    }
  }
}

/**
 * 次のトークンが期待した型かどうかをチェックし, 
 * 期待した型の場合だけ入力を1トークン読み進めて真を返す関数
 */ 
fn consume(ty: Tk, tokens: &Vec<Token>, pos: &mut usize) -> bool {
  if tokens[*pos].ty != ty {
    return false;
  }
  *pos += 1;
  true
}